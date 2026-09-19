use crate::sync::atomic::{AtomicI32, Ordering};
use crate::ffi::CStr;

pub static ADDRESS_SPACE_HANDLE: AtomicI32 = AtomicI32::new(-1);
pub static MAIN_THREAD_HANDLE: AtomicI32 = AtomicI32::new(-1);
pub static STDIN_HANDLE: AtomicI32 = AtomicI32::new(-1);
pub static STDOUT_HANDLE: AtomicI32 = AtomicI32::new(-1);
pub static STDERR_HANDLE: AtomicI32 = AtomicI32::new(-1);

#[repr(C)]
pub struct ProcInfo {
	magic: [u8; 7],
	version: u8,
	// version 0 fields
	pub argc: core::ffi::c_int,
	pub argv: *const *const core::ffi::c_char,
	named_handles: *const NamedHandle,
	info_ty: usize,
	info_ptr: *const core::ffi::c_void,
}

#[repr(C)]
struct NamedHandle {
	name: *const core::ffi::c_char,
	num: i32,
}

#[used]
#[unsafe(link_section = ".init_array.0")]
static STARTUP_INIT_ARRAY: extern "C" fn(
    *const ProcInfo
) = {
    extern "C" fn init_wrapper(
        proc_info: *const ProcInfo,
    ) {
        unsafe { init_info(&*proc_info) };
    }
    init_wrapper
};

extern "C" fn init_info(proc_info: &ProcInfo) {
    // Locate handles for libstd
    let mut handle_ptr = proc_info.named_handles;
    loop {
        let handle = unsafe { handle_ptr.read() };
        if handle.name.is_null() { break; }

        let cstr = unsafe { CStr::from_ptr(handle.name) };
        if cstr == c"address_space.main" {
            ADDRESS_SPACE_HANDLE.store(handle.num, Ordering::Relaxed);
        } else if cstr == c"task.main" {
            MAIN_THREAD_HANDLE.store(handle.num, Ordering::Relaxed);
        } else if cstr == c"io.stdin" {
            STDIN_HANDLE.store(handle.num, Ordering::Relaxed);
        } else if cstr == c"io.stdout" {
            STDOUT_HANDLE.store(handle.num, Ordering::Relaxed);
        } else if cstr == c"io.stderr" {
            STDERR_HANDLE.store(handle.num, Ordering::Relaxed);
        }

        handle_ptr = unsafe { handle_ptr.add(1) };
    }

    unsafe { crate::sys::args::init(proc_info.argc as isize, proc_info.argv.cast()); }
}
