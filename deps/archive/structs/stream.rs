use super::{Reserved, Validatable};
use static_assertions::assert_eq_size;
use std::ops::{Deref, DerefMut, Range};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamRunlist {
    pub run_count: u32,
    pub reserved: Reserved<4>,
    pub size: u64,
    pub runs: [StreamRun; 1023],
}

assert_eq_size!(StreamRunlist, [u8; 16384]);

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone)]
pub struct StreamRun {
    pub stream_sector_offset: u32,
    pub sector_offset: u32,
    pub sector_count: u32,
    pub reserved: Reserved<4>,
}

assert_eq_size!(StreamRun, [u8; 16]);

impl Deref for StreamRunlist {
    type Target = [StreamRun];

    fn deref(&self) -> &Self::Target {
        let entry_count = usize::min(self.run_count as usize, self.runs.len());
        &self.runs[..entry_count]
    }
}

impl DerefMut for StreamRunlist {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let entry_count = usize::min(self.run_count as usize, self.runs.len());
        &mut self.runs[..entry_count]
    }
}

impl AsRef<[StreamRun]> for StreamRunlist {
    fn as_ref(&self) -> &[StreamRun] {
        self.deref()
    }
}

impl StreamRunlist {
    pub fn push(&mut self, run: StreamRun) -> Option<()> {
        if self.run_count as usize == self.runs.len() {
            None
        } else {
            self.runs[self.run_count as usize] = run;
            self.run_count += 1;
            Some(())
        }
    }

    pub fn pop(&mut self) -> Option<StreamRun> {
        if self.run_count == 0 {
            None
        } else {
            self.run_count -= 1;
            Some(self.runs[self.run_count as usize])
        }
    }
}

impl StreamRun {
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

impl Default for StreamRunlist {
    fn default() -> Self {
        Self {
            run_count: 0,
            reserved: Default::default(),
            size: 0,
            runs: [Default::default(); 1023],
        }
    }
}

impl Validatable for StreamRunlist {
    fn validate(&self) -> std::io::Result<()> {
        self.reserved.validate()?;

        if self.run_count as usize > self.runs.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entry count should be less than the length of the entries array",
            ));
        }

        let mut expected_sector_offset: u32 = 0;
        for entry in &self.runs[..self.run_count as usize] {
            entry.validate()?;

            if entry.stream_sector_offset != expected_sector_offset {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Stream sector offset should monotonically increase for every entry",
                ));
            }

            expected_sector_offset += entry.sector_count;
        }

        let (valid_runs, empty_runs) = self.runs.split_at(self.run_count as usize);
        if valid_runs.iter().any(|r| r.validate().is_err())
            || empty_runs.iter().any(|r| r.validate_empty().is_err())
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "All used runs should be valid and all unused runs should be empty",
            ));
        }

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        if self.run_count != 0 {}

        self.reserved.validate_empty()?;

        if self.size != 0 {}

        Ok(())
    }
}

impl Validatable for StreamRun {
    fn validate(&self) -> std::io::Result<()> {
        if self.sector_count == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Run sector count should be nonzero",
            ));
        }

        self.reserved.validate()?;

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        if self.stream_sector_offset != 0 || self.sector_offset != 0 || self.sector_offset != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Entry should be all zeroes",
            ));
        }

        self.reserved.validate_empty()?;

        Ok(())
    }
}
