use std::{
    mem::size_of,
    ops::{Deref, Range},
};

use static_assertions::assert_eq_size;

assert_eq_size!(Header, [u8; 256]);
assert_eq_size!(StreamHeader, [u8; 256]);
assert_eq_size!(Freelist, [u8; 32768]);
assert_eq_size!(StreamRunlist, [u8; 16384]);

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub magic: u32,
    pub version: Version,
    pub sector_size: u32,
    pub stream_count: u32,
    pub max_stream_count: u32,
    pub plugin_id: SmallString,
    pub app_id: SmallString,
    pub plugin_version: SmallString,
    pub app_version: SmallString,
    pub plugin_name: SmallString,
    pub app_name: SmallString,
    pub environment: SmallString,
    _reserved: [u8; 12],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamHeader {
    pub id: SmallString,
    _reserved: [u8; 32],
    pub context: [u8; 192],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Freelist {
    pub entry_count: u32,
    pub total_sector_count: u32,
    pub entries: [FreelistEntry; 4095],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FreelistEntry {
    pub sector_count: u32,
    pub sector_offset: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamRunlist {
    pub entry_count: u32,
    _reserved: [u8; 4],
    pub size: u64,
    pub entries: [StreamEntry; 1023],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamEntry {
    pub stream_sector_offset: u32,
    pub sector_offset: u32,
    pub sector_count: u32,
    _reserved: [u8; 4],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Version {
    Unknown,
    Initial,
    Environment,
    UseIds,
    NewMagic,
    UseRust,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArchiveString<const N: usize> {
    size: u8,
    data: [u8; N],
}

impl Header {
    fn align_offset(value: usize, align_to: usize) -> usize {
        (value + align_to - 1) & !(align_to - 1)
    }

    pub fn freelist_offset(&self) -> usize {
        let header_size = size_of::<Header>();
        let stream_header_size = size_of::<StreamHeader>();
        Self::align_offset(
            header_size + self.max_stream_count as usize * stream_header_size,
            self.sector_size as usize,
        )
    }

    pub fn file_data_offset(&self) -> usize {
        let freelist_size = size_of::<Freelist>();
        let stream_runlist_size = size_of::<StreamRunlist>();
        Self::align_offset(
            self.freelist_offset()
                + freelist_size
                + self.max_stream_count as usize * stream_runlist_size,
            self.sector_size as usize,
        )
    }

    pub fn stream_header_offset(&self, stream_idx: u32) -> Option<usize> {
        if self.stream_count <= stream_idx {
            return None;
        }
        let header_size = size_of::<Header>();
        let stream_header_size = size_of::<StreamHeader>();
        Some(header_size + stream_idx as usize * stream_header_size)
    }

    pub fn stream_runlist_offset(&self, stream_idx: u32) -> Option<usize> {
        if self.stream_count <= stream_idx {
            return None;
        }
        let freelist_size = size_of::<Freelist>();
        let stream_runlist_size = size_of::<StreamRunlist>();
        Some(self.freelist_offset() + freelist_size + stream_idx as usize * stream_runlist_size)
    }
}

impl Deref for Freelist {
    type Target = [FreelistEntry];

    fn deref(&self) -> &Self::Target {
        let entry_count = usize::min(self.entry_count as usize, self.entries.len());
        &self.entries[..entry_count]
    }
}

impl AsRef<[FreelistEntry]> for Freelist {
    fn as_ref(&self) -> &[FreelistEntry] {
        self.deref()
    }
}

impl Deref for StreamRunlist {
    type Target = [StreamEntry];

    fn deref(&self) -> &Self::Target {
        let entry_count = usize::min(self.entry_count as usize, self.entries.len());
        &self.entries[..entry_count]
    }
}

impl AsRef<[StreamEntry]> for StreamRunlist {
    fn as_ref(&self) -> &[StreamEntry] {
        self.deref()
    }
}

impl FreelistEntry {
    pub fn archive_sector_range(&self) -> Range<u32> {
        Range {
            start: self.sector_offset,
            end: self.sector_count + self.sector_offset,
        }
    }
}

impl StreamEntry {
    pub fn archive_sector_range(&self) -> Range<u32> {
        Range {
            start: self.sector_offset,
            end: self.sector_count + self.sector_offset,
        }
    }

    pub fn stream_sector_range(&self) -> Range<u32> {
        Range {
            start: self.stream_sector_offset,
            end: self.stream_sector_offset + self.sector_count,
        }
    }
}

impl<const N: usize> ArchiveString<N> {
    pub fn as_str(&self) -> Option<&str> {
        if self.size as usize > N {
            return None;
        }
        std::str::from_utf8(&self.data[..self.size as usize]).ok()
    }
}

type SmallString = ArchiveString<31>;
type LargeString = ArchiveString<127>;
