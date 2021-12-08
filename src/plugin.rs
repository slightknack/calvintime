use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};

pub trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

pub enum Handler {
    Read(Arc<RwLock<dyn Read>>),
    Writer(Arc<RwLock<dyn Write>>),
    ReadWrite(Arc<RwLock<dyn ReadWrite>>),
}

pub struct HandlerMap {
    map: Arc<HashMap<&'static str, Handler>>,
}

unsafe impl Send for Handler {}
unsafe impl Send for HandlerMap {}
unsafe impl Sync for Handler {}
unsafe impl Sync for HandlerMap {}

pub trait Plugin: Send + Sync + std::fmt::Debug {
    type State: Default + Sized;

    // Returns the name of this plugin
    fn name() -> &'static str where Self: Sized;

    // Returns this plugin's API
    fn api() -> HandlerMap where Self: Sized;
}


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

    async fn open_file(
        &self,
        _symlink_follow: bool,
        path: &str,
        _oflags: OFlags,
        _read: bool,
        _write: bool,
        _fdflags: FdFlags,
    ) -> Result<Box<dyn WasiFile>, Error> {
        let handler = self.map.get(path).ok_or_else(|| Error::badf())?;
        todo!()
    }

    async fn open_dir(&self, _symlink_follow: bool, _path: &str) -> Result<Box<dyn WasiDir>, Error> {
        Err(Error::badf())
    }

    async fn create_dir(&self, _path: &str) -> Result<(), Error> {
        Err(Error::badf())
    }

    // XXX the iterator here needs to be asyncified as well!
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

// #[derive(Default, Debug)]
// struct Counter {
//     number: usize
// }
//
// impl Counter {
//     fn reset(&mut self, read: &mut dyn Read, write: &mut dyn Write) {
//         let mut input = String::new();
//         read.read_to_string(&mut input);
//         let to_reset_to: usize = std::str::FromStr::from_str(&input).unwrap_or(0);
//         self.number = to_reset_to;
//     }
//
//     fn incr(&mut self, read: &mut dyn Read, write: &mut dyn Write) {
//         let mut input = String::new();
//         read.read_to_string(&mut input);
//         let to_increase_by: usize = std::str::FromStr::from_str(&input).unwrap_or(0);
//         self.number += to_increase_by;
//         write.write_all(self.number.to_string().as_bytes());
//     }
// }

// impl Plugin for Counter {
//     fn name() -> &'static str { "counter" }
//
//     fn api() -> HandlerMap<Self> {
//         let mut map: HandlerMap<Self> = HashMap::new();
//         map.insert("incr", Box::new(Counter::incr));
//         map.insert("reset", Box::new(Counter::reset));
//         return map;
//     }
// }
