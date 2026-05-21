use core::mem::ManuallyDrop;
use crate::ffi::OsString;
use crate::process::Command;
use crate::sys::AsInnerMut;
use super::io::{OwnedHandle, FromRawHandle, AsRawHandle};
use crate::io::Error;

pub trait CommandExt: crate::sealed::Sealed {
    fn handle(&mut self, id: OsString, handle: impl Into<Handle>) -> &mut Command;
}

impl CommandExt for Command {
    #[track_caller]
    fn handle(&mut self, id: OsString, handle: impl Into<Handle>) -> &mut Command {
        let handle = handle.into();
        let handle = match handle {
            Handle::Null => {
                self.as_inner_mut().remove_handle(id.as_os_str());
                return self;
            },
            Handle::Inherit => {
                crate::os::popcorn::env::get_handle_untyped(id.as_os_str())
                    .ok_or(Error::from_raw_os_error(8))
                    .and_then(|h| h.try_clone_to_owned())
            },
            Handle::Handle(owned) => Ok(owned),
        };
        self.as_inner_mut().handle(id, handle);
        self
    }
}

#[derive(Debug)]
pub enum Handle {
    Null,
    Inherit,
    Handle(OwnedHandle),
}

pub fn inherit() -> Handle { Handle::Inherit }

pub fn null() -> Handle { Handle::Null }

pub fn inherit_from<T>(from: OwnedHandle<T>) -> Handle {
    let from = ManuallyDrop::new(from);
    Handle::Handle(
        unsafe { OwnedHandle::from_raw_handle(from.as_raw_handle()) }
    )
}
