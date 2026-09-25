use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::thread::ThreadInit;
use crate::time::{Duration, Instant};
use crate::sync::atomic::Ordering;
use crate::sys::AsInner;

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096 * 4;

pub struct Thread {
    handle: i32,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new(stack_size: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        extern "C" fn popcorn_rust_startup(thread_arg: usize) {
            unsafe {
                let init = Box::from_raw(core::ptr::with_exposed_provenance_mut::<ThreadInit>(
                    thread_arg,
                ));
                let rust_start = init.init();
                if let Some(_name) = crate::thread::current().name() {
                    
                }

                rust_start();

                let _ = unsafe {
                    crate::sys::syscall!(
                        -2, // current thread pseudo-handle
                        2,
                        1,
                        @integer = [/* exit code */ 0],
                        @oob = [],
                    )
                };
            }
        }

        unsafe extern "C" {
            fn __rt_create_thread(entry: extern "C" fn(usize), entry_arg: usize, stack_ptr: usize, parent_handle: i32) -> isize;
        }

        let address_space_handle = crate::sys::ADDRESS_SPACE_HANDLE.load(Ordering::Relaxed);

        let stack_size = stack_size.div_ceil(4096) * 4096;

        let stack_ptr = unsafe {
                crate::sys::syscall!(
                    address_space_handle,
                    0,
                    1,
                    @integer = [stack_size, 0b101],
                    @oob = [],
                )
        };
        let stack_ptr = if stack_ptr < 0 { return Err(io::Error::from_raw_os_error((-stack_ptr) as i32)); }
            else { stack_ptr.cast_unsigned() };
        let thread_arg = Box::into_raw(init).expose_provenance();

        let thread_handle = unsafe {
            __rt_create_thread(popcorn_rust_startup, thread_arg, stack_ptr + stack_size, -2)
        };

        let thread_handle = if thread_handle < 0 { return Err(io::Error::from_raw_os_error((-thread_handle) as i32)); }
            else { thread_handle as i32 };
        Ok(Thread { handle: thread_handle })
    }

    pub fn join(self) {
        let _result = unsafe {
            crate::sys::syscall!(
                self.handle,
                2,
                5,
                @integer = [],
                @oob = [],
            )
        };
    }
}

pub fn yield_now() {
    let _result = unsafe {
        crate::sys::syscall!(
            -2, // current thread pseudo-handle
            2,
            6,
            @integer = [],
            @oob = [],
        )
    };
}
