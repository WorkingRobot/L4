use fs2::FileExt;
use std::{fs::File, ops::Deref};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Lock {
    Shared,
    Exclusive,
}

pub struct LockableFile {
    inner: File,
    lock: Option<Lock>,
}

impl From<File> for LockableFile {
    fn from(value: File) -> Self {
        Self {
            inner: value,
            lock: None,
        }
    }
}

impl Deref for LockableFile {
    type Target = File;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<File> for LockableFile {
    #[inline]
    fn as_ref(&self) -> &File {
        self.deref()
    }
}

impl Drop for LockableFile {
    fn drop(&mut self) {
        if self.lock.is_some() {
            _ = self.inner.unlock();
        }
    }
}

impl LockableFile {
    pub fn try_from_file(file: File, lock: Lock) -> std::io::Result<LockableFile> {
        let mut this = LockableFile {
            inner: file,
            lock: None,
        };
        this.try_lock(lock)?;
        Ok(this)
    }

    pub fn from_file(file: File, lock: Lock) -> std::io::Result<LockableFile> {
        let mut this = LockableFile {
            inner: file,
            lock: None,
        };
        this.lock(lock)?;
        Ok(this)
    }

    fn ensure_unlocked(&self) -> std::io::Result<()> {
        if self.is_locked() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "File is already locked",
            ));
        }
        Ok(())
    }

    pub fn lock_type(&self) -> Option<Lock> {
        self.lock
    }

    pub fn is_locked(&self) -> bool {
        self.lock.is_some()
    }

    pub fn is_shared(&self) -> bool {
        self.lock.map(|l| l == Lock::Shared).unwrap_or_default()
    }

    pub fn is_exclusive(&self) -> bool {
        self.lock.map(|l| l == Lock::Exclusive).unwrap_or_default()
    }

    pub fn try_lock(&mut self, lock: Lock) -> std::io::Result<()> {
        self.ensure_unlocked()?;
        match lock {
            Lock::Exclusive => self.inner.try_lock_exclusive(),
            Lock::Shared => self.inner.try_lock_shared(),
        }
    }

    pub fn lock(&mut self, lock: Lock) -> std::io::Result<()> {
        self.ensure_unlocked()?;
        match lock {
            Lock::Exclusive => self.inner.lock_exclusive(),
            Lock::Shared => self.inner.lock_shared(),
        }
    }

    pub fn lock_any(&mut self) -> std::io::Result<Lock> {
        let lock = match self.try_lock(Lock::Exclusive) {
            Ok(_) => Lock::Exclusive,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    self.lock(Lock::Shared)?;
                    Lock::Shared
                } else {
                    return Err(e);
                }
            }
        };
        Ok(lock)
    }

    pub fn unlock(&mut self) -> std::io::Result<()> {
        if !self.is_locked() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File is not locked",
            ));
        }
        self.inner.unlock()?;
        self.lock = None;
        Ok(())
    }

    pub fn downgrade(&mut self) -> std::io::Result<()> {
        if self.is_exclusive() {
            self.inner.try_lock_shared()?;
            self.inner.unlock()?;
        }
        Ok(())
    }
}
