use static_assertions::assert_eq_size;
use std::ops::{Deref, Range};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Freelist {
    pub entry_count: u32,
    pub total_sector_count: u32,
    pub entries: [FreelistEntry; 4095],
}

assert_eq_size!(Freelist, [u8; 32768]);

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FreelistEntry {
    pub sector_count: u32,
    pub sector_offset: u32,
}

assert_eq_size!(FreelistEntry, [u8; 8]);

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

impl FreelistEntry {
    pub fn archive_sector_range(&self) -> Range<u32> {
        Range {
            start: self.sector_offset,
            end: self.sector_count + self.sector_offset,
        }
    }
}
