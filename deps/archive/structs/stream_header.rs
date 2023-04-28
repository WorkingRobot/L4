use super::{Reserved, SmallString, Validatable};
use static_assertions::assert_eq_size;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamHeader {
    pub id: SmallString,
    pub reserved: Reserved<32>,
    pub context: [u8; 192],
}

assert_eq_size!(StreamHeader, [u8; 256]);

impl Default for StreamHeader {
    fn default() -> Self {
        Self {
            id: Default::default(),
            reserved: Default::default(),
            context: [0; 192],
        }
    }
}

impl Validatable for StreamHeader {
    fn validate(&self) -> std::io::Result<()> {
        self.id.validate()?;
        self.reserved.validate()?;

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        self.id.validate_empty()?;
        self.reserved.validate_empty()?;
        if self.context.iter().any(|b| b != &0u8) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Stream header context should be all zeros",
            ));
        }

        Ok(())
    }
}
