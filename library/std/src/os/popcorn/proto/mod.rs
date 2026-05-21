#![unstable(feature = "popcorn_protocol", issue = "none")]

mod has_protocol;
pub use has_protocol::HasProtocol;

pub trait Protocol {
	const UID: u128;
}

#[doc(hidden)]
pub trait ProtocolTuple: crate::sealed::Sealed {
    const UIDs: &'static [u128];
}

/*#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum Error {
	InvalidPointer,
	InvalidUtf8,
	UnsupportedProtocol,
	UnknownProtocol,
	EndpointNotFound,
	NameInUse,
	InvalidHandle,
	Overflow,
	InvalidName,
	DeadServer,
	InProgress,
	Invalid,
	AllocationFailure,
}

impl From<crate::io::Error> for Error {
	fn from(val: crate::io::Error) -> Error {
		let raw = val.raw_os_error().unwrap_or(11);
		if raw > 12 { Error::Invalid }
		else { unsafe { core::mem::transmute(raw as u16) } }
	}
}*/

macro_rules! protocol_tuple {
    ($T:ident) => {
        protocol_tuple!(@ #[cfg_attr(doc, doc(fake_variadic))] $T);
    };
    ($($T:ident)*) => {
        protocol_tuple!(@ #[cfg_attr(doc, doc(hidden))] $($T)*);
    };
    (@ $(#[$attr:meta])* $($T:ident)*) => {
        impl<$($T: Protocol),*> crate::sealed::Sealed for ($($T),*,) {}

        $(#[$attr])*
        impl<$($T: Protocol),*> ProtocolTuple for ($($T),*,) {
            const UIDs: &'static [u128] = &[
                $(<$T as Protocol>::UID),*
            ];
        }
    };
}

impl<T> crate::sealed::Sealed for T where T: Protocol {}
impl<T> ProtocolTuple for T where T: Protocol {
    const UIDs: &'static [u128] = &[<T as Protocol>::UID];
}

protocol_tuple!(T);
protocol_tuple!(T U);
protocol_tuple!(T U V);
protocol_tuple!(T U V W);
protocol_tuple!(T U V W X);

/*

impl Proto for HasProtocol<T> + PopcornHandle { // or something
    fn normal(&self) -> ... {
        ...
        std::os::popcorn::sys::syscall!(...)
        ...
    }
    fn destructive(self) -> ... where Self: IntoRawHandle { ... }
}

impl Proto for HasProtocol<T> + PopcornAsyncHandle { // or something
    async fn normal(&self) -> ... {
        ...
        let res = Self::wait_result(|key| std::os::popcorn::sys::syscall_async!(...)).await;
        ...
    }
    fn destructive(self) -> ... where Self: IntoRawHandle { ... }
}

*/

#[allow_internal_unstable(trivial_bounds)]
pub macro protocol {
    () => {},
    ($vis:vis unsafe protocol ($name_sync:ident, $name_async:ident) = $uid:literal {
        $(fn $fn_name:ident @ $fn_num:literal $(<$($generic_ident:ident $(: $generic_bound:path)?),* $(,)?>)? (&self $($args:tt)*) -> std::io::Result<$ret:ty> {
            args => [
                $($arg_expr:expr),* $(,)?
            ];

            $ret_ident:pat => $ret_expr:expr;
        })*
    } $($rest:tt)*) => {
        $vis trait $name_sync {
            #![allow(patterns_in_fns_without_body)]
            $(
                fn $fn_name $(< $($generic_ident $(: $generic_bound)?),* >)? (&self $($args)*) -> $crate::io::Result<$ret> where Self: Sized;
            )*
        }

        impl $crate::os::popcorn::proto::Protocol for &dyn $name_sync {
            const UID: u128 = $uid;
        }

        $vis trait $name_async {
            #![allow(patterns_in_fns_without_body)]
            $(
                fn $fn_name $(< $($generic_ident $(: $generic_bound)?),* >)? (&self $($args)*) -> impl ::core::future::Future<Output = $crate::io::Result<$ret>> where Self: Sized;
            )*
        }

        impl $crate::os::popcorn::proto::Protocol for &dyn $name_async {
            const UID: u128 = $uid;
        }

        impl<'a, T: $crate::os::popcorn::io::PopcornHandle<Protocols: $crate::os::popcorn::proto::HasProtocol<&'a dyn $name_sync>>> $name_sync for T {
            $(
                fn $fn_name $(< $($generic_ident $(: $generic_bound)?),* >)? (&self $($args)*) -> $crate::io::Result<$ret> where Self: Sized {
                    // we do this to keep argument evaluation outside the unsafe block
                    #[allow(unused)] // in the case of zero args
                    let args = ($($arg_expr, )*);

                    unsafe {
                        $crate::os::popcorn::sys::syscall!(
                            (($fn_num as u128) << 96 | ($uid as u128)),
                            self.as_raw_handle()
                            $(, args . ${index()} ${ignore($arg_expr)})*
                        )
                    }.map(|$ret_ident| $ret_expr)
                     .map_err(|e| $crate::io::Error::from_raw_os_error(e as isize))
                }
            )*
        }

        /*impl<T: $crate::os::popcorn::io::PopcornAsyncHandle<Protocols: $crate::os::popcorn::proto::HasProtocol<$name_async>>> $name_async for T {
            $(
                fn $fn_name $(< $($generic_ident $(: $generic_bound)?),* >)? (&self, $($args)*) -> impl ::core::future::Future<Output = $crate::io::Result<$ret>> {
                    T::wait_result(|key|
                        unsafe {
                            $crate::syscall_async!(
                                (($fn_num as u128) << 96 | ($uid as u128)),
                                self.as_raw_handle(),
                                $($arg_expr),*
                            )
                        }.map(|$ret_ident| $ret_expr)
                        .map_err(|e| $crate::io::Error::from_raw_os_error(e as isize))
                    )
                }
            )*
        }*/

        $crate::os::popcorn::proto::protocol!($($rest)*);
    }
}

pub mod abi_v1 {
    use crate::os::popcorn::io::{OwnedHandle, PopcornHandle, IntoRawHandle};
    use crate::os::popcorn::sys::syscall;

    pub trait AbiV1 {
        fn has_protocols<T: super::ProtocolTuple>(&self) -> bool where Self: Sized;
        fn destroy(self) -> crate::io::Result<()> where Self: IntoRawHandle + Sized;
        fn dup(&self) -> crate::io::Result<OwnedHandle> where Self: Sized; // fixme `dup` shouldn't default to ownedhandle probably and same for other handle returning methods
    }

    impl super::Protocol for &dyn AbiV1 {
        const UID: u128 = 0;
    }

    impl<'a, T: PopcornHandle<Protocols = U>, U> AbiV1 for T {
        fn has_protocols<V: super::ProtocolTuple>(&self) -> bool where Self: Sized {
            let uids = V::UIDs;
            unsafe {
                syscall!(
                    2u128 << 96,
                    self.as_raw_handle(),
                    uids.as_ptr() as usize,
                    uids.len()
                )
            }.map(|v| v == 1).unwrap_or(false)
        }

        fn destroy(self) -> crate::io::Result<()> where Self: IntoRawHandle + Sized {
            unsafe {
                syscall!(
                    3u128 << 96,
                    self.into_raw_handle()
                )
            }.map(|_| ())
             .map_err(|e| crate::io::Error::from_raw_os_error(e as isize))
        }

        fn dup(&self) -> crate::io::Result<OwnedHandle> where Self: Sized {
            todo!()
        }
    }
}

pub mod fs {
    use super::ProtocolTuple;
    use crate::path::Path;
    use crate::convert::AsRef;
    use crate::os::popcorn::io::{OwnedHandle, RawHandle, FromRawHandle};

    super::protocol! {
        pub unsafe protocol (Dir, AsyncDir) = 1 {
            fn open_file@1<T: ProtocolTuple, P: AsRef<Path>>(&self, path: P, create: u8, append: bool, truncate: bool) -> std::io::Result<OwnedHandle<T>> {
                args => [
                    {
                        // FIXME(popcorn): jesus fuck this is so hacky but i can't think of a better way at
                        // 2am that doesn't invlove changing the abi to support more args
                        // the kernel will have *fun* supporting this cleanly
                        let ptr = T::UIDs.as_ptr() as usize;
                        assert!(create <= 2, "invalid value for `create` argument");
                        assert!(ptr & 0b1111 == 0, "invalid alignment for u128 array (UIDs)");
                        
                        ptr | (create as usize) | (append as usize) << 2 | (truncate as usize) << 3
                    },
                    T::UIDs.len(),
                    path.as_ref().as_os_str().as_encoded_bytes().as_ptr() as usize,
                    path.as_ref().as_os_str().as_encoded_bytes().len(),
                ];

                ret => unsafe { OwnedHandle::from_raw_handle(RawHandle(ret as isize)) };
            }
        }
    }
}

pub mod io {
    use core::io::BorrowedCursor;
    
    super::protocol! {
        pub unsafe protocol (Read, AsyncRead) = 2 {
            fn read@1(&self, mut buf: BorrowedCursor<'_>) -> std::io::Result<usize> {
                args => [
                    unsafe { buf.as_mut() }.as_mut_ptr() as usize,
                    buf.capacity(),
                ];

                ret => {
                    let ret = core::cmp::min(ret as usize, buf.capacity());
                    unsafe { buf.advance(ret); }
                    ret as usize
                };
            }
        }

        pub unsafe protocol (Write, AsyncWrite) = 3 {
            fn write@1(&self, buf: &[u8]) -> std::io::Result<usize> {
                args => [
                    buf.as_ptr() as usize,
                    buf.len(),
                ];

                ret => ret as usize;
            }
        }

        pub unsafe protocol (Seek, AsyncSeek) = 4 {}

        pub unsafe protocol (Terminal, AsyncTerminal) = 5 {}
    }
}

pub mod proc {
	use crate::ffi::OsStr;
	use crate::os::popcorn::io::{OwnedHandle, RawHandle, FromRawHandle, IntoRawHandle};

    super::protocol! {
        pub unsafe protocol (Builder, AsyncBuilder) = 9 {
            /*fn spawn@1(self) -> std::io::Result<OwnedHandle<Thread>> {
                args => [];

                ret => OwnedHandle::from_raw_handle(RawHandle(ret as isize));
            }*/

			fn add_handle@2(&self, name: &OsStr, handle: impl IntoRawHandle) -> std::io::Result<()> {
                args => [
                    name.as_encoded_bytes().as_ptr() as usize,
                    name.as_encoded_bytes().len(),
                    handle.into_raw_handle().0 as usize,
                ];

                _ => ();
            }

			fn add_env_var@3(&self, value: &OsStr) -> std::io::Result<()> {
                args => [
                    value.as_encoded_bytes().as_ptr() as usize,
                    value.as_encoded_bytes().len(),
                ];

                _ => ();
            }

			fn add_arg@4(&self, value: &OsStr) -> std::io::Result<()> {
                args => [
                    value.as_encoded_bytes().as_ptr() as usize,
                    value.as_encoded_bytes().len(),
                ];

                _ => ();
            }
        }

        pub unsafe protocol (Thread, AsyncThread) = 0xA {
			fn spawn_thread@4(&self, name: &OsStr, stack_top: *mut u8, entry: extern "C" fn() -> !) -> std::io::Result<OwnedHandle<&'static dyn Thread>> {
                args => [
                    name.as_encoded_bytes().as_ptr() as usize,
                    name.as_encoded_bytes().len(),
                    stack_top as usize,
                    entry as *mut () as usize,
                ];

                ret => unsafe { OwnedHandle::from_raw_handle(RawHandle(ret as isize)) };
            }

			fn yield_now@5(&self) -> std::io::Result<()> {
                args => [];
                _ret => ();
			}

			fn unstable_mmio_alloc@6(&self, physical_addr: usize, size: usize) -> std::io::Result<*mut u8> {
                args => [physical_addr, size];
                ret => core::ptr::with_exposed_provenance_mut(ret as usize);
            }

			fn exit@7(&self, code: isize) -> std::io::Result<()> {
                args => [code as usize];
                _ => ();
            }

			fn join@8(&self) -> std::io::Result<isize> {
                args => [];
                ret => ret as isize;
            }

			fn map_vmo@9(&self, vmo: impl IntoRawHandle, addr: *mut u8, length: usize, offset: usize) -> std::io::Result<*mut u8> {
                args => [
                    vmo.into_raw_handle().0 as usize,
                    addr as usize,
                    length,
                    offset,
                ];
                ret => core::ptr::with_exposed_provenance_mut(ret as usize);
            }
        }
    }
}

/*
pub macro protocol_old {
    () => {},
    (pub protocol $name:ident = $uid:literal {
        ctor => {
            $($ctor_arg:ident : $ctor_ty:ty),* $(,)?
        }

        $(fn ~$fn_name_d:ident($this_d:ident $(, $fn_arg_d:ident: $fn_ty_d:ty)* $(,)?) $(-> $fn_ret_d:ty)? $f_d:block)*
        $(fn $fn_name:ident $(<$($gen_ident:ident $(: $gen_bound:path)?),* $(,)?>)? (&$this:ident $(, $fn_arg:ident: $fn_ty:ty)* $(,)?) $(-> $fn_ret:ty)? $f:block)*
    } $($rest:tt)*) => {
        #[repr(C)]
        pub struct $name {
            $(pub $ctor_arg : $ctor_ty),*
        }

        impl $crate::os::popcorn::proto::Protocol for $name {
            type Ctor = $name;
            const UID: u128 = $uid;
        }

        pub trait ${concat($name, Tr)} {
            $(fn $fn_name_d($this_d $(, $fn_arg_d: $fn_ty_d)*) $(-> $crate::io::Result<$fn_ret_d>)? where Self: Sized;)*
            $(fn $fn_name $(<$($gen_ident $(: $gen_bound)?),*>)? (&$this $(, $fn_arg: $fn_ty)*) $(-> $crate::io::Result<$fn_ret>)? where Self: Sized;)*
        }

        /*pub trait ${concat(Async, $name, Tr)} {
            $(async fn $fn_name_d($this_d $(, $fn_arg_d: $fn_ty_d)*) $(-> $crate::io::Result<$fn_ret_d>)?;)*
            $(async fn $fn_name $(<$($gen_ident $(: $gen_bound)?),*>)? (&$this $(, $fn_arg: $fn_ty)*) $(-> $crate::io::Result<$fn_ret>)?;)*
        }*/

        impl<T: $crate::os::popcorn::proto::HasProtocol<$name>> ${concat($name, Tr)} for $crate::os::popcorn::handle::OwnedHandle<T> {
            $(fn $fn_name_d($this_d $(, $fn_arg_d: $fn_ty_d)*) $(-> $crate::io::Result<$fn_ret_d>)? where Self: Sized { use $crate::sys::os::syscall as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f_d })*
            $(fn $fn_name $(<$($gen_ident $(: $gen_bound)?),*>)? (&$this $(, $fn_arg: $fn_ty)*) $(-> $crate::io::Result<$fn_ret>)? where Self: Sized { use $crate::sys::os::syscall as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f })*
        }
        impl<T: $crate::os::popcorn::proto::HasProtocol<$name>> ${concat($name, Tr)} for $crate::os::popcorn::handle::BorrowedHandle<'_, T> {
            $(fn $fn_name_d($this_d $(, $fn_arg_d: $fn_ty_d)*) $(-> $crate::io::Result<$fn_ret_d>)? where Self: Sized { use $crate::sys::os::syscall as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f_d })*
            $(fn $fn_name $(<$($gen_ident $(: $gen_bound)?),*>)? (&$this $(, $fn_arg: $fn_ty)*) $(-> $crate::io::Result<$fn_ret>)? where Self: Sized { use $crate::sys::os::syscall as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f })*
        }

        /*impl<T: $crate::os::popcorn::proto::HasProtocol<$name>> ${concat(Async, $name, Tr)} for $crate::os::popcorn::handle::OwnedHandle<T> {
            $(async fn $fn_name_d($this_d $(, $fn_arg_d: $fn_ty_d)*) $(-> $crate::io::Result<$fn_ret_d>)? { use $crate::sys::os::syscall_async as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f_d })*
            $(async fn $fn_name $(<$($gen_ident $(: $gen_bound)?),*>)? (&$this $(, $fn_arg: $fn_ty)*) $(-> $crate::io::Result<$fn_ret>)? { use $crate::sys::os::syscall as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f })*
        }
        impl<T: $crate::os::popcorn::proto::HasProtocol<$name>> ${concat(Async, $name, Tr)} for $crate::os::popcorn::handle::BorrowedHandle<'_, T> {
            $(async fn $fn_name_d($this_d $(, $fn_arg_d: $fn_ty_d)*) $(-> $crate::io::Result<$fn_ret_d>)? { use $crate::sys::os::syscall_async as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f_d })*
            $(async fn $fn_name $(<$($gen_ident $(: $gen_bound)?),*>)? (&$this $(, $fn_arg: $fn_ty)*) $(-> $crate::io::Result<$fn_ret>)? { use $crate::sys::os::syscall as syscall; #[allow(dead_code)] const UID: u128 = $uid; $f })*
        }*/

        $crate::os::popcorn::proto::protocol!($($rest)*);
    }
}

pub mod abi_v1 {

}*/

/*pub mod fs {
    super::protocol! {
        pub protocol File = 1 {
            ctor => {
                create: usize,
                append: bool,
                truncate: bool,
            }
        }
    }
}

pub mod io {
    use core::io::BorrowedCursor;
    use crate::os::popcorn::handle::AsRawHandle;
    
    super::protocol! {
        pub protocol Read = 2 {
            ctor => {}
            fn read(&self, buf: BorrowedCursor<'_>) -> usize {
                let mut buf = buf;
                let ptr = unsafe { buf.as_mut() }.as_mut_ptr();
                let cap = buf.capacity();
                unsafe {
                    syscall!(1u128<<96 | UID, self.as_raw_handle().0, ptr, cap =>
                        Ok(res) => {
                            let res = core::cmp::min(res as usize, cap);
                            buf.advance(res as usize);
                            return Ok(res as usize);
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }
        }

        pub protocol Write = 3 {
            ctor => {}
            fn write(&self, buf: &[u8]) -> usize {
                unsafe {
                    syscall!(1u128<<96 | UID, self.as_raw_handle().0, buf.as_ptr(), buf.len() =>
                        Ok(res) => {
                            return Ok(res as usize);
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }
        }

        pub protocol Seek = 4 {
            ctor => {}
        }

        pub protocol Terminal = 5 {
            ctor => {}
        }
    }
}

pub mod proc {
	use crate::ffi::OsStr;
	use crate::os::popcorn::handle::{OwnedHandle, RawHandle, AsRawHandle, FromRawHandle};

    super::protocol! {
        pub protocol Builder = 9 {
            ctor => {}

            fn ~spawn(self) -> OwnedHandle<Thread> {
				let this = core::mem::ManuallyDrop::new(self);
				unsafe {
                    syscall!(1u128<<96 | UID, this.as_raw_handle().0 =>
                        Ok(res) => {
                            return Ok(OwnedHandle::from_raw_handle(RawHandle(res as isize)));
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			/*fn unstable_write_memory(&self, buf: &[u8], at: *const u8) -> () {
                unsafe {
                    syscall!(2u128<<96 | UID, self.as_raw_handle().0, buf.as_ptr(), buf.len(), at =>
                        Ok(_res) => {
                            return Ok(());
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }*/

			fn add_handle(&self, name: &OsStr, handle: isize) -> () {
                unsafe {
                    syscall!(2u128<<96 | UID, self.as_raw_handle().0, name.as_encoded_bytes().as_ptr(), name.as_encoded_bytes().len(), handle =>
                        Ok(_res) => {
                            return Ok(());
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			fn add_env_var(&self, value: &OsStr) -> () {
                unsafe {
                    syscall!(3u128<<96 | UID, self.as_raw_handle().0, value.as_encoded_bytes().as_ptr(), value.as_encoded_bytes().len() =>
                        Ok(_res) => {
                            return Ok(());
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			fn add_arg(&self, value: &OsStr) -> () {
                unsafe {
                    syscall!(4u128<<96 | UID, self.as_raw_handle().0, value.as_encoded_bytes().as_ptr(), value.as_encoded_bytes().len() =>
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

        pub protocol Thread = 0xA {
            ctor => {}

			fn spawn_thread(&self, name: &OsStr, stack_top: *mut u8, entry: extern "C" fn() -> !) -> OwnedHandle<Thread> {
                unsafe {
                    syscall!(4u128<<96 | UID, self.as_raw_handle().0, name.as_encoded_bytes().as_ptr(), name.as_encoded_bytes().len(), stack_top, entry =>
                        Ok(res) => {
                            return Ok(OwnedHandle::from_raw_handle(RawHandle(res as isize)));
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			fn yield_now(&self) -> () {
				unsafe {
                    syscall!(5u128<<96 | UID, self.as_raw_handle().0 =>
                        Ok(_res) => {
                            return Ok(());
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
			}

			fn unstable_mmio_alloc(&self, physical_addr: usize, size: usize) -> *mut u8 {
                unsafe {
                    syscall!(6u128<<96 | UID, self.as_raw_handle().0, physical_addr, size =>
                        Ok(res) => {
                            return Ok(core::ptr::with_exposed_provenance_mut(res as usize));
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			fn exit(&self, code: isize) -> () {
                unsafe {
                    syscall!(7u128<<96 | UID, self.as_raw_handle().0, code =>
                        Ok(_res) => {
                            return Ok(());
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			fn join(&self) -> isize {
                unsafe {
                    syscall!(8u128<<96 | UID, self.as_raw_handle().0 =>
                        Ok(res) => {
                            return Ok(res as isize);
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

			fn map_vmo(&self, vmo: RawHandle, addr: *mut u8, length: usize, offset: usize) -> *mut u8 {
                unsafe {
                    syscall!(9u128<<96 | UID, self.as_raw_handle().0, vmo.0, addr, length, offset =>
                        Ok(res) => {
                            return Ok(core::ptr::with_exposed_provenance_mut(res as usize));
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }
        }
    }
}

pub mod mem {
    super::protocol! {
        pub protocol Pager = 0x6 {
            ctor => {}
        }
    }
}*/
