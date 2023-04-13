use std::ops::RangeInclusive;

pub trait Validatable {
    fn validate(&self) -> std::io::Result<()>;
}

pub(super) const HEADER_MAGIC: u32 = 0x6b2de8b2;
pub(super) const SECTOR_SIZE_RANGE: RangeInclusive<u32> = 1 << 12..=1 << 20;
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
