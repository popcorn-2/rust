use core::io::BorrowedBuf;

use crate::io;
use crate::os::popcorn::proto::{io::Read, io::Write};
use crate::os::popcorn::io::{BorrowedHandle, RawHandle};

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
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buf = BorrowedBuf::from(buf);
        let handle = unsafe { BorrowedHandle::<'static, &dyn Read>::borrow_raw(RawHandle(0)) };
		match handle.read(buf.unfilled()) {
			Ok(res) => Ok(res),
			Err(e) if e.kind() == io::ErrorKind::InvalidInput => Ok(0), // no stdin attached
			e @ Err(_) => e
		}
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
		let handle = unsafe { BorrowedHandle::<'static, &dyn Write>::borrow_raw(RawHandle(1)) };
		match handle.write(buf) {
			Ok(res) => Ok(res),
			Err(e) if e.kind() == io::ErrorKind::InvalidInput => Ok(0), // no stdin attached
			e @ Err(_) => e
		}
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let handle = unsafe { BorrowedHandle::<'static, &dyn Write>::borrow_raw(RawHandle(2)) };
		match handle.write(buf) {
			Ok(res) => Ok(res),
			Err(e) if e.kind() == io::ErrorKind::InvalidInput => Ok(0), // no stdin attached
			e @ Err(_) => e
		}
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

pub fn is_ebadf(_err: &io::Error) -> bool {
    false
    // FIXME(popcorn): err.raw_os_error() == Some(EBADF as i32)
}

pub const STDIN_BUF_SIZE: usize = crate::sys::io::DEFAULT_BUF_SIZE;

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
