use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasi_common::dir::{WasiDir, ReaddirCursor, ReaddirEntity};
use wasi_common::file::{FdFlags, FileType, Filestat, OFlags, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};
use std::any::Any;
use std::path::PathBuf;
use super::Handler;

pub struct HandlerMap {
    map: Arc<HashMap<&'static str, Handler>>,
}

unsafe impl Send for Handler {}
unsafe impl Send for HandlerMap {}
unsafe impl Sync for Handler {}
unsafe impl Sync for HandlerMap {}

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

        Ok(Box::new(*handler.clone()))
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
