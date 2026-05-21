#[allow_internal_unstable(asm_goto_with_outputs)]
pub macro syscall {
    ($uid:expr, $arg0:expr) => {
        'syscall: {
            let uid = <u128 as ::core::convert::From<_>>::from($uid);
            let arg0: $crate::os::popcorn::io::RawHandle = $arg0;
            let low: u64;
            let high: u64;
            ::core::arch::asm!(
                "clc",
                "syscall",
                "jc {error}",
                inout("rax") uid as u64 => low,
                out("rcx") _,
                out("rdx") high,
                out("rsi") _,
                inout("rdi") arg0.0 => _,
                out("r8") _,
                inout("r9") (uid >> 64) as u64 => _,
                out("r10") _,
                out("r11") _,
                out("r12") _,
                error = label { match (high as u128) << 64 | (low as u128) { err => break 'syscall ::core::result::Result::Err(err), } }
            );
            match (high as u128) << 64 | (low as u128) { res => ::core::result::Result::Ok(res), }
        }
    },
    ($uid:expr, $arg0:expr, $arg1:expr) => {
        'syscall: {
            let uid = <u128 as ::core::convert::From<_>>::from($uid);
            let arg0: $crate::os::popcorn::io::RawHandle = $arg0;
            let arg1 = <u64 as ::core::convert::From<_>>::from($arg1);
            let low: u64;
            let high: u64;
            ::core::arch::asm!(
                "clc",
                "syscall",
                "jc {error}",
                inout("rax") uid as u64 => low,
                out("rcx") _,
                out("rdx") high,
                inout("rdi") arg0.0 => _,
                inout("rsi") arg1 => _,
                out("r8") _,
                inout("r9") (uid >> 64) as u64 => _,
                out("r10") _,
                out("r11") _,
                out("r12") _,
                error = label { match (high as u128) << 64 | (low as u128) { err => break 'syscall ::core::result::Result::Err(err), } }
            );
            match (high as u128) << 64 | (low as u128) { res => ::core::result::Result::Ok(res), }
        }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr) => {
        'syscall: {
            let uid = <u128 as ::core::convert::From<_>>::from($uid);
            let arg0: $crate::os::popcorn::io::RawHandle = $arg0;
            let arg1 = <u64 as ::core::convert::From<_>>::from($arg1);
            let arg2 = <u64 as ::core::convert::From<_>>::from($arg2);
            let low: u64;
            let high: u64;
            ::core::arch::asm!(
                "clc",
                "syscall",
                "jc {error}",
                inout("rax") uid as u64 => low,
                out("rcx") _,
                inout("rdx") arg2 => high,
                inout("rdi") arg0.0 => _,
                inout("rsi") arg1 => _,
                out("r8") _,
                inout("r9") (uid >> 64) as u64 => _,
                out("r10") _,
                out("r11") _,
                out("r12") _,
                error = label { match (high as u128) << 64 | (low as u128) { err => break 'syscall ::core::result::Result::Err(err), } }
            );
            match (high as u128) << 64 | (low as u128) { res => ::core::result::Result::Ok(res), }
        }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3: expr) => {
        'syscall: {
            let uid = <u128 as ::core::convert::From<_>>::from($uid);
            let arg0: $crate::os::popcorn::io::RawHandle = $arg0;
            let arg1 = <u64 as ::core::convert::From<_>>::from($arg1);
            let arg2 = <u64 as ::core::convert::From<_>>::from($arg2);
            let arg3 = <u64 as ::core::convert::From<_>>::from($arg3);
            let low: u64;
            let high: u64;
            ::core::arch::asm!(
                "clc",
                "syscall",
                "jc {error}",
                inout("rax") uid as u64 => low,
                out("rcx") _,
                inout("rdx") arg2 => high,
                inout("rdi") arg0.0 => _,
                inout("rsi") arg1 => _,
                out("r8") _,
                inout("r9") (uid >> 64) as u64 => _,
                inout("r10") arg3 => _,
                out("r11") _,
                out("r12") _,
                error = label { match (high as u128) << 64 | (low as u128) { err => break 'syscall ::core::result::Result::Err(err), } }
            );
            match (high as u128) << 64 | (low as u128) { res => ::core::result::Result::Ok(res), }
        }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        'syscall: {
            let uid = <u128 as ::core::convert::From<_>>::from($uid);
            let arg0: $crate::os::popcorn::io::RawHandle = $arg0;
            let arg1 = <u64 as ::core::convert::From<_>>::from($arg1);
            let arg2 = <u64 as ::core::convert::From<_>>::from($arg2);
            let arg3 = <u64 as ::core::convert::From<_>>::from($arg3);
            let arg4 = <u64 as ::core::convert::From<_>>::from($arg4);
            let low: u64;
            let high: u64;
            ::core::arch::asm!(
                "clc",
                "syscall",
                "jc {error}",
                inout("rax") uid as u64 => low,
                out("rcx") _,
                inout("rdx") arg2 => high,
                inout("rdi") arg0.0 => _,
                inout("rsi") arg1 => _,
                inout("r8") arg4 => _,
                inout("r9") (uid >> 64) as u64 => _,
                inout("r10") arg3 => _,
                out("r11") _,
                out("r12") _,
                error = label { match (high as u128) << 64 | (low as u128) { err => break 'syscall ::core::result::Result::Err(err), } }
            );
            match (high as u128) << 64 | (low as u128) { res => ::core::result::Result::Ok(res), }
        }
    }
}
