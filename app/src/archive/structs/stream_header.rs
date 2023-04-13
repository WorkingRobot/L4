use super::{Reserved, SmallString, Validatable};
use static_assertions::assert_eq_size;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamHeader {
    pub id: SmallString,
    reserved: Reserved<32>,
    pub context: [u8; 192],
}

assert_eq_size!(StreamHeader, [u8; 256]);

impl Validatable for StreamHeader {
    fn validate(&self) -> std::io::Result<()> {
        self.id.validate()?;
        self.reserved.validate()?;

        Ok(())
    }
}
