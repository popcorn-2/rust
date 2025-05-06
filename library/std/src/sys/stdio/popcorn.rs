use core::io::BorrowedBuf;

use crate::io;
use crate::os::popcorn::proto::{io::Read, io::ReadTr, io::Write, io::WriteTr};
use crate::os::popcorn::handle::{FromRawHandle, BorrowedHandle, RawHandle};

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
        let handle = unsafe { BorrowedHandle::<'static, Read>::from_raw_handle(RawHandle(0)) };
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
		let handle = unsafe { BorrowedHandle::<'static, Write>::from_raw_handle(RawHandle(1)) };
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
		let handle = unsafe { BorrowedHandle::<'static, Write>::from_raw_handle(RawHandle(2)) };
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
