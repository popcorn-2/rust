use crate::io;
use super::{AsRawHandle, IntoRawHandle, FromRawHandle, PopcornHandle, RawHandle, AsHandle, BorrowedHandle};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use crate::os::popcorn::proto::abi_v1::AbiV1;
use crate::fmt;
use crate::os::popcorn::proto::ProtocolTuple;

pub struct OwnedHandle<T = ()> {
    handle: RawHandle,
    _phantom: PhantomData<T>,
}

impl<T> OwnedHandle<T> {
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(OwnedHandle {
            handle: self.handle.dup()?.into_raw_handle(), // fixme
            _phantom: PhantomData,
        })
    }

    pub fn try_as<U: ProtocolTuple>(&self) -> Option<BorrowedHandle<'_, U>> {
        self.as_handle().try_as::<U>()
    }

    pub(crate) fn type_erase(self) -> OwnedHandle<()> {
        OwnedHandle {
            handle: self.into_raw_handle(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Drop for OwnedHandle<T> {
    fn drop(&mut self) {
        let _ = self.handle.destroy();
    }
}

impl<T> fmt::Debug for OwnedHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OwnedHandle::<{}>({})", crate::any::type_name::<T>(), &self.handle.0)
    }
}

impl<T> AsRawHandle for OwnedHandle<T> {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

impl<T> IntoRawHandle for OwnedHandle<T> {
    fn into_raw_handle(self) -> RawHandle {
        let this = ManuallyDrop::new(self);
        this.handle
    }
}

impl<T> FromRawHandle for OwnedHandle<T> {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self {
            handle,
            _phantom: PhantomData,
        }
    }
}

impl<T> AsHandle<T> for OwnedHandle<T> {
    fn as_handle(&self) -> BorrowedHandle<'_, T> {
        unsafe { BorrowedHandle::borrow_raw(self.handle) }
    }
}

impl<T> PopcornHandle for OwnedHandle<T> {
    type Protocols = T;
}
