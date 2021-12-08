use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{Arc, RwLock};
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasi_common::dir::{WasiDir, ReaddirCursor, ReaddirEntity};
use wasi_common::file::{FdFlags, FileType, Filestat, OFlags, WasiFile};
use wasi_common::{Error, ErrorExt, SystemTimeSpec};
use std::any::Any;
use std::path::PathBuf;

pub mod map;
pub mod pipe;

pub use pipe::Handler;
pub use map::HandlerMap;

pub trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}
