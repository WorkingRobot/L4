use super::super::heap::is_heap;
use super::Validatable;
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

impl PartialOrd for FreelistEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // This is reversed in order to create a min heap, not a max heap
        let s = self.sector_count;
        let o = other.sector_count;
        o.partial_cmp(&s)
    }
}

impl PartialEq for FreelistEntry {
    fn eq(&self, other: &Self) -> bool {
        self.sector_count == other.sector_count
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

impl Validatable for Freelist {
    fn validate(&self) -> std::io::Result<()> {
        if self.entry_count as usize > self.entries.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entry count should be less than the length of the entries array",
            ));
        }

        if !is_heap(&self.entries[..self.entry_count as usize]) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entries array should be a min heap",
            ));
        }

        Ok(())
    }
}
