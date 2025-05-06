pub macro syscall {
    ($uid:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "clc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            out("rdx") high,
            out("rdi") _,
            out("rsi") _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "clc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            out("rdx") high,
            out("rsi") _,
            inout("rdi") $arg0 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "clc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            out("rdx") high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "clc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            inout("rdx") $arg2 as usize => high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3: expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "clc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            inout("rdx") $arg2 as usize => high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            inout("r10") $arg3 as usize => _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "clc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            inout("rdx") $arg2 as usize => high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            inout("r8") $arg4 as usize => _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            inout("r10") $arg3 as usize => _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    }
}

pub macro syscall_async {
    ($uid:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "stc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            out("rdx") high,
            out("rdi") _,
            out("rsi") _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "stc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            out("rdx") high,
            out("rsi") _,
            inout("rdi") $arg0 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "stc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            out("rdx") high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "stc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            inout("rdx") $arg2 as usize => high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3: expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "stc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            inout("rdx") $arg2 as usize => high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            out("r8") _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            inout("r10") $arg3 as usize => _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    },
    ($uid:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr => Ok($res_h:ident) => $happy:block Err($res_e:ident) => $error:block) => {
        let low: u64;
        let high: u64;
        ::core::arch::asm!(
            "stc",
            "syscall",
            "jc {error}",
            inout("rax") $uid as u64 => low,
            out("rcx") _,
            inout("rdx") $arg2 as usize => high,
            inout("rdi") $arg0 as usize => _,
            inout("rsi") $arg1 as usize => _,
            inout("r8") $arg4 as usize => _,
            inout("r9") (($uid as u128) >> 64) as u64 => _,
            inout("r10") $arg3 as usize => _,
            out("r11") _,
            out("r12") _,
            error = label { match (high as u128) << 64 | (low as u128) { $res_e => $error } }
        );
        match (high as u128) << 64 | (low as u128) { $res_h => $happy }
    }
}
