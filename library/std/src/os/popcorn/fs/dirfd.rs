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

    /// Attempts to open a file according to `opts` relative to this directory.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not point to an existing file.
    /// Other errors may also be returned according to [`OpenOptions::open`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(dirfd, popcorn_std)]
    /// use std::{fs::{Dir, OpenOptions}, io::{self, Write}};
    /// use std::os::popcorn::fs::DirExt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let dir = Dir::open("foo")?;
    ///     let mut opts = OpenOptions::new();
    ///     opts.read(true).write(true);
    ///     let mut f = dir.open_file_with("bar.txt", &opts)?;
    ///     f.write(b"Hello, world!")?;
    ///     let contents = io::read_to_string(f)?;
    ///     assert_eq!(contents, "Hello, world!");
    ///     Ok(())
    /// }
    /// ```
    fn open_file_with<P: AsRef<Path>>(&self, path: P, opts: &OpenOptions) -> io::Result<File>;
}

impl Sealed for Dir {}

impl DirExt for Dir {
    fn config_dir() -> Option<Self> {
        None
    }

    fn working_dir() -> Option<Self> {
        None
    }

    fn open_file_with<P: AsRef<Path>>(&self, path: P, opts: &OpenOptions) -> io::Result<File> {
        self.as_inner().open_file(path.as_ref(), opts.as_inner()).map(|f| File::from_inner(f))
    }
}
