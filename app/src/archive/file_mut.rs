use super::lockable_file::{Lock, LockableFile};
use super::structs::*;
use super::{file::imp::ArchiveImpl, stream_mut::StreamMut};
use super::{file::ArchiveTrait, Archive};
use memmap2::{MmapMut, MmapOptions};
use std::{fs::OpenOptions, ops::Range, path::Path};

mod imp {
    pub trait ArchiveMutImpl {
        fn reserve(&mut self, minimum: usize) -> Option<()>;

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
    file: LockableFile,
    mapping: MmapMut,
}

impl ArchiveImpl for ArchiveMut {
    #[inline]
    fn mapping(&self) -> &[u8] {
        self.mapping.as_ref()
    }
}

impl imp::ArchiveMutImpl for ArchiveMut {
    fn reserve(&mut self, minimum: usize) -> Option<()> {
        None
    }

    #[inline]
    fn mapping_mut(&mut self) -> &mut [u8] {
        self.mapping.as_mut()
    }
}

impl ArchiveTrait for ArchiveMut {}

impl ArchiveMutTrait for ArchiveMut {}

pub struct CreateOptions {
    pub sector_size: u32,
    pub requested_stream_count: u32,
}

impl ArchiveMut {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = LockableFile::try_from_file(
            OpenOptions::new().read(true).write(true).open(path)?,
            Lock::Exclusive,
        )?;

        let options = MmapOptions::new();
        let mapping = unsafe { options.map_mut(&*file) }?;

        let this = ArchiveMut { file, mapping };
        this.validate()?;

        Ok(this)
    }

    pub fn create<P: AsRef<Path>>(path: P, options: CreateOptions) -> std::io::Result<Self> {
        let file = LockableFile::try_from_file(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(path)?,
            Lock::Exclusive,
        )?;

        let mapping = unsafe { MmapOptions::new().len(1 << 30).map_mut(&*file) }?;

        let max_stream_count =
            calculate_max_stream_count_aligned(options.sector_size, options.requested_stream_count)
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Sector size is invalid",
                ))?;

        let mut this = ArchiveMut { file, mapping };
        *this.header_mut() = Header {
            sector_size: options.sector_size,
            max_stream_count,
            ..Default::default()
        };

        Ok(this)
    }

    pub fn make_read_only(mut self) -> std::io::Result<Archive> {
        let mapping = self.mapping.make_read_only()?;
        self.file.downgrade()?;
        Ok(Archive {
            file: self.file,
            mapping,
        })
    }
}
