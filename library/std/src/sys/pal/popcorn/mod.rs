#![deny(unsafe_op_in_unsafe_fn)]

pub mod os;
pub mod pipe;
pub mod thread;
pub mod time;

mod common;
pub use common::*;

pub type RawOsError = isize;

// This function is needed by the panic runtime. The symbol is named in
// pre-link args for the target specification, so keep that in sync.
#[cfg(not(test))]
#[unsafe(no_mangle)]
// NB. used by both libunwind and libpanic_abort
pub extern "C" fn __rust_abort() {
    unsafe { core::arch::asm!("ud2"); }
}
