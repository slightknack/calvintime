use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::pipe::{ReadPipe, WritePipe};

pub trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

pub enum Handler {
    Read(Arc<RwLock<dyn Read>>),
    Write(Arc<RwLock<dyn Write>>),
    ReadWrite(Arc<RwLock<dyn ReadWrite>>),
}

impl Handler {
    fn is_valid(&self, read: bool, write: bool) -> bool {
        use Handler::*;
        match self {
            Read(_) => !write,
            Write(_) => !read,
            ReadWrite(_) => true,
        }
    }
}

use wasi_common::{
    file::{Advice, FdFlags, FileType, Filestat, WasiFile},
    Error, ErrorExt, SystemTimeSpec,
};
use std::any::Any;
use std::convert::TryInto;

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

pub struct HandlerMap {
    map: Arc<HashMap<&'static str, Handler>>,
}

unsafe impl Send for Handler {}
unsafe impl Send for HandlerMap {}
unsafe impl Sync for Handler {}
unsafe impl Sync for HandlerMap {}

use wasi_common::dir::{WasiDir, ReaddirCursor, ReaddirEntity};
use wasi_common::file::{FdFlags, FileType, Filestat, OFlags, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};
use std::any::Any;
use std::path::PathBuf;

#[wiggle::async_trait]
impl WasiDir for HandlerMap {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Open a file.
    /// A file must be opened in either read or write mode?
    async fn open_file(
        &self,
        _symlink_follow: bool,
        path: &str,
        _oflags: OFlags,
        read: bool,
        write: bool,
        _fdflags: FdFlags,
    ) -> Result<Box<dyn WasiFile>, Error> {
        let handler = self.map.get(path).ok_or_else(|| Error::not_found())?;
        if !handler.is_valid(read, write) {
            // is this the best one?
            Err(Error::invalid_argument())?
        }

        use Handler::*;
        let pipe = match handler {
            Read(r)  => Box::new(ReadPipe::from_shared(r.clone())),
            Write(w) => Box::new(WritePipe::from_shared(w.clone())),
            ReadWrite(_) => todo!(),
        };

        Ok(pipe);
        todo!()
    }

    async fn open_dir(&self, _symlink_follow: bool, _path: &str) -> Result<Box<dyn WasiDir>, Error> {
        Err(Error::badf())
    }

    async fn create_dir(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    async fn readdir(
        &self,
        _cursor: ReaddirCursor,
    ) -> Result<Box<dyn Iterator<Item = Result<ReaddirEntity, Error>> + Send>, Error> {
        Err(Error::badf())
    }

    async fn symlink(&self, _old_path: &str, _new_path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    async fn remove_dir(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    async fn unlink_file(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    async fn read_link(&self, _path: &str) -> Result<PathBuf, Error> {
        Err(Error::badf())
    }

    async fn get_filestat(&self) -> Result<Filestat, Error> {
        Ok(Filestat {
            device_id: 0,
            inode: 0,
            filetype: FileType::Directory,
            nlink: 0,
            size: 0,
            atim: None,
            mtim: None,
            ctim: None,
        })
    }

    async fn get_path_filestat(&self, path: &str, _follow_symlinks: bool)
        -> Result<Filestat, Error> {
            // SAFETY: the implementation of open_file never reads these arguments
            let o_flags:  OFlags  = unsafe { std::mem::transmute(0) };
            let fd_flags: FdFlags = unsafe { std::mem::transmute(0) };

            // grab the file and have it return it's stats
            let file = self.open_file(false, path, o_flags, false, false, fd_flags).await?;
            file.get_filestat().await
        }

    async fn rename(
        &self,
        _path: &str,
        _dest_dir: &dyn WasiDir,
        _dest_path: &str,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }

    async fn hard_link(
        &self,
        _path: &str,
        _target_dir: &dyn WasiDir,
        _target_path: &str,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }

    async fn set_times(
        &self,
        _path: &str,
        _atime: Option<SystemTimeSpec>,
        _mtime: Option<SystemTimeSpec>,
        _follow_symlinks: bool,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }
}
