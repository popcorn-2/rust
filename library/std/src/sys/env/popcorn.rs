use crate::ffi::{OsStr, OsString};
use crate::io;
pub use super::common::Env;
use crate::os::popcorn::ffi::OsStrExt;
use crate::sync::{OnceLock, nonpoison::Mutex};
use crate::collections::HashMap;

fn internal_env() -> &'static Mutex<HashMap<OsString, OsString>> {
    static ENV: OnceLock<Mutex<HashMap<OsString, OsString>>> = OnceLock::new();

    ENV.get_or_init(|| {
        let strs = crate::sys::get_env();
        let mut map = HashMap::new();

        for s in strs {
            let s = <OsStr as OsStrExt>::as_str(s);
            if let Some((key, val)) = s.split_once("=") {
                map.insert(OsStr::from_str(key).to_owned(), OsStr::from_str(val).to_owned());
            }
        }

        Mutex::new(map)
    })
}

pub fn env() -> Env {
    Env::new(internal_env().lock().iter().map(|(k,v)| (k.clone(), v.clone())).collect())
}

pub fn getenv(key: &OsStr) -> Option<OsString> {
    internal_env().lock().get(key).map(|val| val.clone())
}

pub unsafe fn setenv(key: &OsStr, val: &OsStr) -> io::Result<()> {
    internal_env().lock().insert(key.to_owned(), val.to_owned());
    Ok(())
}

pub unsafe fn unsetenv(key: &OsStr) -> io::Result<()> {
    internal_env().lock().remove(key);
    Ok(())
}
