#![forbid(unsafe_op_in_unsafe_fn)]

cfg_select! {
    any(target_os = "windows", target_os = "uefi") => {
        mod wtf8;
        pub(crate) use wtf8::{Buf, Slice};
    }
    any(target_os = "motor", target_os = "popcorn") => {
        mod utf8;
        pub(crate) use utf8::{Buf, Slice};
    }
    _ => {
        mod bytes;
        pub(crate) use bytes::{Buf, Slice};
    }
}
