// FIXME(static_mut_refs): use raw pointers instead of references
#![allow(static_mut_refs)]

use crate::alloc::Layout;
use crate::ptr;
use crate::sync::atomic::Ordering;

static mut DLMALLOC: dlmalloc::Dlmalloc<Popcorn> = dlmalloc::Dlmalloc::new_with_allocator(Popcorn);

struct Popcorn;

unsafe impl dlmalloc::Allocator for Popcorn {
    /// Allocs system resources
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let handle = crate::sys::ADDRESS_SPACE_HANDLE.load(Ordering::Relaxed);
        let result: isize;
        unsafe {
            core::arch::asm!(
                "push rbp",
                "push rbx",
                "syscall",
                "pop rbx",
                "pop rbp",
                in("eax") handle,
                in("r12") 1,
                in("rdi") size,
                in("rsi") 0b101,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("rdx") _,
                lateout("rsi") _,
                lateout("rdi") _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r14") _,
                lateout("r15") _,
                clobber_abi("sysv64"),
            );
        }

        if result < 0 { (ptr::null_mut(), 0, 0) }
        else { (ptr::with_exposed_provenance_mut(result.cast_unsigned()), size, 0) }
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        ptr::null_mut()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        let handle = crate::sys::ADDRESS_SPACE_HANDLE.load(Ordering::Relaxed);
        let result: isize;
        unsafe {
            core::arch::asm!(
                "push rbp",
                "push rbx",
                "syscall",
                "pop rbx",
                "pop rbp",
                in("eax") handle,
                in("r12") 2,
                in("rdi") ptr,
                in("rsi") size,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("rdx") _,
                lateout("rsi") _,
                lateout("rdi") _,
                lateout("r8") _,
                lateout("r9") _,
                lateout("r10") _,
                lateout("r11") _,
                lateout("r12") _,
                lateout("r13") _,
                lateout("r14") _,
                lateout("r15") _,
                clobber_abi("sysv64"),
            );
        }

        if result < 0 { false }
        else { true }
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        false
    }

    fn page_size(&self) -> usize {
        0x1000
    }
}

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    // SAFETY: DLMALLOC access is guaranteed to be safe because we are a single-threaded target, which
    // guarantees unique and non-reentrant access to the allocator. As such, no allocator lock is used.
    // Calling malloc() is safe because preconditions on this function match the trait method preconditions.
    unsafe { DLMALLOC.malloc(layout.size(), layout.align()) }
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    // SAFETY: DLMALLOC access is guaranteed to be safe because we are a single-threaded target, which
    // guarantees unique and non-reentrant access to the allocator. As such, no allocator lock is used.
    // Calling calloc() is safe because preconditions on this function match the trait method preconditions.
    unsafe { DLMALLOC.calloc(layout.size(), layout.align()) }
}

#[inline]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    // SAFETY: DLMALLOC access is guaranteed to be safe because we are a single-threaded target, which
    // guarantees unique and non-reentrant access to the allocator. As such, no allocator lock is used.
    // Calling free() is safe because preconditions on this function match the trait method preconditions.
    unsafe { DLMALLOC.free(ptr, layout.size(), layout.align()) }
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: DLMALLOC access is guaranteed to be safe because we are a single-threaded target, which
    // guarantees unique and non-reentrant access to the allocator. As such, no allocator lock is used.
    // Calling realloc() is safe because preconditions on this function match the trait method preconditions.
    unsafe { DLMALLOC.realloc(ptr, layout.size(), layout.align(), new_size) }
}
