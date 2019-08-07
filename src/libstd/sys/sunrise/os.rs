use crate::error::Error as StdError;
use crate::ffi::{OsString, OsStr};
use crate::fmt;
use crate::io;
use crate::path::{self, PathBuf};
use crate::str;
use crate::sys::unsupported;

pub fn errno() -> i32 {
    0
}

pub fn error_string(_errno: i32) -> String {
    "operation successful".to_string()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(&'a Void);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("not supported on sunrise yet")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        match *self.0 {}
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on sunrise yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str {
        "not supported on sunrise yet"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    panic!("not supported on sunrise yet")
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub struct Env(Void);

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        match self.0 {}
    }
}

pub fn env() -> Env {
    panic!("not supported on sunrise yet")
}

pub fn getenv(k: &OsStr) -> io::Result<Option<OsString>> {
    panic!("not supported on sunrise yet")
}

pub fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    panic!("not supported on sunrise yet")
}

pub fn unsetenv(k: &OsStr) -> io::Result<()> {
    panic!("not supported on sunrise yet")
}

pub fn temp_dir() -> PathBuf {
    panic!("not supported on sunrise yet")
}

pub fn home_dir() -> Option<PathBuf> {
    panic!("not supported on sunrise yet")
}

pub fn exit(_code: i32) -> ! {
    panic!("not supported on sunrise yet")
}

pub fn getpid() -> u32 {
    panic!("not supported on sunrise yet")
}
