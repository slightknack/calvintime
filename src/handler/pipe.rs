use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasi_common::dir::{WasiDir, ReaddirCursor, ReaddirEntity};
use wasi_common::file::{Advice, FdFlags, FileType, Filestat, OFlags, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};
use std::any::Any;
use std::path::PathBuf;
use super::ReadWrite;

pub enum Handler {
    Read(Arc<RwLock<dyn Read>>),
    Write(Arc<RwLock<dyn Write>>),
    ReadWrite(Arc<RwLock<dyn ReadWrite>>),
}

impl Handler {
    pub fn is_valid(&self, read: bool, write: bool) -> bool {
        use Handler::*;
        match self {
            Read(_) => !write,
            Write(_) => !read,
            ReadWrite(_) => true,
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
    async fn get_filetype(&self) -> Result<FileType, Error> {
        Ok(FileType::Pipe)
    }
    async fn get_fdflags(&self) -> Result<FdFlags, Error> {
        Ok(FdFlags::empty())
    }
    async fn set_fdflags(&mut self, _fdflags: FdFlags) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn get_filestat(&self) -> Result<Filestat, Error> {
        Ok(Filestat {
            device_id: 0,
            inode: 0,
            filetype: self.get_filetype().await?,
            nlink: 0,
            size: 0, // XXX no way to get a size out of a Read :(
            atim: None,
            mtim: None,
            ctim: None,
        })
    }
    async fn set_filestat_size(&self, _size: u64) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn advise(&self, offset: u64, len: u64, advice: Advice) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn allocate(&self, offset: u64, len: u64) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn read_vectored<'a>(&self, bufs: &mut [io::IoSliceMut<'a>]) -> Result<u64, Error> {
        let n = self.read_vectored(bufs).await?;
        Ok(n.try_into()?)
    }
    async fn read_vectored_at<'a>(
        &self,
        bufs: &mut [io::IoSliceMut<'a>],
        offset: u64,
    ) -> Result<u64, Error> {
        Err(Error::badf())
    }
    async fn write_vectored<'a>(&self, bufs: &[io::IoSlice<'a>]) -> Result<u64, Error> {
        Err(Error::badf())
    }
    async fn write_vectored_at<'a>(
        &self,
        bufs: &[io::IoSlice<'a>],
        offset: u64,
    ) -> Result<u64, Error> {
        Err(Error::badf())
    }
    async fn seek(&self, pos: std::io::SeekFrom) -> Result<u64, Error> {
        Err(Error::badf())
    }
    async fn peek(&self, buf: &mut [u8]) -> Result<u64, Error> {
        Err(Error::badf())
    }
    async fn set_times(
        &self,
        atime: Option<SystemTimeSpec>,
        mtime: Option<SystemTimeSpec>,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn num_ready_bytes(&self) -> Result<u64, Error> {
        Ok(0)
    }
    async fn readable(&self) -> Result<(), Error> {
        Err(Error::badf())
    }
    async fn writable(&self) -> Result<(), Error> {
        Err(Error::badf())
    }
}
