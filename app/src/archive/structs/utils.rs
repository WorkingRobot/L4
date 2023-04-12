use std::ops::RangeInclusive;

pub trait Validatable {
    fn validate(&self) -> std::io::Result<()>;
}

pub(super) const HEADER_MAGIC: u32 = 0x6b2de8b2;
pub(super) const SECTOR_SIZE_RANGE: RangeInclusive<u32> = 1 << 12..=1 << 20;
