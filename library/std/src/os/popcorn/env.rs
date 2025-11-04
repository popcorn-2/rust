#![stable(feature = "popcorn_std", since = "1.88.0")]

use crate::{ffi::{OsStr, OsString}, os::popcorn::{handle::{FromRawHandle, RawHandle}, proto::ProtocolTuple}};
use super::handle::BorrowedHandle;
use crate::sync::OnceLock;
use crate::collections::HashMap;

#[stable(feature = "popcorn_std", since = "1.88.0")]
pub fn get_handle<I: ProtocolTuple>(id: impl AsRef<OsStr>) -> Option<BorrowedHandle<'static, I>> {
    get_handle_untyped(id).and_then(|handle| handle.try_as::<I>())
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
pub fn get_handle_untyped(id: impl AsRef<OsStr>) -> Option<BorrowedHandle<'static>> {
    static HANDLE_MAP: OnceLock<HashMap<&'static OsStr, isize>> = OnceLock::new();
    
    HANDLE_MAP.get_or_init(|| HashMap::from_iter(crate::sys::handles()))
        .get(id.as_ref())
    /*crate::sys::handles().into_iter()
        .find(|(s, _)| s == id.as_ref())*/
        .map(|handle| RawHandle(*handle))
        .map(|handle| unsafe { BorrowedHandle::from_raw_handle(handle) })
}

