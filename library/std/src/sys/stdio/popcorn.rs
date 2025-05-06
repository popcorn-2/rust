use crate::io::{self, Error};
use core_protocols::{
    handle::Handle,
    protocol::core::{
        io::Read,
        io::Write,
    }
};

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
        let res = Handle(0).read(buf).as_raw();
        if res < 0 {
            Err(Error::from_raw_os_error(-res))
        } else {
            Ok(res as usize)
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
        let res = Handle(1).write(buf).as_raw();
        if res < 0 {
            Err(Error::from_raw_os_error(-res))
        } else {
            Ok(res as usize)
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
        let res = Handle(2).write(buf).as_raw();
        if res < 0 {
            Err(Error::from_raw_os_error(-res))
        } else {
            Ok(res as usize)
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
