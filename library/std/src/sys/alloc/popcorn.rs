use crate::alloc::{GlobalAlloc, Layout, System};
use super::{MIN_ALIGN, realloc_fallback};
use core::ffi::c_void;
use crate::ptr;

// todo(popcorn): remove dependency on libc - we don't need to worry about cross-lang malloc/free cause that's UB anyway
#[link(name = "c")]
unsafe extern "C" {
    fn aligned_alloc(__alignment: usize, __size: usize) -> *mut c_void;
    fn calloc(__count: usize, __size: usize) -> *mut c_void;
    fn free(__pointer: *mut c_void);
    fn malloc(__size: usize) -> *mut c_void;
    fn realloc(__pointer: *mut c_void, __size: usize) -> *mut c_void;
}

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            unsafe { malloc(layout.size()) as *mut u8 }
        } else {
            unsafe { aligned_alloc(layout.align(), layout.size()) as *mut u8 }
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            unsafe { calloc(layout.size(), 1) as *mut u8 }
        } else {
            let ptr = unsafe { self.alloc(layout) };
            if !ptr.is_null() {
                unsafe { ptr::write_bytes(ptr, 0, layout.size()) };
            }
            ptr
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr as *mut c_void) }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            unsafe { realloc(ptr as *mut c_void, new_size) as *mut u8 }
        } else {
            unsafe { realloc_fallback(self, ptr, layout, new_size) }
        }
    }
}
