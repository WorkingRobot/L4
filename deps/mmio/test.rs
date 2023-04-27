use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write},
    os::windows::prelude::OpenOptionsExt,
};

use winapi::um::winnt::GENERIC_ALL;

use super::*;

// Mostly taken from the memmap2 crate

#[test]
fn map_file() {
    let expected_len = 128;
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();

    file.set_len(expected_len as u64).unwrap();

    let mut mmap = unsafe { MappedFileMut::new(&file).unwrap() };
    let len = mmap.len();
    assert_eq!(expected_len, len);

    let zeros = vec![0; len];
    let incr: Vec<u8> = (0..len as u8).collect();

    // check that the mmap is empty
    assert_eq!(&zeros[..], &mmap[..]);

    // write values into the mmap
    (&mut mmap[..]).write_all(&incr[..]).unwrap();

    // read values back
    assert_eq!(&incr[..], &mmap[..]);
}

#[test]
fn map_empty_file() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    unsafe { MappedFile::new(&file) }.err().unwrap();
    unsafe { MappedFileMut::new(&file) }.err().unwrap();
}

#[test]
fn file_write() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.set_len(128).unwrap();

    let write = b"abc123";
    let mut read = [0u8; 6];

    let mut mmap = unsafe { MappedFileMut::new(&file).unwrap() };
    (&mut mmap[..]).write_all(write).unwrap();
    mmap.flush().unwrap();
    file.sync_data().unwrap();

    file.read_exact(&mut read).unwrap();
    assert_eq!(write, &read);
}

#[test]
fn flush_range() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.set_len(128).unwrap();
    let write = b"abc123";

    let mut mmap = unsafe { MappedFileMut::new(&file) }.unwrap();
    (&mut mmap[..]).write_all(write).unwrap();
    mmap.flush_range(0, write.len()).unwrap();
    file.sync_data().unwrap();
}

#[test]
fn index() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.set_len(128).unwrap();

    let mut mmap = unsafe { MappedFileMut::new(&file).unwrap() };

    mmap[0] = 42;
    assert_eq!(42, mmap[0]);
}

#[test]
fn sync_send() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.set_len(129).unwrap();

    let mmap = unsafe { MappedFileMut::new(&file).unwrap() };

    fn is_sync_send<T>(_val: T)
    where
        T: Sync + Send,
    {
    }

    is_sync_send(mmap);
}

#[test]
fn mprotect_file() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let mut options = OpenOptions::new();
    options.access_mode(GENERIC_ALL);

    let mut file = options
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("open");
    file.set_len(256_u64).expect("set_len");

    let mut mmap = unsafe { MappedFileMut::new(&file).expect("map_mut") };

    let write = b"abc123";
    let mut read = [0u8; 6];

    (&mut mmap[..]).write_all(write).unwrap();
    mmap.flush().unwrap();
    file.sync_data().unwrap();

    // The mmap contains the write
    (&mmap[..]).read_exact(&mut read).unwrap();
    assert_eq!(write, &read);

    // The file should contain the write
    file.read_exact(&mut read).unwrap();
    assert_eq!(write, &read);

    // another mmap should contain the write
    let mmap2 = unsafe { MappedFileMut::new(&file).unwrap() };
    (&mmap2[..]).read_exact(&mut read).unwrap();
    assert_eq!(write, &read);

    drop(mmap);
}

#[test]
fn reserve_file() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("mmap");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    file.set_len(128).unwrap();

    let write = b"abc123";
    let mut read = [0u8; 6];

    let mut mmap = unsafe { MappedFileMut::new(&file) }.unwrap();
    (&mut mmap[..]).write_all(write).unwrap();
    unsafe { mmap.reserve(1048576 + 128) }.unwrap();
    assert_eq!(mmap.len(), 1048576 + 128);
    (&mut mmap[1048576..]).write_all(write).unwrap();

    mmap.flush().unwrap();
    file.sync_data().unwrap();

    file.read_exact(&mut read).unwrap();
    assert_eq!(write, &read);
    file.seek(std::io::SeekFrom::End(-128)).unwrap();
    file.read_exact(&mut read).unwrap();
    assert_eq!(write, &read);
}
