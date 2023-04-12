use super::Validatable;
use semver::Version as SemVersion;
use static_assertions::assert_eq_size;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArchiveString<const N: usize> {
    size: u8,
    data: [u8; N],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VersionString {
    inner: SmallString,
}

assert_eq_size!(VersionString, [u8; 32]);

impl<const N: usize> ArchiveString<N> {
    pub fn as_str(&self) -> Option<&str> {
        if self.size as usize > N {
            return None;
        }
        std::str::from_utf8(&self.data[..self.size as usize]).ok()
    }
}

impl<const N: usize> Validatable for ArchiveString<N> {
    fn validate(&self) -> std::io::Result<()> {
        self.as_str()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "ArchiveString should be valid UTF-8",
            ))
            .and_then(|s| {
                if s.is_empty() {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "ArchiveString should not be empty",
                    ))
                } else {
                    Ok(())
                }
            })?;
        Ok(())
    }
}

pub type SmallString = ArchiveString<31>;
pub type LargeString = ArchiveString<127>;

impl TryFrom<VersionString> for SemVersion {
    type Error = std::io::Error;

    fn try_from(value: VersionString) -> Result<Self, Self::Error> {
        SemVersion::parse(value.inner.as_str().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VersionString should be valid UTF-8",
            )
        })?)
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VersionString should be a valid SemVer string",
            )
        })
    }
}

impl Validatable for VersionString {
    fn validate(&self) -> std::io::Result<()> {
        let _: SemVersion = (*self).try_into()?;
        Ok(())
    }
}
