use crate::ptr::null;
use crate::sync::atomic::{Atomic, Ordering};
use crate::time::Duration;

/// An atomic for use as a futex that is at least 32-bits but may be larger
pub type Futex = Atomic<Primitive>;
/// Must be the underlying type of Futex
pub type Primitive = u32;

/// An atomic for use as a futex that is at least 8-bits but may be larger.
pub type SmallFutex = Atomic<SmallPrimitive>;
/// Must be the underlying type of SmallFutex
pub type SmallPrimitive = u32;

/// Waits for a `futex_wake` operation to wake us.
///
/// Returns false on timeout, and true in all other cases.
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    if timeout.is_some() { panic!("timeout not supported on popcorn"); }

    let result = unsafe {
        crate::sys::syscall!(
            -2, // current thread pseudo-handle
            2,
            2,
            @integer = [core::ptr::from_ref(futex).addr(), expected as usize],
            @oob = [],
        )
    };

    if result < 0 && result != -18 /* Error::Again */ {
        let error = crate::io::Error::from_raw_os_error(-result as i32);
        panic!("unexpected error in futex_wait:\n{error}");
    }

    true
}

/// Wakes up one thread that's blocked on `futex_wait` on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    let result = unsafe {
        crate::sys::syscall!(
            -2, // current thread pseudo-handle
            2,
            3,
            @integer = [core::ptr::from_ref(futex).addr(), 1],
            @oob = [],
        )
    };

    result > 0
}

/// Wakes up all threads that are waiting on `futex_wait` on this futex.
pub fn futex_wake_all(futex: &Atomic<u32>) {
    let _ = unsafe {
        crate::sys::syscall!(
            -2, // current thread pseudo-handle
            2,
            3,
            @integer = [core::ptr::from_ref(futex).addr(), usize::MAX],
            @oob = [],
        )
    };
}
