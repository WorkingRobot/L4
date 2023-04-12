#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Version {
    Unknown,
    Initial,
    Environment,
    UseIds,
    NewMagic,
    UseRust,
}
