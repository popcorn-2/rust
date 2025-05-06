use crate::{collections::HashMap, io::self, os::popcorn::{proto::{Error, ProtocolTuple}, handle::{AsRawHandle, OwnedHandle, BorrowedHandle, RawHandle, FromRawHandle}}};
use core::{mem::MaybeUninit, ptr::{slice_from_raw_parts, with_exposed_provenance}};
use crate::ffi::OsStr;
use crate::path::Path;
use core::mem::ManuallyDrop;
use core::pin::Pin;
use core::future::Future;

super::protocol! {
	pub protocol Sync = 8 {
		ctor => {}

		fn next(&self) -> Packet {
			let mut buf = MaybeUninit::<Packet>::uninit();

			unsafe {
				syscall!(1u128<<96 | UID, self.as_raw_handle().0, buf.as_mut_ptr() =>
					Ok(_res) => {
						return Ok(buf.assume_init())
					}
					Err(res) => {
						return Err(crate::io::Error::from_raw_os_error(res as isize));
					}
				);
			}
		}

		fn reply(&self, response: Response) -> () {
			unsafe {
				syscall!(2u128<<96 | UID, self.as_raw_handle().0, &response as *const Response =>
					Ok(_res) => {
						return Ok(())
					}
					Err(res) => {
						return Err(crate::io::Error::from_raw_os_error(res as isize));
					}
				);
			}
		}

		fn forge<U: ProtocolTuple>(&self, handle: isize) -> OwnedHandle<U> {
			unsafe {
				syscall!(3u128<<96 | UID, self.as_raw_handle().0, handle, U::UID.as_ptr(), U::UID.len() * size_of::<u128>() =>
					Ok(res) => {
						return Ok(unsafe { OwnedHandle::from_raw_handle(RawHandle(res as isize)) });
					}
					Err(res) => {
						return Err(crate::io::Error::from_raw_os_error(res as isize));
					}
				);
			}
		}
	}
}

