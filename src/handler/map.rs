use std::collections::HashMap;
use std::any::Any;
use std::path::PathBuf;
use std::sync::Arc;

use wasi_common::dir::{WasiDir, ReaddirCursor, ReaddirEntity};
use wasi_common::file::{FdFlags, FileType, Filestat, OFlags, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};

use super::Handler;

pub type HandlerMapInner = HashMap<&'static str, Handler>;

pub struct HandlerMap {
    map: Arc<HandlerMapInner>,
}

impl From<HandlerMapInner> for HandlerMap {
    fn from(map: HandlerMapInner) -> Self {
        let map = Arc::new(map);
        HandlerMap { map }
    }
}

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
        _read: bool,
        _write: bool,
        _fdflags: FdFlags,
    ) -> Result<Box<dyn WasiFile>, Error> {
        let handler = self.map.get(path).ok_or_else(|| Error::not_found())?;
        Ok(Box::new(handler.clone()))
    }

    /// Directory is fixed and cannot be opened.
    async fn open_dir(&self, _symlink_follow: bool, _path: &str) -> Result<Box<dyn WasiDir>, Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and can not be modified.
    async fn create_dir(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and cannot be read.
    async fn readdir(
        &self,
        _cursor: ReaddirCursor,
    ) -> Result<Box<dyn Iterator<Item = Result<ReaddirEntity, Error>> + Send>, Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and cannot be modified.
    async fn symlink(&self, _old_path: &str, _new_path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and cannot be modified.
    async fn remove_dir(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and cannot be modified.
    async fn unlink_file(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and does not have links.
    async fn read_link(&self, _path: &str) -> Result<PathBuf, Error> {
        Err(Error::badf())
    }

    /// Returns that this is indeed a `Directory`.
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

    /// Returns the `filestat` of the file at the provided path.
    async fn get_path_filestat(&self, path: &str, _follow_symlinks: bool)
        -> Result<Filestat, Error> {
            // SAFETY: the implementation of open_file never reads these arguments
            let o_flags:  OFlags  = unsafe { std::mem::transmute(0) };
            let fd_flags: FdFlags = unsafe { std::mem::transmute(0) };

            // grab the file and have it return it's stats
            let file = self.open_file(false, path, o_flags, false, false, fd_flags).await?;
            file.get_filestat().await
        }

    /// Directory is fixed and cannot be modified.
    async fn rename(
        &self,
        _path: &str,
        _dest_dir: &dyn WasiDir,
        _dest_path: &str,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and cannot be modified.
    async fn hard_link(
        &self,
        _path: &str,
        _target_dir: &dyn WasiDir,
        _target_path: &str,
    ) -> Result<(), Error> {
        Err(Error::badf())
    }

    /// Directory is fixed and cannot be modified.
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
