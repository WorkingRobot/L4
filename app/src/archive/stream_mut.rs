use super::stream::StreamImpl;
use super::structs::*;
use super::{file_mut::ArchiveMutTrait, stream::StreamTrait};
use std::ops::Range;

pub trait StreamMutImpl<A: ArchiveMutTrait> {
    fn archive_mut(&mut self) -> &mut A;
}

pub trait StreamMutTrait<A: ArchiveMutTrait>: StreamMutImpl<A> + StreamTrait<A> {
    #[inline]
    fn header_mut(&mut self) -> &mut StreamHeader {
        let stream_idx = self.stream_idx();
        self.archive_mut().stream_header_mut(stream_idx).unwrap()
    }

    #[inline]
    fn runlist_mut(&mut self) -> &mut StreamRunlist {
        let stream_idx = self.stream_idx();
        self.archive_mut().stream_runlist_mut(stream_idx).unwrap()
    }

    #[inline]
    fn archive_header_mut(&mut self) -> &mut Header {
        self.archive_mut().header_mut()
    }

    fn iter_bytes_mut(&mut self, range: Range<u64>) -> Option<StreamMutIter<A>> {
        let (start_run_idx, start_run_offset) = self.calculate_byte_location(range.start)?;
        let (end_run_idx, end_run_offset) = self.calculate_byte_location(range.end)?;

        let stream_idx = self.stream_idx();

        Some(StreamMutIter {
            archive: self.archive_mut(),
            stream_idx,
            current_run_idx: start_run_idx as usize,
            current_run_offset: start_run_offset as usize,
            end_run_idx: end_run_idx as usize,
            end_run_offset: end_run_offset as usize,
        })
    }
}

pub struct StreamMut<'a, A: ArchiveMutTrait> {
    pub(super) archive: &'a mut A,
    pub(super) stream_idx: u32,
}

impl<A: ArchiveMutTrait> StreamImpl<A> for StreamMut<'_, A> {
    #[inline]
    fn archive(&self) -> &A {
        self.archive
    }
}

impl<A: ArchiveMutTrait> StreamMutImpl<A> for StreamMut<'_, A> {
    #[inline]
    fn archive_mut(&mut self) -> &mut A {
        self.archive
    }
}

impl<A: ArchiveMutTrait> StreamTrait<A> for StreamMut<'_, A> {
    #[inline]
    fn stream_idx(&self) -> u32 {
        self.stream_idx
    }
}

impl<A: ArchiveMutTrait> StreamMutTrait<A> for StreamMut<'_, A> {}

pub struct StreamMutIter<'a, A: ArchiveMutTrait> {
    archive: &'a mut A,
    stream_idx: u32,
    current_run_idx: usize,
    current_run_offset: usize,
    end_run_idx: usize,
    end_run_offset: usize,
}

impl<A: ArchiveMutTrait> StreamImpl<A> for StreamMutIter<'_, A> {
    #[inline]
    fn archive(&self) -> &A {
        self.archive
    }
}

impl<A: ArchiveMutTrait> StreamMutImpl<A> for StreamMutIter<'_, A> {
    #[inline]
    fn archive_mut(&mut self) -> &mut A {
        self.archive
    }
}

impl<A: ArchiveMutTrait> StreamTrait<A> for StreamMutIter<'_, A> {
    #[inline]
    fn stream_idx(&self) -> u32 {
        self.stream_idx
    }
}

impl<A: ArchiveMutTrait> StreamMutTrait<A> for StreamMutIter<'_, A> {}

impl<A: ArchiveMutTrait> StreamMutIter<'_, A> {
    // Cannot impl Iterator until LendingIterator
    // becomes part of the standard library
    // Just use a `while let Some(slice) = iter.next` loop for now
    pub fn next(&mut self) -> Option<&mut [u8]> {
        if self.current_run_idx > self.end_run_idx {
            return None;
        }

        let offset = self.current_run_offset;
        let run_idx = self.current_run_idx;

        self.current_run_offset = 0;
        self.current_run_idx += 1;

        let run = &self.runlist()[run_idx];
        let mut slice = self.archive.get_sectors_mut(run.archive_sector_range())?;
        if run_idx == self.end_run_idx {
            slice = &mut slice[..self.end_run_offset];
        }
        if offset != 0 {
            slice = &mut slice[offset..];
        }

        Some(slice)
    }
}
