use crate::io::{self, BorrowedCursor, IoSliceMut};
use core::sync::atomic::Ordering;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }

    #[inline]
    fn read_buf(&mut self, _cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn read_vectored(&mut self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Ok(0)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        false
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if !buf.is_empty() { Err(io::Error::READ_EXACT_EOF) } else { Ok(()) }
    }

    #[inline]
    fn read_buf_exact(&mut self, cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        if cursor.capacity() != 0 { Err(io::Error::READ_EXACT_EOF) } else { Ok(()) }
    }

    #[inline]
    fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> io::Result<usize> {
        Ok(0)
    }

    #[inline]
    fn read_to_string(&mut self, _buf: &mut String) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let trampoline = crate::sys::get_syscall_trampoline();
        let handle = crate::sys::STDOUT_HANDLE.load(Ordering::Relaxed);
        let mut count = 0usize;

        for chunk in buf.chunks(0x1000) {
            let result = unsafe {
                core::ptr::copy_nonoverlapping(
                    chunk.as_ptr(),
                    trampoline,
                    chunk.len(),
                );

                crate::sys::syscall!(
                    handle,
                    1,
                    1,
                    @integer = [],
                    @oob = [chunk.len()],
                )
            };

            if result < 0 { return Err(io::Error::from_raw_os_error((-result) as i32)); }
            else { count += result.cast_unsigned(); }
        }

        Ok(count)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let trampoline = crate::sys::get_syscall_trampoline();
        let handle = crate::sys::STDERR_HANDLE.load(Ordering::Relaxed);
        let mut count = 0usize;

        for chunk in buf.chunks(0x1000) {
            let result = unsafe {
                core::ptr::copy_nonoverlapping(
                    chunk.as_ptr(),
                    trampoline,
                    chunk.len(),
                );

                crate::sys::syscall!(
                    handle,
                    1,
                    1,
                    @integer = [],
                    @oob = [chunk.len()],
                )
            };

            if result < 0 { return Err(io::Error::from_raw_os_error((-result) as i32)); }
            else { count += result.cast_unsigned(); }
        }

        Ok(count)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(err: &io::Error) -> bool {
    err.raw_os_error() == Some(6)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
