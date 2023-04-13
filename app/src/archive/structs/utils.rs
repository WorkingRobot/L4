use std::ops::RangeInclusive;

pub const HEADER_MAGIC: u32 = 0x6b2de8b2;
pub const SECTOR_SIZE_RANGE: RangeInclusive<u32> = 1 << 12..=1 << 20;

pub trait Validatable {
    fn validate(&self) -> std::io::Result<()>;
}

#[derive(Debug, Copy, Clone)]
pub struct Reserved<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Default for Reserved<N> {
    fn default() -> Self {
        Self { data: [0; N] }
    }
}

impl<const N: usize> Validatable for Reserved<N> {
    fn validate(&self) -> std::io::Result<()> {
        if self.data.iter().all(|b| b == &0u8) {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Reserved data should be all zeros",
            ))
        }
    }
}
