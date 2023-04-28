use std::ops::RangeInclusive;

use super::StreamHeader;

pub const HEADER_MAGIC: u32 = 0x6b2de8b2;
pub const SECTOR_SIZE_RANGE: RangeInclusive<u32> = 1 << 12..=1 << 20;

pub fn calculate_max_stream_count_multiplier(sector_size: u32, multiplier: u32) -> Option<u32> {
    if !sector_size.is_power_of_two() {
        return None;
    }

    if !SECTOR_SIZE_RANGE.contains(&sector_size) {
        return None;
    }

    let stream_headers_per_sector = sector_size / std::mem::size_of::<StreamHeader>() as u32;

    Some(stream_headers_per_sector * multiplier - 1)
}

pub fn calculate_max_stream_count_aligned(
    sector_size: u32,
    minimum_stream_count: u32,
) -> Option<u32> {
    if !sector_size.is_power_of_two() {
        return None;
    }

    if !SECTOR_SIZE_RANGE.contains(&sector_size) {
        return None;
    }

    let stream_headers_per_sector = sector_size / std::mem::size_of::<StreamHeader>() as u32;

    Some(minimum_stream_count.next_multiple_of(stream_headers_per_sector) - 1)
}

pub trait Validatable {
    // Ensures the data contains valid data
    fn validate(&self) -> std::io::Result<()>;

    // Ensures the data contains empty, untouched data
    fn validate_empty(&self) -> std::io::Result<()>;
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
        self.validate_empty()
    }

    fn validate_empty(&self) -> std::io::Result<()> {
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
