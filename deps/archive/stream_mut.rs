use super::stream::StreamImpl;
use super::structs::*;
use super::{file_mut::ArchiveMutTrait, stream::StreamTrait};
use crate::utils::Alignable;
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

    fn reserve(&mut self, additional: usize) {
        self.reserve_sectors(
            additional.div_ceil(self.archive().header().sector_size as usize) as u32,
        )
    }

    fn reserve_sectors(&mut self, mut additional: u32) {
        if additional == 0 {
            return;
        }

        // Extend current run and extend archive
        if let Some(last_entry) = self.runlist().last() {
            // If the last run for this stream touches the end of the archive, we can extend as far as we want
            if last_entry.sector_offset + last_entry.sector_count
                == self.archive().freelist().total_sector_count
            {
                // vvv TODO: do atomically
                self.runlist_mut().last_mut().unwrap().sector_count += additional;
                self.archive_mut().freelist_mut().total_sector_count += additional;
                // ^^^ TODO: do atomically
                return;
            }
        }

        // Utilize freelist
        while let Some(entry) = self.archive_mut().freelist_mut().pop() {
            if entry.sector_count > additional {
                let new_run = StreamRun {
                    stream_sector_offset: self.capacity_in_sectors(),
                    sector_offset: entry.sector_offset,
                    sector_count: additional,
                    ..Default::default()
                };

                let mut new_entry = entry;
                new_entry.sector_count -= additional;
                new_entry.sector_offset -= additional;

                // vvv TODO: do atomically
                self.archive_mut().freelist_mut().push(new_entry);
                self.runlist_mut().push(new_run);
                // ^^^ TODO: do atomically
                return;
            } else {
                let new_run = StreamRun {
                    stream_sector_offset: self.capacity_in_sectors(),
                    sector_offset: entry.sector_offset,
                    sector_count: entry.sector_count,
                    ..Default::default()
                };
                self.runlist_mut().push(new_run);

                additional -= entry.sector_count;
            }
        }

        // Create new run and extend archive
        let new_run = StreamRun {
            stream_sector_offset: self.capacity_in_sectors(),
            sector_offset: self.archive().freelist().total_sector_count,
            sector_count: additional,
            ..Default::default()
        };
        // vvv TODO: do atomically
        self.archive_mut().freelist_mut().total_sector_count += additional;
        let new_archive_size = self.archive().header().sector_size as usize
            * self.archive().freelist().total_sector_count as usize;
        self.archive_mut().reserve(new_archive_size);
        self.runlist_mut().push(new_run);
        // ^^^ TODO: do atomically
    }

    fn shrink_to_fit(&mut self) {
        self.shrink_to(self.len() as usize)
    }

    fn shrink_to(&mut self, min_capacity: usize) {
        let min_sector_count =
            min_capacity.div_ceil(self.archive().header().sector_size as usize) as u32;
        self.shrink_to_sectors(min_sector_count)
    }

    fn shrink_to_sectors(&mut self, min_capacity: u32) {
        if self.capacity_in_sectors() <= min_capacity {
            return;
        }

        while let Some(run) = self.runlist_mut().pop() {
            if run.stream_sector_offset >= min_capacity {
                // Place full run into freelist
                let new_entry = FreelistEntry {
                    sector_offset: run.sector_offset,
                    sector_count: run.sector_count,
                };
                self.archive_mut().freelist_mut().push(new_entry);
            } else if run.stream_sector_offset + run.sector_count > min_capacity {
                // Partially place run into freelist
                let new_run = StreamRun {
                    stream_sector_offset: run.stream_sector_offset,
                    sector_offset: run.sector_offset,
                    sector_count: min_capacity - run.stream_sector_offset,
                    ..Default::default()
                };
                let new_entry = FreelistEntry {
                    sector_offset: run.sector_offset + (min_capacity - run.stream_sector_offset),
                    sector_count: run.stream_sector_offset + run.sector_count - min_capacity,
                };
                // vvv TODO: do atomically
                self.runlist_mut().push(new_run);
                self.archive_mut().freelist_mut().push(new_entry);
                // ^^^ TODO: do atomically
                break;
            } else {
                // Ignore run
                self.runlist_mut().push(run);
                break;
            }
        }
    }

    fn clear(&mut self) {
        todo!()
    }

    fn truncate(&mut self, len: u64) {
        if len >= self.len() {
            return;
        }
        self.runlist_mut().size = len;
    }

    fn truncate_sectors(&mut self, len: u32) {
        self.truncate(len as u64 * self.archive().header().sector_size as u64)
    }

    fn resize(&mut self, new_len: u64) {
        if new_len > self.len() {
            self.reserve_sectors(new_len.align_to(self.archive().header().sector_size) as u32);
            self.runlist_mut().size = new_len;
        } else {
            self.truncate(new_len);
        }
    }

    fn resize_sectors(&mut self, _new_len: u32) {
        todo!()
    }

    fn extend_from_slice(&mut self, other: &[u8]) {
        self.write(self.len(), other);
    }

    fn write(&mut self, offset: u64, mut data: &[u8]) -> Option<()> {
        let mut iter = self.iter_bytes_mut(offset..offset + data.len() as u64)?;
        while let Some(range) = iter.next() {
            let (data_to_write, next_data) = data.split_at(range.len());
            range.copy_from_slice(data_to_write);
            data = next_data;
        }
        Some(())
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
    // Just use a `while let Some(slice) = iter.next()` loop for now
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut [u8]> {
        if self.current_run_idx > self.end_run_idx {
            return None;
        }

        let offset = self.current_run_offset;
        let run_idx = self.current_run_idx;

        self.current_run_offset = 0;
        self.current_run_idx += 1;

        let run = &self.runlist().get(run_idx)?;
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
