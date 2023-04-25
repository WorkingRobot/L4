use std::{
    fs::File,
    io,
    mem::ManuallyDrop,
    ops::Deref,
    os::windows::io::{FromRawHandle, RawHandle},
    slice,
};

use super::mmap::Section;

pub struct MappedFile {
    pub(super) section: Section,
}

impl MappedFile {
    pub unsafe fn new(handle: RawHandle) -> io::Result<Self> {
        let mut section = Section::new(handle, calculate_section_size(handle)?, false)?;
        section.map()?;

        Ok(Self { section })
    }
}

impl Deref for MappedFile {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.section.ptr().unwrap(), self.section.len()) }
    }
}

impl AsRef<[u8]> for MappedFile {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

pub(super) fn calculate_section_size(handle: RawHandle) -> io::Result<usize> {
    unsafe {
        let file = ManuallyDrop::new(File::from_raw_handle(handle));
        Ok(file.metadata()?.len() as usize)
    }
}
