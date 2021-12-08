use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::file::{Advice, FdFlags, FileType, Filestat, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};
use std::any::Any;

/// A general-purpose pipe that can be read from, written to, or both.
#[derive(Clone)]
pub struct Handler {
    read: Option<Arc<RwLock<Box<dyn Read>>>>,
    write: Option<Arc<RwLock<Box<dyn Write>>>>,
}

impl Handler {
    /// Returns whether the given permissions match the abilities of the handler.
    /// For example, a `Read` handle can't be written to.
    /// In addition, a `ReadWrite` handle is also a valid read-only handle.
    pub fn is_valid(&self, read: bool, write: bool) -> bool {
        match (self.read.is_some(), self.write.is_some()) {
            (true, true) => true,
            (true, _) => !write,
            (_, true) => !read,
            _ => false,
        }
    }

    fn borrow_read(&self) -> Option<std::sync::RwLockWriteGuard<Box<dyn Read + 'static>>> {
        if let Some(ref r) = self.read {
            RwLock::write(&r).ok()
        } else {
            None
        }
    }

    fn borrow_write(&self) -> Option<std::sync::RwLockWriteGuard<Box<dyn Write + 'static>>> {
        if let Some(ref w) = self.write {
            RwLock::write(&w).ok()
        } else {
            None
        }
    }
}

#[wiggle::async_trait]
impl WasiFile for Handler {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn datasync(&self) -> Result<(), Error> {
        Ok(()) // trivial: no implementation needed
    }

    async fn sync(&self) -> Result<(), Error> {
        Ok(()) // trivial
    }

    /// This is a `Pipe`.
    async fn get_filetype(&self) -> Result<FileType, Error> {
        Ok(FileType::Pipe)
    }

    /// Returns `APPEND` if this pipe is write-able
    async fn get_fdflags(&self) -> Result<FdFlags, Error> {
        if self.write.is_some() {
            Ok(FdFlags::APPEND)
        } else {
            Ok(FdFlags::empty())
        }
    }

    /// This pipe is fixed, you can't change it!
    async fn set_fdflags(&mut self, _fdflags: FdFlags) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Returns that this is a pipe.
    async fn get_filestat(&self) -> Result<Filestat, Error> {
        Ok(Filestat {
            device_id: 0,
            inode: 0,
            filetype: self.get_filetype().await?,
            nlink: 0,
            size: 0,
            atim: None,
            mtim: None,
            ctim: None,
        })
    }

    /// Yeah, you thought.
    async fn set_filestat_size(&self, _size: u64) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// What does advising even mean?
    /// "Aah! don't call this function!" is good advice.
    async fn advise(&self, _offset: u64, _len: u64, _advice: Advice) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Can not allocate.
    async fn allocate(&self, _offset: u64, _len: u64) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Reads from the slice if possible.
    async fn read_vectored<'a>(&self, bufs: &mut [io::IoSliceMut<'a>]) -> Result<u64, Error> {
        if let Some(mut r) = self.borrow_read() {
            let n = r.read_vectored(bufs)?;
            Ok(n.try_into()?)
        } else {
            Err(Error::badf())
        }
    }

    /// Can only read off the top.
    async fn read_vectored_at<'a>(
        &self,
        _bufs: &mut [io::IoSliceMut<'a>],
        _offset: u64,
    ) -> Result<u64, Error> {
        Err(Error::badf())
    }

    /// Writes to the buffer if applicable.
    async fn write_vectored<'a>(&self, bufs: &[io::IoSlice<'a>]) -> Result<u64, Error> {
        if let Some(mut w) = self.borrow_write() {
            let n = w.write_vectored(bufs)?;
            Ok(n.try_into()?)
        } else {
            Err(Error::badf())
        }
    }

    /// Can only read to the top.
    async fn write_vectored_at<'a>(
        &self,
        _bufs: &[io::IoSlice<'a>],
        _offset: u64,
    ) -> Result<u64, Error> {
        Err(Error::badf())
    }

    /// Can't seek, this ain't a file.
    async fn seek(&self, _pos: std::io::SeekFrom) -> Result<u64, Error> {
        Err(Error::badf())
    }

    /// No peeking.
    async fn peek(&self, _buf: &mut [u8]) -> Result<u64, Error> {
        Err(Error::badf())
    }

    /// This is fixed, can't set it.
    async fn set_times(
        &self,
        _atime: Option<SystemTimeSpec>,
        _mtime: Option<SystemTimeSpec>,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// We don't know how many bytes are ready.
    async fn num_ready_bytes(&self) -> Result<u64, Error> {
        Ok(0)
    }

    // I'm not touching these two, not sure what they're for.

    async fn readable(&self) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn writable(&self) -> Result<(), Error> {
        Err(Error::badf())
    }
}
