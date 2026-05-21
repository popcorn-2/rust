use crate::{io, fs};
use super::{AsRawHandle, OwnedHandle, PopcornHandle, RawHandle, IntoRawHandle, FromRawHandle};
use core::marker::PhantomData;
use crate::os::popcorn::proto::{self, abi_v1::AbiV1};
use crate::fmt;
use crate::sys::AsInner;
use crate::os::popcorn::proto::ProtocolTuple;

pub struct BorrowedHandle<'handle, T = ()> {
    handle: RawHandle,
    _phantom: PhantomData<&'handle OwnedHandle<T>>,
}

impl<T> BorrowedHandle<'_, T> {
    pub const unsafe fn borrow_raw(handle: RawHandle) -> Self {
        Self {
            handle,
            _phantom: PhantomData,
        }
    }

    pub fn try_clone_to_owned(&self) -> io::Result<OwnedHandle<T>> {
        Ok(unsafe { OwnedHandle::from_raw_handle(self.handle.dup()?.into_raw_handle()) }) // fixme `dup` shouldn't default to ownedhandle probably
    }
}

impl<'a, T> BorrowedHandle<'a, T> {
    pub fn try_as<U: ProtocolTuple>(&self) -> Option<BorrowedHandle<'a, U>> {
        if self.has_protocols::<U>() {
            Some(self.force_protocol())
        } else {
            None
        }
    }

    // not unsafe as the kernel checks if the protocol is supported on every use, just
    // wastes time and sanity if this is abused
    // use `try_as` for a checked version
    pub(crate) fn force_protocol<U: ProtocolTuple>(&self) -> BorrowedHandle<'a, U> {
        unsafe { BorrowedHandle::borrow_raw(self.handle) }
    }
}

impl<T> fmt::Debug for BorrowedHandle<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BorrowedHandle::<{}>({})", crate::any::type_name::<T>(), &self.handle.0)
    }
}

impl<T> Clone for BorrowedHandle<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for BorrowedHandle<'_, T> {}

impl<T> AsRawHandle for BorrowedHandle<'_, T> {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

impl<T> PopcornHandle for BorrowedHandle<'_, T> {
    type Protocols = T;
}

/// A trait to borrow the handle from an underlying object.
#[stable(feature = "io_safety", since = "1.63.0")]
pub trait AsHandle<T> {
    /*
    /// Borrows the handle.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use std::os::windows::io::{AsHandle, BorrowedHandle};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// let borrowed_handle: BorrowedHandle<'_> = f.as_handle();
    /// # Ok::<(), io::Error>(())
    /// ```
    */
    #[stable(feature = "io_safety", since = "1.63.0")]
    fn as_handle(&self) -> BorrowedHandle<'_, T>;
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: AsHandle<U> + ?Sized, U> AsHandle<U> for &T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, U> {
        T::as_handle(self)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: AsHandle<U> + ?Sized, U> AsHandle<U> for &mut T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, U> {
        T::as_handle(self)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<&'static dyn proto::io::Read> for fs::File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Read> {
        self.as_inner().as_handle()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<&'static dyn proto::io::Write> for fs::File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Write> {
        self.as_inner().as_handle()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<(&'static dyn proto::io::Read, &'static dyn proto::io::Write)> for fs::File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, (&'static dyn proto::io::Read, &'static dyn proto::io::Write)> {
        self.as_inner().as_handle()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<(&'static dyn proto::io::Write, &'static dyn proto::io::Read)> for fs::File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, (&'static dyn proto::io::Write, &'static dyn proto::io::Read)> {
        self.as_inner().as_handle()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<&'static dyn proto::io::Read> for io::Stdin {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Read> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<'a> AsHandle<&'static dyn proto::io::Read> for io::StdinLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Read> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<&'static dyn proto::io::Write> for io::Stdout {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Write> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<'a> AsHandle<&'static dyn proto::io::Write> for io::StdoutLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Write> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<&'static dyn proto::io::Write> for io::Stderr {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Write> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<'a> AsHandle<&'static dyn proto::io::Write> for io::StderrLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, &'static dyn proto::io::Write> {
        unsafe { BorrowedHandle::borrow_raw(self.as_raw_handle()) }
    }
}
