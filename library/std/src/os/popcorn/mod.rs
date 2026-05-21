//! Platform-specific extensions to `std` for popcorn.

#![unstable(feature = "popcorn_std", issue = "none")]
#![doc(cfg(target_os = "popcorn"))]

pub mod handle;
pub mod proto;
pub mod env;
pub mod process;
pub mod ffi;
pub mod sys;
