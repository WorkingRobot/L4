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

        if self.data[self.size as usize..].iter().any(|b| b != &0u8) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "ArchiveString should not have any data outside its valid range",
            ));
        }

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        if self.size != 0 || self.data.iter().any(|b| b != &0u8) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "String should be empty and filled with zeroes",
            ));
        }

        Ok(())
    }
}

impl<const N: usize> TryFrom<&str> for ArchiveString<N> {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > N {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "value should be at most N characters",
            ));
        }

        let mut this = Self {
            size: value.len() as u8,
            data: [0; N],
        };
        this.data[..value.len()].copy_from_slice(value.as_bytes());

        Ok(this)
    }
}

impl<const N: usize> Default for ArchiveString<N> {
    fn default() -> Self {
        // Panics if N < "Unknown".len()
        "Unknown".try_into().unwrap()
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

impl TryFrom<SemVersion> for VersionString {
    type Error = std::io::Error;

    fn try_from(value: SemVersion) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: value.to_string().as_str().try_into()?,
        })
    }
}

impl Default for VersionString {
    fn default() -> Self {
        SemVersion::new(0, 0, 0).try_into().unwrap()
    }
}

impl Validatable for VersionString {
    fn validate(&self) -> std::io::Result<()> {
        self.inner.validate()?;

        let _: SemVersion = (*self).try_into()?;

        Ok(())
    }

    fn validate_empty(&self) -> std::io::Result<()> {
        self.inner.validate_empty()?;

        Ok(())
    }
}
