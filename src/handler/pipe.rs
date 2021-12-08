use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::file::{Advice, FdFlags, FileType, Filestat, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};
use std::any::Any;

/// A general-purpose pipe that can be read from, written to, or both.
/// When you write some data to it, it will process the data and pump the result to read
#[derive(Clone)]
pub struct Handler {
    to_process: Vec<u8>,
    processor:  Arc<dyn FnMut(Vec<u8>) -> Result<Vec<u8>, ()>>
}

impl Handler {
    pub fn new<T>(
        shared_state: Arc<RwLock<T>>,
        handler: Box<dyn Fn(T, ) -> T>,
    ) -> Self {
        todo!()
    }

    pub fn new_nonsense() -> Self {
        Handler {
            to_process: "Hello Read".as_bytes().to_vec(),
            processor:  Arc::new(|inp, out| { out = &mut inp; Ok(()) } ),
        }
    }

    pub fn new_shared_state<T>(
        shared_state: Arc<RwLock<T>>,
        stepper: Box<dyn Fn(T, &mut dyn Read, &mut dyn Write) -> T>,
    ) -> Handler {
        todo!()
    }

    // /// Returns whether the given permissions match the abilities of the handler.
    // /// For example, a `Read` handle can't be written to.
    // /// In addition, a `ReadWrite` handle is also a valid read-only handle.
    // pub fn is_valid(&self, read: bool, write: bool) -> bool {
    //     match (self.read.is_some(), self.write.is_some()) {
    //         (true, true) => true,
    //         (true, _) => !write,
    //         (_, true) => !read,
    //         _ => false,
    //     }
    // }

    // fn borrow_read(&self) -> Option<std::sync::RwLockWriteGuard<Box<dyn Read + 'static>>> {
    //     if let Some(ref r) = self.read {
    //         RwLock::write(&r).ok()
    //     } else {
    //         None
    //     }
    // }
    //
    // fn borrow_write(&self) -> Option<std::sync::RwLockWriteGuard<Box<dyn Write + 'static>>> {
    //     if let Some(ref w) = self.write {
    //         RwLock::write(&w).ok()
    //     } else {
    //         None
    //     }
    // }
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

    /// Returns `APPEND`
    async fn get_fdflags(&self) -> Result<FdFlags, Error> {
        Ok(FdFlags::APPEND)
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
        let to_process = std::mem::replace(&mut self.to_process, vec![]);
        let processed = (self.processor)(to_process).or_else(|_| Err(Error::invalid_argument()))?;
        let n = (&processed as &[u8]).read_vectored(bufs)?;
        Ok(n.try_into()?)
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
        // TODO: fix
        let n = (&self.to_process as &mut [u8]).write_vectored(&bufs)?;
        Ok(n.try_into()?)
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
