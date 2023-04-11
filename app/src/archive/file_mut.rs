use super::file::ArchiveTrait;
use super::structs::*;
use super::{file::imp::ArchiveImpl, stream_mut::StreamMut};
use fs2::FileExt;
use memmap2::{MmapMut, MmapOptions};
use std::{
    fs::{File, OpenOptions},
    ops::Range,
    path::Path,
};

mod imp {
    pub trait ArchiveMutImpl {
        fn mapping_mut(&mut self) -> &mut [u8];

        #[inline]
        fn read_type_mut<T>(&mut self, offset: usize) -> Option<&mut T> {
            let len = std::mem::size_of::<T>();

            let slice = self.mapping_mut().get_mut(offset..offset + len)?;
            let data = unsafe { &mut *slice.as_mut_ptr().cast::<T>() };
            Some(data)
        }
    }
}

pub trait ArchiveMutTrait: imp::ArchiveMutImpl + ArchiveTrait {
    #[inline]
    fn header_mut(&mut self) -> &mut Header {
        self.read_type_mut::<Header>(0).unwrap()
    }

    #[inline]
    fn freelist_mut(&mut self) -> &mut Freelist {
        self.read_type_mut::<Freelist>(self.header().freelist_offset())
            .unwrap()
    }

    #[inline]
    fn stream_header_mut(&mut self, stream_idx: u32) -> Option<&mut StreamHeader> {
        self.read_type_mut::<StreamHeader>(self.header().stream_header_offset(stream_idx)?)
    }

    #[inline]
    fn stream_runlist_mut(&mut self, stream_idx: u32) -> Option<&mut StreamRunlist> {
        self.read_type_mut::<StreamRunlist>(self.header().stream_runlist_offset(stream_idx)?)
    }

    #[inline]
    fn get_sectors_mut(&mut self, range: Range<u32>) -> Option<&mut [u8]> {
        let sector_size = self.header().sector_size as usize;
        self.mapping_mut().get_mut(Range {
            start: range.start as usize * sector_size,
            end: range.end as usize * sector_size,
        })
    }

    fn stream_mut(&mut self, stream_idx: u32) -> Option<StreamMut<Self>> {
        if self.stream_header(stream_idx).is_some() && self.stream_runlist(stream_idx).is_some() {
            return Some(StreamMut {
                archive: self,
                stream_idx,
            });
        }
        None
    }
}

pub struct ArchiveMut {
    file: File,
    mapping: MmapMut,
}

impl Drop for ArchiveMut {
    fn drop(&mut self) {
        _ = self.file.unlock();
    }
}

impl ArchiveImpl for ArchiveMut {
    #[inline]
    fn mapping(&self) -> &[u8] {
        self.mapping.as_ref()
    }
}

impl imp::ArchiveMutImpl for ArchiveMut {
    #[inline]
    fn mapping_mut(&mut self) -> &mut [u8] {
        self.mapping.as_mut()
    }
}

impl ArchiveTrait for ArchiveMut {}

impl ArchiveMutTrait for ArchiveMut {}

pub struct CreateOptions;

impl ArchiveMut {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        file.try_lock_exclusive()?;

        let options = MmapOptions::new();
        let mapping = unsafe { options.map_mut(&file) }?;

        let this = ArchiveMut { file, mapping };

        if let Some(err) = this.validate() {
            return Err(err);
        }

        Ok(this)
    }

    pub fn create<P: AsRef<Path>>(_path: P, _options: CreateOptions) -> std::io::Result<Self> {
        todo!()
    }
}
