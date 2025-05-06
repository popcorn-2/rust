#![stable(feature = "popcorn_std", since = "1.88.0")]

use core::marker::PhantomData;

use alloc_crate::{rc::Rc, sync::Arc};
use popcorn_abi::proto::core::{fs::File as AbiFile, io::{Read, Seek, Write}};

use crate::{fs::File, io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock}, sys_common::FromInner};
use crate::sys_common::AsInner;

#[stable(feature = "popcorn_std", since = "1.88.0")]
pub type OwnedHandle<I> = popcorn_abi::handle::Handle<I>;

use popcorn_abi::handle::{FromRawHandle, AsRawHandle, RawHandle};

#[stable(feature = "popcorn_std", since = "1.88.0")]
pub struct BorrowedHandle<'handle, I> {
    handle: RawHandle,
    _phantom: PhantomData<&'handle OwnedHandle<I>>,
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> Clone for BorrowedHandle<'_, I> {
    fn clone(&self) -> Self {
        Self { handle: self.handle, _phantom: PhantomData }
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> Copy for BorrowedHandle<'_, I> {}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> core::fmt::Debug for BorrowedHandle<'_, I> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "BorrowedHandle::<{}>({})", core::any::type_name::<I>(), self.handle.0)
	}
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I: popcorn_abi::proto::Protocol> popcorn_abi::proto::HasProtocol<I> for BorrowedHandle<'_, I> {}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> AsRawHandle for BorrowedHandle<'_, I> {
	fn as_raw_handle(&self) -> RawHandle {
		self.handle
	}
}

macro_rules! impl_is_terminal {
    ($T: ident @ $($t:ty),*$(,)?) => {$(
        #[unstable(feature = "sealed", issue = "none")]
        impl<$T> crate::sealed::Sealed for $t {}

        #[stable(feature = "is_terminal", since = "1.70.0")]
        impl<$T> crate::io::IsTerminal for $t {
            #[inline]
            fn is_terminal(&self) -> bool {
                crate::sys::io::is_terminal(self)
            }
        }
    )*}
}

impl_is_terminal!(I @ BorrowedHandle<'_, I>, OwnedHandle<I>);

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> BorrowedHandle<'_, I> {
    #[stable(feature = "popcorn_std", since = "1.88.0")]
    pub fn try_as<T: popcorn_abi::proto::Protocol>(&self) -> Option<BorrowedHandle<'_, T>> {
        if self.handle.has_protocol::<T>() { Some(unsafe { core::mem::transmute(self) }) }
        else { None }
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> FromRawHandle for BorrowedHandle<'_, I> {
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        Self { handle: raw, _phantom: PhantomData }
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
pub trait AsHandle<I> {
    #[stable(feature = "popcorn_std", since = "1.88.0")]
    fn as_handle(&self) -> BorrowedHandle<'_, I>;
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> AsHandle<I> for OwnedHandle<I> {
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        BorrowedHandle {
            handle: self.as_raw_handle(),
            _phantom: PhantomData,
        }
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I> AsHandle<I> for BorrowedHandle<'_, I> {
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        *self
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<(AbiFile, Seek)> for File {
    fn as_handle(&self) -> BorrowedHandle<'_, (AbiFile, Seek)> {
        self.as_inner().as_handle()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsRawHandle for File {
    fn as_raw_handle(&self) -> RawHandle {
        self.as_inner().as_handle().handle
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl FromRawHandle for File {
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        File::from_inner(crate::sys::fs::File::from_raw_handle(raw))
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<Read> for Stdin {
    fn as_handle(&self) -> BorrowedHandle<'_, Read> {
        crate::os::popcorn::env::get_handle("io.stdin").unwrap()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<Write> for Stdout {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stdout").unwrap()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<Write> for Stderr {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stderr").unwrap()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<Read> for StdinLock<'_> {
    fn as_handle(&self) -> BorrowedHandle<'_, Read> {
        crate::os::popcorn::env::get_handle("io.stdin").unwrap()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<Write> for StdoutLock<'_> {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stdout").unwrap()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl AsHandle<Write> for StderrLock<'_> {
    fn as_handle(&self) -> BorrowedHandle<'_, Write> {
        crate::os::popcorn::env::get_handle("io.stderr").unwrap()
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I, T: AsHandle<I> + ?Sized> AsHandle<I> for &T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(self)
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I, T: AsHandle<I> + ?Sized> AsHandle<I> for &mut T {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(self)
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I, T: AsHandle<I> + ?Sized> AsHandle<I> for Box<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(&*self)
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I, T: AsHandle<I> + ?Sized> AsHandle<I> for Rc<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(&*self)
    }
}

#[stable(feature = "popcorn_std", since = "1.88.0")]
impl<I, T: AsHandle<I> + ?Sized> AsHandle<I> for Arc<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_, I> {
        T::as_handle(&*self)
    }
}
