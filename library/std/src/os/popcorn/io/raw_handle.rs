use crate::{io, fs};
use crate::sys::AsInner;

#[derive(Debug, Copy, Clone)]
pub struct RawHandle(pub isize);

impl AsRawHandle for RawHandle {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(self.0)
    }
}

impl IntoRawHandle for RawHandle {
    fn into_raw_handle(self) -> RawHandle {
        self
    }
}

impl FromRawHandle for RawHandle {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        handle
    }
}

impl super::PopcornHandle for RawHandle {
    type Protocols = super::AnyProtocol;
}

/// Extracts raw handles.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawHandle {
    /// Extracts the raw handle.
    ///
    /// This function is typically used to **borrow** an owned handle.
    /// When used in this way, this method does **not** pass ownership of the
    /// raw handle to the caller, and the handle is only guaranteed
    /// to be valid while the original object has not yet been destroyed.
    ///
    /// However, borrowing is not strictly required. See [`AsHandle::as_handle`]
    /// for an API which strictly borrows a handle.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_handle(&self) -> RawHandle;
}

/// A trait to express the ability to consume an object and acquire ownership of
/// its raw handle.
#[stable(feature = "into_raw_os", since = "1.4.0")]
pub trait IntoRawHandle {
    /// Consumes this object, returning the raw underlying handle.
    ///
    /// This function is typically used to **transfer ownership** of the underlying
    /// handle to the caller. When used in this way, callers are then the unique
    /// owners of the handle and must close it once it's no longer needed.
    ///
    /// However, transferring ownership is not strictly required. Use a
    /// `Into<OwnedHandle>::into` implementation for an API which strictly
    /// transfers ownership.
    #[must_use = "losing the raw handle may leak resources"]
    #[stable(feature = "into_raw_os", since = "1.4.0")]
    fn into_raw_handle(self) -> RawHandle;
}

/// Constructs I/O objects from raw handles.
#[stable(feature = "from_raw_os", since = "1.1.0")]
pub trait FromRawHandle {
    /// Constructs a new I/O object from the specified raw handle.
    ///
    /// This function is typically used to **consume ownership** of the handle
    /// given, passing responsibility for closing the handle to the returned
    /// object. When used in this way, the returned object
    /// will take responsibility for closing it when the object goes out of
    /// scope.
    ///
    /// However, consuming ownership is not strictly required. Use a
    /// `From<OwnedHandle>::from` implementation for an API which strictly
    /// consumes ownership.
    ///
    /// # Safety
    ///
    /// The `handle` passed in must be an [owned handle][io-safety]; in particular, it must be open.
    ///
    /// [io-safety]: io#io-safety
    #[stable(feature = "from_raw_os", since = "1.1.0")]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawHandle for fs::File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.as_inner().as_raw_handle()
    }
}

#[stable(feature = "asraw_stdio", since = "1.21.0")]
impl AsRawHandle for io::Stdin {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(0)
    }
}

#[stable(feature = "asraw_stdio", since = "1.21.0")]
impl AsRawHandle for io::Stdout {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(1)
    }
}

#[stable(feature = "asraw_stdio", since = "1.21.0")]
impl AsRawHandle for io::Stderr {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(2)
    }
}

#[stable(feature = "asraw_stdio_locks", since = "1.35.0")]
impl<'a> AsRawHandle for io::StdinLock<'a> {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(0)
    }
}

#[stable(feature = "asraw_stdio_locks", since = "1.35.0")]
impl<'a> AsRawHandle for io::StdoutLock<'a> {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(1)
    }
}

#[stable(feature = "asraw_stdio_locks", since = "1.35.0")]
impl<'a> AsRawHandle for io::StderrLock<'a> {
    fn as_raw_handle(&self) -> RawHandle {
        RawHandle(2)
    }
}
