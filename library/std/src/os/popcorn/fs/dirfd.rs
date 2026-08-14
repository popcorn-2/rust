use crate::fs::{Dir, File, OpenOptions};
use crate::sealed::Sealed;
use crate::io;
use crate::path::Path;
use crate::sys::{AsInner, FromInner};

/// Platform-specific extensions to [`Dir`].
///
/// This trait is sealed: it cannot be implemented outside the standard library.
/// This is so that future additional methods are not breaking changes.
pub trait DirExt: Sealed + Sized {
    /// Gets the config directory for the current executable if it exists
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(dirfd, popcorn_std)]
    /// use std::{fs::{Dir, OpenOptions}, io::{self, Write}};
    /// use std::os::popcorn::fs::DirExt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let dir = Dir::config_dir().unwrap();
    ///     let mut opts = OpenOptions::new();
    ///     opts.read(true).write(true).create(true);
    ///     let mut f = dir.open_file_with("config.ini", &opts)?;
    ///     f.write(b"hello = world")?;
    ///     let contents = io::read_to_string(f)?;
    ///     assert_eq!(contents, "hello = world");
    ///     Ok(())
    /// }
    /// ```
    fn config_dir() -> Option<Self>;
    
    fn working_dir() -> Option<Self>;
}

impl Sealed for Dir {}

impl DirExt for Dir {
    fn config_dir() -> Option<Self> {
        None
    }

    fn working_dir() -> Option<Self> {
        None
    }
}
