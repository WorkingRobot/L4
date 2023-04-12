use super::{Freelist, SmallString, StreamHeader, StreamRunlist, Version};
use static_assertions::assert_eq_size;
use std::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub magic: u32,
    pub version: Version,
    pub sector_size: u32,
    pub stream_count: u32,
    pub max_stream_count: u32,
    pub plugin_id: SmallString,
    pub app_id: SmallString,
    pub plugin_version: SmallString,
    pub app_version: SmallString,
    pub plugin_name: SmallString,
    pub app_name: SmallString,
    pub environment: SmallString,
    _reserved: [u8; 12],
}

assert_eq_size!(Header, [u8; 256]);

impl Header {
    #[inline]
    fn align_offset(value: usize, align_to: usize) -> usize {
        (value + align_to - 1) & !(align_to - 1)
    }

    pub fn freelist_offset(&self) -> usize {
        let header_size = size_of::<Header>();
        let stream_header_size = size_of::<StreamHeader>();
        Self::align_offset(
            header_size + self.max_stream_count as usize * stream_header_size,
            self.sector_size as usize,
        )
    }

    pub fn file_data_offset(&self) -> usize {
        let freelist_size = size_of::<Freelist>();
        let stream_runlist_size = size_of::<StreamRunlist>();
        Self::align_offset(
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
