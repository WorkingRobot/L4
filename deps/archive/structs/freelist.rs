use super::super::heap::is_heap;
use super::Validatable;
use crate::archive::heap::{pop_heap, push_heap};
use static_assertions::assert_eq_size;
use std::ops::{Deref, DerefMut, Range};

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

impl DerefMut for Freelist {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let entry_count = usize::min(self.entry_count as usize, self.entries.len());
        &mut self.entries[..entry_count]
    }
}

impl AsRef<[FreelistEntry]> for Freelist {
    fn as_ref(&self) -> &[FreelistEntry] {
        self.deref()
    }
}

impl Freelist {
    pub fn push(&mut self, entry: FreelistEntry) -> Option<()> {
        if self.entry_count as usize == self.entries.len() {
            None
        } else {
            self.entries[self.entry_count as usize] = entry;
            self.entry_count += 1;
            push_heap(self);
            Some(())
        }
    }

    pub fn pop(&mut self) -> Option<FreelistEntry> {
        if self.entry_count == 0 {
            None
        } else {
            pop_heap(self);
            self.entry_count -= 1;
            Some(self.entries[self.entry_count as usize])
        }
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

        let (valid_entries, empty_entries) = self.entries.split_at(self.entry_count as usize);
        if valid_entries.iter().any(|e| e.validate().is_err())
            || empty_entries.iter().any(|e| e.validate_empty().is_err())
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "All used entries should be valid and all unused entries should be empty",
            ));
        }

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        unimplemented!()
    }
}

impl Validatable for FreelistEntry {
    fn validate(&self) -> std::io::Result<()> {
        if self.sector_count == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entry sector count should be nonzero",
            ));
        }

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        if self.sector_count != 0 || self.sector_offset != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entry should be all zeroes",
            ));
        }

        Ok(())
    }
}
