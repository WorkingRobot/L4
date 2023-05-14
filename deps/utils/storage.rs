use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs::{File, OpenOptions},
    io::Seek,
    path::Path,
};

pub struct Storage {
    file: File,
    data: HashMap<String, rmpv::Value>,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadRead(rmp_serde::decode::Error),
    DoesNotExist,
    BadSchema(rmpv::ext::Error),
    BadWrite(rmp_serde::encode::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Self::BadRead(e)
    }
}

impl From<rmpv::ext::Error> for Error {
    fn from(e: rmpv::ext::Error) -> Self {
        Self::BadSchema(e)
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Self::BadWrite(e)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(fmt),
            Self::BadRead(e) => e.fmt(fmt),
            Self::DoesNotExist => fmt.write_str("plugin does not have any saved data"),
            Self::BadSchema(e) => e.fmt(fmt),
            Self::BadWrite(e) => e.fmt(fmt),
        }
    }
}

impl std::error::Error for Error {}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let data: HashMap<String, rmpv::Value> = rmp_serde::from_read(&file)?;

        Ok(Self { file, data })
    }

    pub fn get<T: DeserializeOwned, P: AsRef<impl plugins_core::Plugin>>(
        &self,
        plugin: P,
    ) -> Result<T, Error> {
        self.data
            .get(plugin.as_ref().id())
            .ok_or(Error::DoesNotExist)
            .and_then(|d| -> Result<T, Error> {
                let v = rmpv::ext::from_value(d.clone())?;
                Ok(v)
            })
    }

    pub fn set<T: Serialize, P: AsRef<impl plugins_core::Plugin>>(
        &mut self,
        plugin: P,
        data: T,
    ) -> Result<(), Error> {
        _ = self
            .data
            .insert(plugin.as_ref().id().to_string(), rmpv::ext::to_value(data)?);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.file.set_len(0)?;
        self.file.seek(std::io::SeekFrom::Start(0))?;
        rmp_serde::encode::write(&mut self.file, &self.data)?;
        self.file.sync_data()?;
        Ok(())
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        _ = self.flush();
    }
}
