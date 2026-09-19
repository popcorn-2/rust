#![deny(unsafe_op_in_unsafe_fn)]

mod startup;

#[allow(unused)]
pub use startup::{ADDRESS_SPACE_HANDLE, MAIN_THREAD_HANDLE, STDIN_HANDLE, STDOUT_HANDLE, STDERR_HANDLE};

use crate::io;

pub fn unsupported<T>() -> io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> io::Error {
    io::const_error!(io::ErrorKind::Unsupported, "operation not supported on Popcorn yet")
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

#[cfg(target_arch = "x86_64")]
pub fn get_syscall_trampoline() -> *mut u8 {
    let ptr: *mut u8;
    unsafe { core::arch::asm!("rdgsbase {}", out(reg) ptr, options(nostack, nomem, preserves_flags)); }
    ptr
}
