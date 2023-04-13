use static_assertions::assert_eq_size;
use std::ops::{Deref, Range};

use super::{Reserved, Validatable};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamRunlist {
    pub entry_count: u32,
    _reserved: [u8; 4],
    pub size: u64,
    pub entries: [StreamEntry; 1023],
}

assert_eq_size!(StreamRunlist, [u8; 16384]);

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamEntry {
    pub stream_sector_offset: u32,
    pub sector_offset: u32,
    pub sector_count: u32,
    _reserved: [u8; 4],
}

assert_eq_size!(StreamEntry, [u8; 16]);

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

impl Validatable for StreamRunlist {
    fn validate(&self) -> std::io::Result<()> {
        self.reserved.validate()?;

        if self.entry_count as usize > self.entries.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entry count should be less than the length of the entries array",
            ));
        }

        let mut expected_sector_offset: u32 = 0;
        for entry in self.entries {
            entry.validate()?;

            if entry.stream_sector_offset != expected_sector_offset {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Stream sector offset should monotonically increase for every entry",
                ));
            }

            expected_sector_offset += entry.sector_count;
        }

        Ok(())
    }
}

impl Validatable for StreamEntry {
    fn validate(&self) -> std::io::Result<()> {
        self.reserved.validate()
    }
}
