#![unstable(feature = "std_internals", issue = "none")]

use crate::os::popcorn::handle::{AsRawHandle, FromRawHandle, OwnedHandle, BorrowedHandle};

pub trait IsHandle: AsRawHandle + FromRawHandle {
    type Handle<'a, T: 'a>;
}

impl<T> IsHandle for OwnedHandle<T> {
    type Handle<'a, U: 'a> = OwnedHandle<U>;
}

impl<T> IsHandle for BorrowedHandle<'_, T> {
    type Handle<'a, U: 'a> = BorrowedHandle<'a, U>;
}
