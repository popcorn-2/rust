//! Popcorn-specific extensions to primitives in the [`std::ffi`] module.
//!
//! # Examples
//!
//! ```
//! #![feature(popcorn_std)]
//! 
//! use std::ffi::OsString;
//! use std::os::popcorn::ffi::OsStringExt;
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
//! #![feature(popcorn_std)]
//! 
//! use std::ffi::OsStr;
//! use std::os::popcorn::ffi::OsStrExt;
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

mod os_str;
pub use self::os_str::{OsStrExt, OsStringExt};
