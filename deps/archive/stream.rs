use super::file::ArchiveTrait;
use super::structs::*;
use std::{cmp::Ordering, ops::Range};
use superslice::Ext;

pub trait StreamImpl<A: ArchiveTrait> {
    fn archive(&self) -> &A;
}

pub trait StreamTrait<A: ArchiveTrait>: StreamImpl<A> {
    fn stream_idx(&self) -> u32;

    fn header(&self) -> &StreamHeader {
        self.archive().stream_header(self.stream_idx()).unwrap()
    }

    fn runlist(&self) -> &StreamRunlist {
        self.archive().stream_runlist(self.stream_idx()).unwrap()
    }

    fn id(&self) -> Option<&str> {
        self.header().id.as_str()
    }

    fn capacity(&self) -> u64 {
        self.capacity_in_sectors() as u64 * self.archive().header().sector_size as u64
    }

    fn len(&self) -> u64 {
        self.runlist().size
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn capacity_in_sectors(&self) -> u32 {
        self.runlist()
            .last()
            .map(|e| e.stream_sector_offset + e.sector_count)
            .unwrap_or_default()
    }

    fn calculate_byte_location(&self, offset: u64) -> Option<(u32, u64)> {
        match offset.cmp(&self.len()) {
            Ordering::Greater => None,
            Ordering::Equal => Some((self.runlist().run_count, 0)),
            Ordering::Less => {
                let sector_size = self.archive().header().sector_size as u64;
                let stream_sector_idx = (offset / sector_size) as u32;
                let run_idx = self.runlist().lower_bound_by_key(&stream_sector_idx, |r| {
                    r.stream_sector_offset + r.sector_count
                }) as u32;
                if run_idx == self.runlist().run_count {
                    return None;
                }
                Some((
                    run_idx,
                    offset
                        - (self.runlist()[run_idx as usize].stream_sector_offset as u64
                            * sector_size),
                ))
            }
        }
    }

    fn iter_bytes(&self, range: Range<u64>) -> Option<StreamIter<A>> {
        let (start_run_idx, start_run_offset) = self.calculate_byte_location(range.start)?;
        let (end_run_idx, end_run_offset) = self.calculate_byte_location(range.end)?;

        Some(StreamIter {
            archive: self.archive(),
            stream_idx: self.stream_idx(),
            current_run_idx: start_run_idx as usize,
            current_run_offset: start_run_offset as usize,
            end_run_idx: end_run_idx as usize,
            end_run_offset: end_run_offset as usize,
        })
    }
}

pub struct Stream<'a, A: ArchiveTrait> {
    pub(super) archive: &'a A,
    pub(super) stream_idx: u32,
}

impl<A: ArchiveTrait> StreamImpl<A> for Stream<'_, A> {
    #[inline]
    fn archive(&self) -> &A {
        self.archive
    }
}

impl<A: ArchiveTrait> StreamTrait<A> for Stream<'_, A> {
    #[inline]
    fn stream_idx(&self) -> u32 {
        self.stream_idx
    }
}

pub struct StreamIter<'a, A: ArchiveTrait> {
    archive: &'a A,
    stream_idx: u32,
    current_run_idx: usize,
    current_run_offset: usize,
    end_run_idx: usize,
    end_run_offset: usize,
}

impl<A: ArchiveTrait> StreamImpl<A> for StreamIter<'_, A> {
    #[inline]
    fn archive(&self) -> &A {
        self.archive
    }
}

impl<A: ArchiveTrait> StreamTrait<A> for StreamIter<'_, A> {
    #[inline]
    fn stream_idx(&self) -> u32 {
        self.stream_idx
    }
}

impl<'a, A: ArchiveTrait> Iterator for StreamIter<'a, A> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_run_idx > self.end_run_idx {
            return None;
        }

        let offset = self.current_run_offset;
        let run_idx = self.current_run_idx;

        self.current_run_offset = 0;
        self.current_run_idx += 1;

        let run = &self.runlist().get(run_idx)?;
        let mut slice = self.archive.get_sectors(run.archive_sector_range())?;
        if run_idx == self.end_run_idx {
            slice = &slice[..self.end_run_offset];
        }
        if offset != 0 {
            slice = &slice[offset..];
        }

        Some(slice)
    }
}
