use core::sync::atomic::{AtomicIsize, AtomicPtr, Ordering};

use crate::ffi::{OsString, OsStr, CStr};
use crate::os::popcorn::ffi::OsStrExt;

static ARGC: AtomicIsize = AtomicIsize::new(0);
static ARGV: AtomicPtr<*const u8> = AtomicPtr::new(core::ptr::null_mut());
static ENVP: AtomicPtr<*const u8> = AtomicPtr::new(core::ptr::null_mut());
static AUXV: AtomicPtr<*const u8> = AtomicPtr::new(core::ptr::null_mut());
static HANDLEP: AtomicPtr<*const u8> = AtomicPtr::new(core::ptr::null_mut());

pub fn init(argc: isize, argv: *const *const u8) {
    ARGC.store(argc, Ordering::Relaxed);
    ARGV.store(argv.cast_mut(), Ordering::Relaxed);

    let mut ptr = unsafe { argv.offset(argc + 1) };
    ENVP.store(ptr.cast_mut(), Ordering::Relaxed);

    // skip to end of envp
    while unsafe { *ptr } != core::ptr::null_mut() { ptr = unsafe { ptr.add(1) }; }
    ptr = unsafe { ptr.add(1) }; // skip null termiator of envp
    AUXV.store(ptr.cast_mut(), Ordering::Relaxed);

    // skip to end of auxv
    while unsafe { *ptr } != core::ptr::null_mut() { ptr = unsafe { ptr.add(2) }; }
    ptr = unsafe { ptr.add(2) }; // skip AT_NULL and 0 a_val
    HANDLEP.store(ptr.cast_mut(), Ordering::Relaxed);
}

pub fn args() -> Vec<OsString> {
    let (argc, argv) = (ARGC.load(Ordering::Relaxed), ARGV.load(Ordering::Relaxed));
    let mut vec = Vec::with_capacity(argc as usize);

    for i in 0..argc {
        // SAFETY: `argv` is non-null if `argc` is positive, and it is
        // guaranteed to be at least as long as `argc`, so reading from it
        // should be safe.
        let ptr = unsafe { argv.offset(i).read() };

        if ptr.is_null() { break; }

        // SAFETY: Just checked that the pointer is not NULL, and on Popcorn,
        // arguments are guarunteed to be valid UTF-8
        let cstr = unsafe { CStr::from_ptr(ptr.cast()) };
        vec.push(<OsStr as OsStrExt>::from_str(unsafe { str::from_utf8_unchecked(cstr.to_bytes()) }).to_os_string());
    }

    vec
}

pub fn get_env() -> Vec<&'static OsStr> {
    let mut vec = vec![];

    let mut ptr = ENVP.load(Ordering::Relaxed);
    loop {
        let env_ptr = unsafe { ptr.read() };
        if env_ptr.is_null() { break; }

        // SAFETY: Just checked that the pointer is not NULL, and on Popcorn,
        // environment variables are guarunteed to be valid UTF-8
        let env_var = unsafe { CStr::from_ptr(env_ptr.cast()) };
        let env_var = <OsStr as OsStrExt>::from_str(unsafe { str::from_utf8_unchecked(env_var.to_bytes()) });

        vec.push(env_var);
        unsafe { ptr = ptr.offset(1) };
    }

    vec
}

pub fn handles() -> Vec<(&'static OsStr, isize)> {
    let mut vec = vec![];

    let mut ptr = HANDLEP.load(Ordering::Relaxed);
    loop {
        let handle_name = unsafe { ptr.read() };
        if handle_name.is_null() { break; }

        // SAFETY: Just checked that the pointer is not NULL, and on Popcorn,
        // handle names are guarunteed to be valid UTF-8
        let handle_name = unsafe { CStr::from_ptr(handle_name.cast()) };
        let handle_name = <OsStr as OsStrExt>::from_str(unsafe { str::from_utf8_unchecked(handle_name.to_bytes()) });

        let handle_id = unsafe { ptr.offset(1).read() }.addr() as isize;

        vec.push((handle_name, handle_id));
        unsafe { ptr = ptr.offset(2) };
    }

    vec
}
