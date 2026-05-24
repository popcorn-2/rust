//! Popcorn-specific extensions to general I/O primitives.
//!
//! Just like raw pointers, raw Popcorn handles point to resources
//! with dynamic lifetimes, and they can dangle if they outlive their resources
//! or be forged if they're created from invalid values.
//!
//! This module provides three types for representing raw handles
//! with different ownership properties: raw, borrowed, and owned, which are
//! analogous to types used for representing pointers. These types reflect concepts of [I/O
//! safety][io-safety] on Popcorn.
//!
//! | Type                   | Analogous to |
//! | ---------------------- | ------------ |
//! | [`RawHandle`]          | `*mut _`     |
//! | [`BorrowedHandle<'a>`] | `&'a _`      |
//! | [`OwnedHandle`]        | `Box<_>`     |
//!
//! Like raw pointers, `RawHandle` values are primitive values.
//! And in new code, they should be considered unsafe to do I/O on (analogous
//! to dereferencing them).
//!
//! Like references, `BorrowedHandle`  values are tied to a
//! lifetime, to ensure that they don't outlive the resource they point to.
//! These are safe to use. `BorrowedHandle` values may be
//! used in APIs which provide safe access to any system call except for
//! `abi_v1::destroy()`, or any other call that would end the
//! dynamic lifetime of the resource without
//! ending the lifetime of the handle.
//!
//! `BorrowedHandle` values may be used in APIs which
//! provide safe access to `abi_v1::dup()` and
//! related functions, so types implementing `AsHandle`, `AsSocket`,
//! `From<OwnedHandle>` should not assume they always
//! have exclusive access to the underlying object.
//!
//! Like boxes, `OwnedHandle` values conceptually own the
//! resource they point to, and free (close) it when they are dropped.
//!
//! See the [`io` module docs][io-safety] for a general explanation of I/O safety.
//!
//! [`BorrowedHandle<'a>`]: crate::os::popcorn::io::BorrowedHandle
//! [io-safety]: crate::io#io-safety

mod raw_handle;
pub use raw_handle::{RawHandle, AsRawHandle, IntoRawHandle, FromRawHandle};

mod owned_handle;
pub use owned_handle::OwnedHandle;

mod borrowed_handle;
pub use borrowed_handle::{BorrowedHandle, AsHandle};

pub enum AnyProtocol {}

pub trait PopcornHandle: AsRawHandle/* + FromRawHandle*/ {
    type Protocols;
}

pub trait PopcornAsyncHandle: AsRawHandle/* + FromRawHandle*/ {
    type Protocols;

    fn wait_result(f: impl FnOnce(usize) -> crate::io::Result<u128>) -> impl Future<Output = crate::io::Result<u128>>;
}

#[stable(feature = "io_safety", since = "1.63.0")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandleNotFoundError(pub(crate) ());

#[stable(feature = "io_safety", since = "1.63.0")]
impl crate::fmt::Display for HandleNotFoundError {
    fn fmt(&self, fmt: &mut crate::fmt::Formatter<'_>) -> crate::fmt::Result {
        "A handle could not be found".fmt(fmt)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl crate::error::Error for HandleNotFoundError {}
