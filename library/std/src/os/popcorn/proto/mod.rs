#![unstable(feature = "popcorn_protocol", issue = "none")]

mod has_protocol;
pub use has_protocol::*;
//pub mod server;

pub trait Protocol {
	type Ctor;
	const UID: u128;
}

pub trait ProtocolTuple: crate::sealed::Sealed {
    type Ctor;
    const UID: &'static [u128];
    type __PrivateAbiCompat;
    fn __private_abi_convert(r: Self::Ctor) -> Self::__PrivateAbiCompat;
}

#[derive(Debug, Copy, Clone)]
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
}

macro_rules! protocol_tuple {
    ($compat_ty:ident $T:ident) => {
        protocol_tuple!(@ $compat_ty #[cfg_attr(doc, doc(fake_variadic))] $T);
    };
    ($compat_ty:ident $($T:ident)*) => {
        protocol_tuple!(@ $compat_ty #[cfg_attr(doc, doc(hidden))] $($T)*);
    };
    (@ $compat_ty:ident $(#[$attr:meta])* $($T:ident)*) => {
        #[repr(C)]
        pub struct $compat_ty <$($T),*> ($($T),*,);

        impl<$($T: Protocol),*> crate::sealed::Sealed for ($($T),*,) {}

        $(#[$attr])*
        impl<$($T: Protocol),*> ProtocolTuple for ($($T),*,) {
            type Ctor = ($($T),*,);
            type __PrivateAbiCompat = $compat_ty<$($T),*>;
            const UID: &'static [u128] = &[
                $(<$T as Protocol>::UID),*
            ];
            #[inline(always)]
            fn __private_abi_convert(r: Self::Ctor) -> Self::__PrivateAbiCompat {
                $compat_ty ($( ${ignore($T)} r.${index()} ),*)
            }
        }
    };
}

impl<T: ?Sized> crate::sealed::Sealed for T where T: Protocol {}
impl<T: ?Sized> ProtocolTuple for T where T: Protocol {
	type Ctor = <T as Protocol>::Ctor;
    const UID: &'static [u128] = &[<T as Protocol>::UID];
    type __PrivateAbiCompat = <T as Protocol>::Ctor;
    fn __private_abi_convert(r: Self::Ctor) -> Self::__PrivateAbiCompat { r }
}

protocol_tuple!(__PrivateT T);
protocol_tuple!(__PrivateTU T U);
protocol_tuple!(__PrivateTUV T U V);
protocol_tuple!(__PrivateTUVW T U V W);
protocol_tuple!(__PrivateTUVWX T U V W X);

macro_rules! protocol {
    () => {};
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
    };
}

pub(self) use protocol;

pub mod fs {
    protocol! {
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
    
    protocol! {
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

    protocol! {
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

			/*fn set_entry(&self, entry: *const u8) -> () {
                unsafe {
                    syscall!(4u128<<96 | UID, self.as_raw_handle().0, entry =>
                        Ok(_res) => {
                            return Ok(());
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }*/
        }

        pub protocol Thread = 0xA {
            ctor => {}

			fn unstable_anon_alloc(&self, size: usize) -> *mut u8 {
                unsafe {
                    syscall!(1u128<<96 | UID, self.as_raw_handle().0, size =>
                        Ok(res) => {
                            return Ok(core::ptr::with_exposed_provenance_mut(res as usize));
                        }
                        Err(res) => {
                            return Err(crate::io::Error::from_raw_os_error(res as isize));
                        }
                    );
                }
            }

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
        }
    }
}
