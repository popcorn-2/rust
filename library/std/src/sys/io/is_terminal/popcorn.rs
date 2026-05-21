use crate::os::popcorn::io::AsRawHandle;
use crate::os::popcorn::proto::io::Terminal;
use crate::os::popcorn::proto::abi_v1::AbiV1;

pub fn is_terminal(handle: &impl AsRawHandle) -> bool {
    handle.as_raw_handle().has_protocols::<&dyn Terminal>()
}
