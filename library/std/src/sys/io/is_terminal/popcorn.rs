use crate::os::popcorn::handle::{AsHandle, AsRawHandle};
use crate::os::popcorn::proto::Protocol;

pub fn is_terminal<I: ?Sized>(handle: &impl AsHandle<I>) -> bool {
    handle.as_handle()
        .as_raw_handle()
        .has_protocol(&[crate::os::popcorn::proto::io::Terminal::UID])
        .unwrap_or(false)
}
