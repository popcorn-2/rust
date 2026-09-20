#[cfg(target_arch = "x86_64")]
pub macro syscall($handle:expr, $interface:expr, $method: expr, @integer = $i:expr, @oob = $oob:expr $(,)?) {{
    let handle: i32 = $handle;
    let interface: u64 = $interface;
    const METHOD: u16 = $method;
    let result: isize;
    let int_args: &[usize] = & $i;
    let oob_args: &[usize] = & $oob;

    core::arch::asm!(
        "shl rax, 16",
        "mov ax, {method}",
        "push rbp",
        "push rbx",
        "syscall",
        "pop rbx",
        "pop rbp",
        method = const METHOD,
        in("rax") handle,
        in("r12") interface,
        // fixme: pointless to init all of this
        in("rdi") *int_args.get(0).unwrap_or(&0),
        in("rsi") *int_args.get(1).unwrap_or(&0),
        in("rdx") *int_args.get(2).unwrap_or(&0),
        in("r8") *oob_args.get(0).unwrap_or(&0),
        in("r9") *oob_args.get(1).unwrap_or(&0),
        lateout("rax") result,
        lateout("rcx") _,
        lateout("rdx") _,
        lateout("rsi") _,
        lateout("rdi") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("r13") _,
        lateout("r14") _,
        lateout("r15") _,
        clobber_abi("sysv64"),
    );

    result
}}
