#![unstable(feature = "popcorn_std", issue = "none")]

use crate::{ffi::OsStr, os::popcorn::{handle::{FromRawHandle, RawHandle}, proto::ProtocolTuple}};
use super::handle::BorrowedHandle;
use crate::sync::OnceLock;
use crate::collections::HashMap;

pub fn get_handle<I: ProtocolTuple>(id: impl AsRef<OsStr>) -> Option<BorrowedHandle<'static, I>> {
    get_handle_untyped(id).and_then(|handle| handle.try_as::<I>())
}

pub fn get_handle_untyped(id: impl AsRef<OsStr>) -> Option<BorrowedHandle<'static>> {
    static HANDLE_MAP: OnceLock<HashMap<&'static OsStr, isize>> = OnceLock::new();
    
    HANDLE_MAP.get_or_init(|| HashMap::from_iter(crate::sys::handles()))
        .get(id.as_ref())
        .map(|handle| RawHandle(*handle))
        .map(|handle| unsafe { BorrowedHandle::from_raw_handle(handle) })
}

unsafe extern "C" {
    safe fn thrd_current() -> *const LibcTcb;
}

#[repr(C)]
struct LibcTcb {
	_self_pointer: *const LibcTcb,
	_dtv_size: usize,
	_dtv_pointers: *const *const core::ffi::c_void,
	tid: core::ffi::c_int,
}

pub fn current_thread_handle() -> BorrowedHandle<'static, crate::os::popcorn::proto::proc::Thread> {
    let id = unsafe { (*thrd_current()).tid } as isize;
    unsafe { BorrowedHandle::from_raw_handle(RawHandle(id)) }
}

