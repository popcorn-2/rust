use crate::ffi::OsStr;
use crate::io;
use crate::os::popcorn::handle::{BorrowedHandle, RawHandle, FromRawHandle};
use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicI32, Ordering};
use core::mem::ManuallyDrop;
use crate::os::popcorn::proto::{proc::ThreadTr, mem::Pager};
use crate::os::popcorn::handle::{AsRawHandle, OwnedHandle};
use crate::thread::ThreadInit;

#[repr(transparent)]
pub struct Thread {
    tcb: *mut Tcb,
}

#[repr(C)]
#[allow(dead_code)]
enum TcbThreadReturnValue {
	Pointer,
	Integer,
}

#[repr(C)]
union ThreadReturnValue {
    void_ptr: *mut core::ffi::c_void,
    int_val: core::ffi::c_int,
}

#[repr(C)]
struct AtforkHandler {
    prepare: extern "C" fn(),
    parent: extern "C" fn(),
    child: extern "C" fn(),

    next: *mut AtforkHandler,
    prev: *mut AtforkHandler,
}

#[repr(C)]
struct CleanupHandler {
    func: extern "C" fn(*mut core::ffi::c_void),
    arg: *mut core::ffi::c_void,

    next: *mut CleanupHandler,
    prev: *mut CleanupHandler,
}

#[repr(C)]
struct Tcb {
	self_pointer: *mut Tcb,
	dtv_size: usize,
	dtv_pointers: *mut *mut core::ffi::c_void,
	tid: core::ffi::c_int,
	did_exit: core::ffi::c_int,

	#[cfg(target_arch = "x86_64")] padding: [u8; 8],

	stack_canary: usize,
	cancel_bits: core::ffi::c_int,

	return_value: ThreadReturnValue,
	return_value_type: TcbThreadReturnValue,

	atfork_begin: *mut AtforkHandler,
	atfork_end: *mut AtforkHandler,

	cleanup_begin: *mut CleanupHandler,
	cleanup_end: *mut CleanupHandler,

	is_joinable: core::ffi::c_int,

    local_keys: *mut core::ffi::c_void,

	stack_size: usize,
	stack_addr: *mut core::ffi::c_void,
	guard_size: usize,
}

#[cfg_attr(target_feature = "crt-static", link(name = "librt.a", modifiers="+verbatim"))]
#[cfg_attr(not(target_feature = "crt-static"), link(name = "librt.so", modifiers="+verbatim"))]
#[cfg_attr(not(target_feature = "crt-static"), link(name = "ld.so", modifiers="+verbatim"))]
unsafe extern "C" {
    safe fn __rtld_allocateTcb() -> *mut Tcb;
    safe fn __mlibc_start_thread() -> !;
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 32 * 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack_size: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
		let data = Box::into_raw(init);

		let process_handle = crate::os::popcorn::env::current_thread_handle();
        let tcb = __rtld_allocateTcb();

        let stack_top = unsafe {
            process_handle.unstable_anon_alloc(stack_size)?
            	.byte_add(stack_size)
				.cast::<usize>()
        };

        extern "C" fn thread_start(data: *mut core::ffi::c_void) -> core::ffi::c_int {
            let init = unsafe { Box::from_raw(data as *mut ThreadInit) };
            let rust_start = init.init();
            rust_start();
            0
        }

        // mlibc::sys_stack_prepare()
        unsafe {
            stack_top.offset(-1).write(tcb as usize);
            stack_top.offset(-2).write(data as usize);
            stack_top.offset(-3).write(thread_start as *const () as usize);
        }

        unsafe {
            addr_of_mut!((*tcb).stack_size).write(stack_size);
            addr_of_mut!((*tcb).guard_size).write(0); // fixme
            addr_of_mut!((*tcb).return_value_type).write(TcbThreadReturnValue::Integer);
        }

        // mlibc::sys_clone()
		let thread_handle = ManuallyDrop::new(process_handle.spawn_thread(
			OsStr::new(""),
			unsafe { stack_top.offset(-3).cast() },
			__mlibc_start_thread,
		)?);

        unsafe {
            let tid = AtomicI32::from_ptr(addr_of_mut!((*tcb).tid));
            tid.store(thread_handle.as_raw_handle().0 as _, Ordering::Relaxed);

            // do futex wake
        }
        
        Ok(Thread {
            tcb,
        })
    }

    pub fn join(self) {
        todo!();
    }
}

pub fn yield_now() {
    let _ = crate::os::popcorn::env::current_thread_handle()
            .yield_now();
}
