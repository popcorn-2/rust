use crate::io;
use crate::sys::fs::{OpenOptions, File};
use crate::path::Path;
use crate::os::popcorn::io::{OwnedHandle, FromRawHandle, IntoRawHandle};
use crate::os::popcorn::proto::{io::Seek as AbiSeek, io::Read as AbiRead, io::Write as AbiWrite, fs::Dir as AbiDir};
use core::fmt;
use crate::sys::unsupported;

pub struct Dir {
    dir: OwnedHandle<&'static dyn AbiDir>,
}

impl Dir {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<Self> {
        unsupported()
    }

    pub fn open_file(&self, path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let create = match (opts.create, opts.create_new) {
            (_, true) => 1,
            (true, false) => 2,
            (false, false) => 0,
        };

        let handle = match (opts.read, opts.write) {
            (false, false) => {
                let handle = self.dir.open_file::<&dyn AbiSeek, _>(path, create, opts.append, opts.truncate)?;
                handle.into_raw_handle()
            },
            (true, false) => {
                let handle = self.dir.open_file::<(&dyn AbiSeek, &dyn AbiRead), _>(path, create, opts.append, opts.truncate)?;
                handle.into_raw_handle()
            },
            (false, true) => {
                let handle = self.dir.open_file::<(&dyn AbiSeek, &dyn AbiWrite), _>(path, create, opts.append, opts.truncate)?;
                handle.into_raw_handle()
            },
            (true, true) => {
                let handle = self.dir.open_file::<(&dyn AbiSeek, &dyn AbiRead, &dyn AbiWrite), _>(path, create, opts.append, opts.truncate)?;
                handle.into_raw_handle()
            },
        };

        Ok(unsafe { File::from_raw_handle(handle) })
    }
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dir").field("handle", &self.dir).finish()
    }
}
