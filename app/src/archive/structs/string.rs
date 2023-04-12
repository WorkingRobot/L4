#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArchiveString<const N: usize> {
    size: u8,
    data: [u8; N],
}

impl<const N: usize> ArchiveString<N> {
    pub fn as_str(&self) -> Option<&str> {
        if self.size as usize > N {
            return None;
        }
        std::str::from_utf8(&self.data[..self.size as usize]).ok()
    }
}

pub type SmallString = ArchiveString<31>;
pub type LargeString = ArchiveString<127>;
