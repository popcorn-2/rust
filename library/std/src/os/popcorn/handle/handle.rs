use core::{fmt::{Debug, Formatter}, marker::PhantomData};

use crate::{ffi::OsStr, fs::File};
use crate::sys::{FromInner, AsInner};
use crate::io::{Stdin, Stdout, Stderr, StdinLock, StdoutLock, StderrLock, self};
use alloc_crate::{rc::Rc, sync::Arc};
use super::super::proto::{ProtocolTuple, Protocol, io::Write, io::Read};
use core::mem::ManuallyDrop;

#[stable(feature = "io_safety", since = "1.63.0")]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct RawHandle(pub isize);

impl RawHandle {
    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn new(path: impl AsRef<OsStr>, uids: &[u128], args: *const u8) -> crate::io::Result<Self> {
        let path = path.as_ref().as_encoded_bytes();
        unsafe {
            crate::sys::os::syscall!(1u128<<96, path.as_ptr(), path.len(), uids.as_ptr(), uids.len(), args =>
                Ok(res) => {
                    return Ok(RawHandle(res as isize));
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }

    /*#[unstable(feature = "popcorn_protocol", issue = "none")]
    pub async fn new_async<T: ProtocolTuple + ?Sized>(path: impl AsRef<OsStr>, args: T::Ctor) -> crate::io::Result<Self> {
        let path = path.as_ref().as_encoded_bytes();
        let args = T::__private_abi_convert(args);
        unsafe {
            crate::sys::os::syscall_async!(1u128<<96, path.as_ptr(), path.len(), T::UID.as_ptr(), T::UID.len(), &args as *const _ =>
                Ok(res) => {
                    return Ok(RawHandle(res as isize));
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }*/

    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn new_from(path: impl AsRef<OsStr>, uid: u128, handle: RawHandle) -> crate::io::Result<Self> {
        let path = path.as_ref().as_encoded_bytes();
        unsafe {
            crate::sys::os::syscall!(5u128<<96, path.as_ptr(), path.len(), uid as u64, (uid >> 64) as u64, handle.0 =>
                Ok(res) => {
                    return Ok(RawHandle(res as isize));
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }

    /*#[unstable(feature = "popcorn_protocol", issue = "none")]
    pub async fn new_from_async<T: Protocol + ?Sized>(path: impl AsRef<OsStr>, handle: RawHandle) -> crate::io::Result<Self> {
        let path = path.as_ref().as_encoded_bytes();
        unsafe {
            crate::sys::os::syscall_async!(5u128<<96, path.as_ptr(), path.len(), T::UID as u64, (T::UID >> 64) as u64, handle.0 =>
                Ok(res) => {
                    return Ok(RawHandle(res as isize));
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }*/
    
    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn destroy(&self) {
        unsafe {
            crate::sys::os::syscall!(3u128<<96, self.0 =>
                Ok(res) => {
                    let _ = res;
                }
                Err(res) => {
                    let _ = res;
                }
            );
        }
    }

    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn has_protocol(&self, uids: &[u128]) -> crate::io::Result<bool> {
        unsafe {
            crate::sys::os::syscall!(2u128<<96, self.0, uids.as_ptr(), uids.len() =>
                Ok(res) => {
                    return Ok(res != 0);
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }

    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn dup(&self) -> crate::io::Result<Self> {
        unsafe {
            crate::sys::os::syscall!(4u128<<96, self.0 =>
                Ok(res) => {
                    return Ok(RawHandle(res as isize));
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }

    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn combine(&self, other: Self) -> crate::io::Result<()> {
        unsafe {
            crate::sys::os::syscall!(7u128<<96, self.0, other.0 =>
                Ok(_res) => {
                    return Ok(());
                }
                Err(res) => {
                    return Err(crate::io::Error::from_raw_os_error(res as isize));
                }
            );
        }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
#[repr(transparent)]
pub struct OwnedHandle<T: ?Sized = ()> {
    handle: RawHandle,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> OwnedHandle<T> {
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(OwnedHandle {
            handle: self.handle.dup()?,
            _phantom: PhantomData,
        })
    }

    #[unstable(feature = "popcorn_protocol", issue = "none")]
	pub fn type_erase(self) -> OwnedHandle {
		let this = ManuallyDrop::new(self);
		OwnedHandle {
			handle: this.handle,
			_phantom: PhantomData,
		}
	}
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> Debug for OwnedHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Handle::<{}>({})", core::any::type_name::<T>(), self.handle.0)
    }
}

impl<T: ProtocolTuple + ?Sized> OwnedHandle<T> {
    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn new(path: impl AsRef<OsStr>, args: T::Ctor) -> crate::io::Result<Self> {
        let args = T::__private_abi_convert(args);
        Ok(Self {
            handle: RawHandle::new(path, T::UID, (&args as *const T::__PrivateAbiCompat).cast())?,
            _phantom: PhantomData,
        })
    }

    /*#[unstable(feature = "popcorn_protocol", issue = "none")]
    pub async fn new_async(path: impl AsRef<OsStr>, args: T::Ctor) -> crate::io::Result<Self> {
        Ok(Self {
            handle: RawHandle::new_async::<T>(path, args).await?,
            _phantom: PhantomData,
        })
    }*/
}

impl<T: Protocol + ?Sized> OwnedHandle<T> {
    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn new_from<U: ProtocolTuple + ?Sized>(path: impl AsRef<OsStr>, handle: OwnedHandle<U>) -> crate::io::Result<Self> {
		let handle = ManuallyDrop::new(handle);
        Ok(Self {
            handle: RawHandle::new_from(path, T::UID, handle.as_raw_handle())?,
            _phantom: PhantomData,
        })
    }

    /*#[unstable(feature = "popcorn_protocol", issue = "none")]
    pub async fn new_from_async<U: ProtocolTuple + ?Sized>(path: impl AsRef<OsStr>, handle: OwnedHandle<U>) -> crate::io::Result<Self> {
		let handle = ManuallyDrop::new(handle);
        Ok(Self {
            handle: RawHandle::new_from_async::<T>(path, handle.as_raw_handle()).await?,
            _phantom: PhantomData,
        })
    }*/
}

impl<T: ?Sized> OwnedHandle<T> {
    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn try_as<U: ProtocolTuple + ?Sized>(&self) -> Option<BorrowedHandle<'_, U>> {
        if self.handle.has_protocol(U::UID).unwrap_or(false) { Some(BorrowedHandle { handle: self.handle, _phantom: PhantomData }) }
        else { None }
    }

    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn try_combine_with<U: ProtocolTuple + ?Sized>(&self, other: OwnedHandle<U>) -> Result<(), OwnedHandle<U>> {
		let other = ManuallyDrop::new(other);
        match self.handle.combine(other.handle) {
			Ok(_) => Ok(()),
			Err(_) => Err(ManuallyDrop::into_inner(other)),
		}
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> Drop for OwnedHandle<T> {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
#[repr(transparent)]
pub struct BorrowedHandle<'handle, T: ?Sized = ()> {
    handle: RawHandle,
    _phantom: PhantomData<&'handle OwnedHandle<T>>,
}

impl<'a, T: ?Sized> BorrowedHandle<'a, T> {
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedHandle<T>> {
        Ok(OwnedHandle {
            handle: self.handle.dup()?,
            _phantom: PhantomData,
        })
    }

    #[unstable(feature = "popcorn_protocol", issue = "none")]
    pub fn try_as<U: ProtocolTuple + ?Sized>(&self) -> Option<BorrowedHandle<'a, U>> {
        if self.handle.has_protocol(U::UID).unwrap_or(false) { Some(BorrowedHandle { handle: self.handle, _phantom: PhantomData }) }
        else { None }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> Clone for BorrowedHandle<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> Copy for BorrowedHandle<'_, T> {}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> Debug for BorrowedHandle<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "BorrowedHandle::<{}>({})", core::any::type_name::<T>(), self.handle.0)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
pub trait FromRawHandle {
    #[stable(feature = "io_safety", since = "1.63.0")]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self;
}

#[stable(feature = "io_safety", since = "1.63.0")]
pub trait AsRawHandle {
    #[stable(feature = "io_safety", since = "1.63.0")]
    fn as_raw_handle(&self) -> RawHandle;
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> FromRawHandle for OwnedHandle<T> {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self { handle, _phantom: PhantomData }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> AsRawHandle for OwnedHandle<T> {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> FromRawHandle for BorrowedHandle<'_, T> {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self { handle, _phantom: PhantomData }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> AsRawHandle for BorrowedHandle<'_, T> {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl FromRawHandle for File {
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self::from_inner(unsafe { crate::sys::fs::File::from_raw_handle(handle) })
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsRawHandle for File {
    fn as_raw_handle(&self) -> RawHandle {
        self.as_inner().as_raw_handle()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
pub trait AsHandle<T: ?Sized> {
    #[stable(feature = "io_safety", since = "1.63.0")]
    fn as_handle(&self) -> BorrowedHandle<'_, T>;
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> AsHandle<T> for OwnedHandle<T> {
    fn as_handle(&self) -> BorrowedHandle<'_, T> {
        BorrowedHandle { handle: self.handle, _phantom: PhantomData }
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: ?Sized> AsHandle<T> for BorrowedHandle<'_, T> {
    fn as_handle(&self) -> BorrowedHandle<'_, T> {
        *self
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<super::super::proto::fs::File> for File {
    fn as_handle(&self) -> BorrowedHandle<'_, super::super::proto::fs::File> {
        self.as_inner().as_handle()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl From<OwnedHandle<super::super::proto::fs::File>> for File {
    fn from(owned: OwnedHandle<super::super::proto::fs::File>) -> Self {
        Self::from_inner(FromInner::from_inner(owned))
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<T> From<OwnedHandle<T>> for crate::process::Stdio {
    fn from(owned: OwnedHandle<T>) -> Self {
        Self::from_inner(crate::sys::process::Stdio::FromOwned(owned.type_erase()))
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<Read> for Stdin {
    fn as_handle(&self) -> BorrowedHandle<'_, Read> {
        crate::os::popcorn::env::get_handle("io.stdin").unwrap()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<Write> for Stdout {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stdout").unwrap()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<Write> for Stderr {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stderr").unwrap()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<Read> for StdinLock<'_> {
    fn as_handle(&self) -> BorrowedHandle<'_, Read> {
        crate::os::popcorn::env::get_handle("io.stdin").unwrap()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<Write> for StdoutLock<'_> {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stdout").unwrap()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl AsHandle<Write> for StderrLock<'_> {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stderr").unwrap()
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<I, T: AsHandle<I> + ?Sized> AsHandle<I> for &T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(self)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<I: ?Sized, T: AsHandle<I> + ?Sized> AsHandle<I> for &mut T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(self)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<I: ?Sized, T: AsHandle<I> + ?Sized> AsHandle<I> for Box<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(&*self)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<I: ?Sized, T: AsHandle<I> + ?Sized> AsHandle<I> for Rc<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(&*self)
    }
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl<I: ?Sized, T: AsHandle<I> + ?Sized> AsHandle<I> for Arc<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(&*self)
    }
}

macro_rules! impl_is_terminal {
    ($T: ident @ $($t:ty),*$(,)?) => {$(
        #[unstable(feature = "sealed", issue = "none")]
        impl<$T: ?Sized> crate::sealed::Sealed for $t {}

        #[stable(feature = "is_terminal", since = "1.70.0")]
        impl<$T: ?Sized> crate::io::IsTerminal for $t {
            #[inline]
            fn is_terminal(&self) -> bool {
                crate::sys::io::is_terminal(self)
            }
        }
    )*}
}

impl_is_terminal!(I @ BorrowedHandle<'_, I>, OwnedHandle<I>);
