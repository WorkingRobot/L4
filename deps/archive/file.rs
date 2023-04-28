use self::imp::ArchiveImpl;
use super::stream::Stream;
use super::structs::*;
use crate::mmio::MappedFile;
use crate::utils::{Lock, LockableFile};
use std::{fs::OpenOptions, ops::Range, path::Path};

pub(super) mod imp {
    use std::io;

    use crate::archive::structs::{Freelist, Header, Validatable};

    pub trait ArchiveImpl {
        fn mapping(&self) -> &[u8];

        fn validate(&self) -> io::Result<()> {
            self.read_type::<Header>(0)
                .ok_or(io::Error::new(
                    io::ErrorKind::Other,
                    "File is too small to hold header",
                ))
                .and_then(|header| header.validate().and(Ok(header)))
                .and_then(|header| {
                    self.read_type::<Freelist>(header.freelist_offset())
                        .ok_or(io::Error::new(
                            io::ErrorKind::Other,
                            "File is too small to hold freelist",
                        ))
                })
                .and_then(|freelist| freelist.validate().and(Ok(freelist)))?;
            Ok(())
        }

        #[inline]
        fn read_type<T>(&self, offset: usize) -> Option<&T> {
            let len = std::mem::size_of::<T>();

            let slice = self.mapping().get(offset..offset + len)?;
            let data = unsafe { &*slice.as_ptr().cast::<T>() };
            Some(data)
        }
    }
}

pub trait ArchiveTrait: imp::ArchiveImpl + Sized + 'static {
    #[inline]
    fn header(&self) -> &Header {
        self.read_type::<Header>(0).unwrap()
    }

    #[inline]
    fn freelist(&self) -> &Freelist {
        self.read_type::<Freelist>(self.header().freelist_offset())
            .unwrap()
    }

    #[inline]
    fn stream_header(&self, stream_idx: u32) -> Option<&StreamHeader> {
        self.read_type::<StreamHeader>(self.header().stream_header_offset(stream_idx)?)
    }

    #[inline]
    fn stream_runlist(&self, stream_idx: u32) -> Option<&StreamRunlist> {
        self.read_type::<StreamRunlist>(self.header().stream_runlist_offset(stream_idx)?)
    }

    #[inline]
    fn get_sectors(&self, range: Range<u32>) -> Option<&[u8]> {
        let sector_size = self.header().sector_size as usize;
        self.mapping().get(Range {
            start: range.start as usize * sector_size,
            end: range.end as usize * sector_size,
        })
    }

    fn stream(&self, stream_idx: u32) -> Option<Stream<Self>> {
        if self.stream_header(stream_idx).is_some() && self.stream_runlist(stream_idx).is_some() {
            return Some(Stream {
                archive: self,
                stream_idx,
            });
        }
        None
    }
}

pub struct Archive {
    pub(super) file: LockableFile,
    pub(super) mapping: MappedFile,
}

impl imp::ArchiveImpl for Archive {
    #[inline]
    fn mapping(&self) -> &[u8] {
        self.mapping.as_ref()
    }
}

impl ArchiveTrait for Archive {}

impl Archive {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file =
            LockableFile::try_from_file(OpenOptions::new().read(true).open(path)?, Lock::Shared)?;

        let mapping = unsafe { MappedFile::new(&*file) }?;

        let this = Archive { file, mapping };
        this.validate()?;

        Ok(this)
    }
}
