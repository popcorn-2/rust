use core::cell::SyncUnsafeCell;

use crate::alloc::Layout;

struct SyncDlmalloc(dlmalloc::Dlmalloc);
unsafe impl Sync for SyncDlmalloc {}

static mut DLMALLOC: dlmalloc::Dlmalloc<Vexos> = dlmalloc::Dlmalloc::new_with_allocator(Vexos);

struct Popcorn;

unsafe impl dlmalloc::Allocator for Popcorn {
    fn alloc(&self, _size: usize) -> (*mut u8, usize, u32) {
        todo!()
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        todo!()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        todo!()
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        false
    }

    fn page_size(&self) -> usize {
        4096
    }
}

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
    // Calling malloc() is safe because preconditions on this function match the trait method preconditions.
    let _lock = lock::lock();
    unsafe { DLMALLOC.malloc(layout.size(), layout.align()) }
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
    // Calling calloc() is safe because preconditions on this function match the trait method preconditions.
    let _lock = lock::lock();
    unsafe { DLMALLOC.calloc(layout.size(), layout.align()) }
}

#[inline]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
    // Calling free() is safe because preconditions on this function match the trait method preconditions.
    let _lock = lock::lock();
    unsafe { DLMALLOC.free(ptr, layout.size(), layout.align()) }
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: DLMALLOC access is guaranteed to be safe because the lock gives us unique and non-reentrant access.
    // Calling realloc() is safe because preconditions on this function match the trait method preconditions.
    let _lock = lock::lock();
    unsafe { DLMALLOC.realloc(ptr, layout.size(), layout.align(), new_size) }
}

// FIXME: use proper mutex
mod lock {
    use crate::sync::atomic::Ordering::{Acquire, Release};
    use crate::sync::atomic::{Atomic, AtomicI32};

    static LOCKED: Atomic<i32> = AtomicI32::new(0);

    pub struct DropLock;

    pub fn lock() -> DropLock {
        loop {
            if LOCKED.swap(1, Acquire) == 0 {
                return DropLock;
            }
            crate::thread::yield_now();
        }
    }

    impl Drop for DropLock {
        fn drop(&mut self) {
            let r = LOCKED.swap(0, Release);
            debug_assert_eq!(r, 1);
        }
    }
}
