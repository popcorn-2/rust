//! Popcorn-specific extensions to primitives in the [`std::ffi`] module.
//!
//! # Examples
//!
//! ```
//! use std::ffi::OsString;
//! use std::os::unix::ffi::OsStringExt;
//!
//! let string = "foo".to_string();
//!
//! // OsStringExt::from_string
//! let os_string = OsString::from_string(string);
//! assert_eq!(os_string.to_str(), Some("foo"));
//!
//! // OsStringExt::into_string
//! let string = os_string.as_string();
//! assert_eq!(string, "foo");
//! ```
//!
//! ```
//! use std::ffi::OsStr;
//! use std::os::unix::ffi::OsStrExt;
//!
//! let str = "foo";
//!
//! // OsStrExt::from_str
//! let os_str = OsStr::from_str(bytes);
//! assert_eq!(os_str.to_str(), Some("foo"));
//!
//! // OsStrExt::as_bytes
//! let str = os_str.as_str();
//! assert_eq!(str, b"foo");
//! ```
//!
//! [`std::ffi`]: crate::ffi

#![stable(feature = "popcorn_std", since = "1.88.0")]

mod os_str;

#[stable(feature = "popcorn_std", since = "1.88.0")]
pub use self::os_str::{OsStrExt, OsStringExt};
