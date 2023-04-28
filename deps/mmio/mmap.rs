use crate::utils::Alignable;

use super::file::calculate_section_size;
use ntapi::{
    ntmmapi::{NtCreateSection, NtExtendSection, NtMapViewOfSection, NtUnmapViewOfSection},
    ntobapi::NtClose,
};
use std::{
    io,
    os::{
        raw::c_void,
        windows::prelude::{AsRawHandle, RawHandle},
    },
};
use winapi::um::{processthreadsapi::GetCurrentProcess, winnt::LARGE_INTEGER};
use windows::Win32::System::Memory::FlushViewOfFile;

const VIEW_SIZE_INCREMENT: usize = 1 << 37; // 128 gb

pub trait AsRawDescriptor {
    fn as_raw_descriptor(&self) -> RawHandle;
}

impl AsRawDescriptor for RawHandle {
    fn as_raw_descriptor(&self) -> RawHandle {
        *self
    }
}

impl<'a, T> AsRawDescriptor for &'a T
where
    T: AsRawHandle,
{
    fn as_raw_descriptor(&self) -> RawHandle {
        self.as_raw_handle()
    }
}

trait AsNTSTATUS {
    fn as_ntstatus(&self) -> windows::Win32::Foundation::NTSTATUS;
}

impl AsNTSTATUS for winapi::shared::ntdef::NTSTATUS {
    fn as_ntstatus(&self) -> windows::Win32::Foundation::NTSTATUS {
        windows::Win32::Foundation::NTSTATUS(*self)
    }
}

unsafe fn create_large_integer(value: i64) -> LARGE_INTEGER {
    let mut ret = LARGE_INTEGER::default();
    *ret.QuadPart_mut() = value;
    ret
}

struct MapPermissions {
    writable: bool,
    desired_access: u32,
    page_protection: u32,
    allocation_type: u32,
}

const SECTION_MAP_READ: u32 = ntapi::winapi::um::winnt::SECTION_MAP_READ;
const SECTION_MAP_WRITE: u32 = ntapi::winapi::um::winnt::SECTION_MAP_WRITE;
const SECTION_EXTEND_SIZE: u32 = ntapi::winapi::um::winnt::SECTION_EXTEND_SIZE;
const MEM_RESERVE: u32 = ntapi::winapi::um::winnt::MEM_RESERVE;
const PAGE_READWRITE: u32 = ntapi::winapi::um::winnt::PAGE_READWRITE;
const PAGE_READONLY: u32 = ntapi::winapi::um::winnt::PAGE_READONLY;
const SEC_COMMIT: u32 = ntapi::winapi::um::winnt::SEC_COMMIT;
const VIEW_UNMAP: u32 = ntapi::ntmmapi::ViewUnmap;

impl MapPermissions {
    fn new(writable: bool) -> Self {
        let mut desired_access = SECTION_MAP_READ;
        let mut allocation_type = 0;
        let page_protection = match writable {
            true => {
                desired_access |= SECTION_MAP_WRITE | SECTION_EXTEND_SIZE;
                allocation_type |= MEM_RESERVE;
                PAGE_READWRITE
            }
            false => PAGE_READONLY,
        };
        Self {
            writable,
            desired_access,
            page_protection,
            allocation_type,
        }
    }
}

pub struct Section {
    handle: RawHandle,
    permissions: MapPermissions,
    section_size: usize,
    ptr: Option<*mut c_void>,
}

unsafe impl Sync for Section {}
unsafe impl Send for Section {}

impl Drop for Section {
    fn drop(&mut self) {
        unsafe {
            _ = self.unmap();

            NtClose(self.handle);
        }
    }
}

impl Section {
    pub unsafe fn new<T: AsRawDescriptor>(file: T, writable: bool) -> io::Result<Self> {
        let file_handle = file.as_raw_descriptor();
        let section_size = calculate_section_size(file_handle)?;

        let permissions = MapPermissions::new(writable);

        let mut section_handle = std::ptr::null_mut();

        NtCreateSection(
            &mut section_handle,
            permissions.desired_access,
            std::ptr::null_mut(),
            &mut create_large_integer(section_size as i64),
            permissions.page_protection,
            SEC_COMMIT,
            file_handle,
        )
        .as_ntstatus()
        .ok()?;

        Ok(Self {
            handle: section_handle,
            permissions,
            section_size,
            ptr: None,
        })
    }

    pub unsafe fn map(&mut self) -> io::Result<()> {
        if self.ptr.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Mapping already exists",
            ));
        }

        self.map_unchecked()
    }

    unsafe fn map_unchecked(&mut self) -> io::Result<()> {
        let mut base_address = std::ptr::null_mut();

        NtMapViewOfSection(
            self.handle,
            GetCurrentProcess(),
            &mut base_address,
            0,
            0,
            std::ptr::null_mut(),
            &mut self.view_size(),
            VIEW_UNMAP,
            self.permissions.allocation_type,
            self.permissions.page_protection,
        )
        .as_ntstatus()
        .ok()?;
        self.ptr = Some(base_address);
        Ok(())
    }

    pub unsafe fn unmap(&mut self) -> io::Result<()> {
        if let Some(ptr) = self.ptr.take() {
            NtUnmapViewOfSection(GetCurrentProcess(), ptr)
                .as_ntstatus()
                .ok()?;
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Mapping does not exist",
        ))
    }

    pub unsafe fn reserve(&mut self, size: usize) -> io::Result<()> {
        if let Some(ptr) = self.ptr {
            if size > self.section_size {
                NtExtendSection(self.handle, &mut create_large_integer(size as i64))
                    .as_ntstatus()
                    .ok()?;

                let old_view_size = self.view_size();
                self.section_size = size;

                if self.view_size() > old_view_size {
                    // Remap the new section
                    self.map_unchecked()?;

                    // If remapping succeeds, unmap the old section
                    NtUnmapViewOfSection(GetCurrentProcess(), ptr)
                        .as_ntstatus()
                        .ok()?;
                }
            }
        }

        Ok(())
    }

    fn view_size(&self) -> usize {
        if self.permissions.writable {
            self.section_size.align_to(VIEW_SIZE_INCREMENT)
        } else {
            self.section_size
        }
    }

    pub fn make_read_only(&mut self) -> Option<()> {
        // Windows doesn't allow changing protections for mapped sections.
        // VirtualProtect will give an access denied, as well.
        // The only way to get around this is by closing and reopening the section,
        // but that defeats the purpose of make_read_only in making an already
        // existing section read-only.
        if !self.permissions.writable {
            return None;
        }

        self.permissions = MapPermissions::new(false);

        Some(())
    }

    // Flushes part of file asynchronously.
    // Call `File::datasync()` afterwards to ensure flushes.
    pub fn flush(&self, offset: usize, len: usize) -> io::Result<()> {
        if len == 0 {
            return Ok(());
        }
        if let Some(ptr) = self.ptr {
            unsafe { FlushViewOfFile(ptr.add(offset), len) }.ok()?;
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Mapping does not exist",
        ))
    }

    #[inline]
    pub fn ptr(&self) -> Option<*const u8> {
        if let Some(ptr) = self.ptr {
            return Some(ptr as *const u8);
        }
        None
    }

    #[inline]
    pub fn mut_ptr(&mut self) -> Option<*mut u8> {
        if let Some(ptr) = self.ptr {
            return Some(ptr as *mut u8);
        }
        None
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.section_size
    }
}
