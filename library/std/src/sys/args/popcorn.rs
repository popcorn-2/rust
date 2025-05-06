#![forbid(unsafe_op_in_unsafe_fn)]

#[path = "common.rs"]
mod common;
pub use common::Args;

use crate::sys::pal;

/// Returns the command line arguments
pub fn args() -> Args {
    Args::new(pal::args::args())
}
