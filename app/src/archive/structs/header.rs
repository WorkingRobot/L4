use crate::utils::align;

use super::{
    calculate_max_stream_count_aligned, Freelist, Reserved, SmallString, StreamHeader,
    StreamRunlist, Validatable, VersionString, HEADER_MAGIC, SECTOR_SIZE_RANGE,
};
use static_assertions::assert_eq_size;
use std::mem::size_of;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub magic: u32,
    pub version: Version,
    pub sector_size: u32,
    pub stream_count: u32,
    pub max_stream_count: u32,
    pub plugin_id: SmallString,
    pub app_id: SmallString,
    pub plugin_version: VersionString,
    pub app_version: VersionString,
    pub plugin_name: SmallString,
    pub app_name: SmallString,
    pub environment: SmallString,
    pub reserved: Reserved<12>,
}

assert_eq_size!(Header, [u8; 256]);

#[repr(u32, align(1))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Version {
    Unknown,
    Initial,
    Environment,
    UseIds,
    NewMagic,
    UseRust,
}

impl Header {
    pub fn freelist_offset(&self) -> usize {
        let header_size = size_of::<Header>();
        let stream_header_size = size_of::<StreamHeader>();
        align(
            header_size + self.max_stream_count as usize * stream_header_size,
            self.sector_size as usize,
        )
    }

    pub fn file_data_offset(&self) -> usize {
        let freelist_size = size_of::<Freelist>();
        let stream_runlist_size = size_of::<StreamRunlist>();
        align(
            self.freelist_offset()
                + freelist_size
                + self.max_stream_count as usize * stream_runlist_size,
            self.sector_size as usize,
        )
    }

    pub fn stream_header_offset(&self, stream_idx: u32) -> Option<usize> {
        if self.stream_count <= stream_idx {
            return None;
        }
        let header_size = size_of::<Header>();
        let stream_header_size = size_of::<StreamHeader>();
        Some(header_size + stream_idx as usize * stream_header_size)
    }

    pub fn stream_runlist_offset(&self, stream_idx: u32) -> Option<usize> {
        if self.stream_count <= stream_idx {
            return None;
        }
        let freelist_size = size_of::<Freelist>();
        let stream_runlist_size = size_of::<StreamRunlist>();
        Some(self.freelist_offset() + freelist_size + stream_idx as usize * stream_runlist_size)
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            magic: HEADER_MAGIC,
            version: Version::UseRust,
            sector_size: *SECTOR_SIZE_RANGE.start(),
            stream_count: 0,
            max_stream_count: calculate_max_stream_count_aligned(*SECTOR_SIZE_RANGE.start(), 1)
                .unwrap(),
            plugin_id: Default::default(),
            app_id: Default::default(),
            plugin_version: Default::default(),
            app_version: Default::default(),
            plugin_name: Default::default(),
            app_name: Default::default(),
            environment: Default::default(),
            reserved: Default::default(),
        }
    }
}

impl Validatable for Header {
    fn validate(&self) -> std::io::Result<()> {
        if self.magic != HEADER_MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid archive header (`HEADER_MAGIC`)",
            ));
        }

        if self.version != Version::UseRust {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unsupported archive version",
            ));
        }

        if !self.sector_size.is_power_of_two() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Sector size should be a power of two",
            ));
        }

        if !SECTOR_SIZE_RANGE.contains(&self.sector_size) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Sector size should be in a valid range (`SECTOR_SIZE_RANGE`)",
            ));
        }

        if self.stream_count > self.max_stream_count {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Stream count should not exceed max stream count",
            ));
        }

        let stream_headers_per_sector = self.sector_size / size_of::<StreamHeader>() as u32;
        if self.max_stream_count % stream_headers_per_sector != stream_headers_per_sector - 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Max stream count should align up to the next sector",
            ));
        }

        self.plugin_id.validate()?;
        self.app_id.validate()?;
        self.plugin_version.validate()?;
        self.app_version.validate()?;
        self.plugin_name.validate()?;
        self.app_name.validate()?;
        self.environment.validate()?;
        self.reserved.validate()?;

        Ok(())
    }
}
