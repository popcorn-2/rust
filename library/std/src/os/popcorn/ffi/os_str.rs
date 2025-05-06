use crate::ffi::{OsStr, OsString};
use crate::mem;
use crate::sealed::Sealed;
use crate::sys::os_str::Buf;
use crate::sys::{AsInner, FromInner, IntoInner};

/// Platform-specific extensions to [`OsString`].
///
/// This trait is sealed: it cannot be implemented outside the standard library.
/// This is so that future additional methods are not breaking changes.
#[stable(feature = "popcorn_std", since = "1.88.0")]
pub trait OsStringExt: Sealed {
    /// Creates an [`OsString`] from a [`String`].
    ///
    /// See the module documentation for an example.
    #[stable(feature = "popcorn_std", since = "1.88.0")]
    fn from_string(string: String) -> Self;

    /// Yields the underlying [`String`] of this [`OsString`].
    ///
    /// See the module documentation for an example.
    #[stable(feature = "popcorn_std", since = "1.88.0")]
    fn into_string(self) -> String;
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl OsStringExt for OsString {
    #[inline]
    fn from_string(string: String) -> OsString {
        FromInner::from_inner(Buf { inner: string })
    }
    #[inline]
    fn into_string(self) -> String {
        self.into_inner().inner
    }
}

/// Platform-specific extensions to [`OsStr`].
///
/// This trait is sealed: it cannot be implemented outside the standard library.
/// This is so that future additional methods are not breaking changes.
#[stable(feature = "popcorn_std", since = "1.88.0")]
pub trait OsStrExt: Sealed {
    /// Creates an [`OsStr`] from a [`str`].
    ///
    /// See the module documentation for an example.
    #[stable(feature = "popcorn_std", since = "1.88.0")]
    fn from_str(slice: &str) -> &Self;

    /// Gets the underlying [`str`] view of the [`OsStr`] slice.
    ///
    /// See the module documentation for an example.
    #[stable(feature = "popcorn_std", since = "1.88.0")]
    fn as_str(&self) -> &str;
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl OsStrExt for OsStr {
    #[inline]
    fn from_str(slice: &str) -> &OsStr {
        unsafe { mem::transmute(slice) }
    }
    #[inline]
    fn as_str(&self) -> &str {
        &self.as_inner().inner
    }
}
