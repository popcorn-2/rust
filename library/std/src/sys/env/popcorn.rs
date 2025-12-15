use crate::ffi::{OsStr, OsString};
use crate::io;
pub use super::common::Env;
use crate::os::popcorn::ffi::OsStrExt;

pub fn env() -> Env {
    let strs = crate::sys::get_env();
    let mut vec = vec![];

    for s in strs {
        let s = <OsStr as OsStrExt>::as_str(s);
	    if let Some((key, val)) = s.split_once("=") {
		    vec.push((OsStr::from_str(key).to_owned(), OsStr::from_str(val).to_owned()));
	    }
    }

    Env::new(vec)
}

pub fn getenv(find_key: &OsStr) -> Option<OsString> {
    // fixme: not linear search
    env().find_map(|(key, val)| (find_key == key).then(|| val.clone()))
}

pub unsafe fn setenv(_: &OsStr, _: &OsStr) -> io::Result<()> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "cannot set env vars on this platform"))
}

pub unsafe fn unsetenv(_: &OsStr) -> io::Result<()> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "cannot unset env vars on this platform"))
}
