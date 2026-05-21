#![deny(unsafe_op_in_unsafe_fn)]

pub mod args;

mod common;
pub use common::*;
pub use args::{handles, get_env};
