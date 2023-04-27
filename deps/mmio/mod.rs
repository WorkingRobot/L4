mod file;
mod file_mut;
mod mmap;
#[cfg(test)]
mod test;

pub use file::MappedFile;
pub use file_mut::MappedFileMut;
