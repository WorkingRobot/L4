use super::SmallString;
use static_assertions::assert_eq_size;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StreamHeader {
    pub id: SmallString,
    _reserved: [u8; 32],
    pub context: [u8; 192],
}

assert_eq_size!(StreamHeader, [u8; 256]);
