use std::{
    io,
    ops::{Deref, DerefMut},
    slice,
};

use super::{
    mmap::{AsRawDescriptor, Section},
    MappedFile,
};

pub struct MappedFileMut {
    section: Section,
}

impl MappedFileMut {
    /// Create a new writable mapped file.
    /// # Safety
    /// Calls undocumented ntdll APIs.
    pub unsafe fn new<T: AsRawDescriptor>(file: T) -> io::Result<Self> {
        let mut section = Section::new(file, true)?;
        section.map()?;

        Ok(Self { section })
    }

    pub fn into_read_only(mut self) -> MappedFile {
        // writable is always true, this unwrap will never fail
        self.section.make_read_only().unwrap();
        MappedFile {
            section: self.section,
        }
    }

    /// Extends & remaps section to ensure the its length is at least `size`.
    /// # Safety
    /// Calls undocumented ntdll APIs; may invalidate ptr when other code is still using it.
    pub unsafe fn reserve(&mut self, size: usize) -> io::Result<()> {
        self.section.reserve(size)
    }

    pub fn flush(&self) -> io::Result<()> {
        self.section.flush(0, self.len())
    }

    pub fn flush_range(&self, offset: usize, len: usize) -> io::Result<()> {
        self.section.flush(offset, len)
    }
}

impl Deref for MappedFileMut {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.section.ptr().unwrap(), self.section.len()) }
    }
}

impl DerefMut for MappedFileMut {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.section.mut_ptr().unwrap(), self.section.len()) }
    }
}

impl AsRef<[u8]> for MappedFileMut {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl AsMut<[u8]> for MappedFileMut {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}
