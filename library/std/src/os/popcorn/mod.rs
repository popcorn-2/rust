//! Platform-specific extensions to `std` for popcorn.

#![stable(feature = "popcorn_std", since = "1.88.0")]
#![doc(cfg(target_os = "popcorn"))]

pub mod handle;
pub mod proto;
pub mod env;
pub mod process;
pub mod ffi;
