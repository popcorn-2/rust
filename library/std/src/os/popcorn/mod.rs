//! Platform-specific extensions to `std` for popcorn.

#![unstable(feature = "popcorn_std", issue = "none")]
#![doc(cfg(target_os = "popcorn"))]

pub mod io;
pub mod proto;
pub mod env;
pub mod process;
pub mod ffi;
pub mod sys;
pub mod prelude;
pub mod fs;
