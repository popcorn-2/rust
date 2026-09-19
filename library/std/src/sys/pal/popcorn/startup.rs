use crate::sync::atomic::{AtomicPtr, AtomicI32};

pub static PROC_INFO: AtomicPtr<ProcInfo> = AtomicPtr::new(core::ptr::null_mut());
pub static ADDRESS_SPACE_HANDLE: AtomicI32 = AtomicI32::new(-1);

#[repr(C)]
pub struct ProcInfo {
	magic: [u8; 7],
	version: u8,
	// version 0 fields
	pub argc: core::ffi::c_int,
	pub argv: *const *const core::ffi::c_char,
	pub named_handles: *const core::ffi::c_void,
	info_ty: usize,
	info_ptr: *const core::ffi::c_void,
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
r#"
.section .rodata
address_space_handle_name: .asciz "address_space.main"
popcorn_startup_magic: .asciz "POPCRN"

.section .text
.global _start

_start:
  mov qword ptr [{proc_info_storage}], rdi # proc info global holds pointer to `struct proc_info_t`
  # check startup magic is valid
  mov ecx, 7
  lea rsi, [popcorn_startup_magic]
  repe cmpsb
  jne .startup_failure_magic

  # locate address space handle
  mov rbx, qword ptr [rdi - 7 + 24] # hold pointer to current `struct named_handle_t`
.address_space_handle_loop:
  cmp qword ptr [rbx], 0
  jz .startup_failure_no_address_space # if null string, end of handle list
  lea rsi, [address_space_handle_name]
  mov rdi, qword ptr [rbx]
  mov ecx, 19
  repe cmpsb
  je .found_address_space
  add rbx, 16 # load next `struct named_handle_t` and loop
  jmp .address_space_handle_loop

  # request a stack
.found_address_space:
  mov eax, dword ptr [rbx + 8]    # load handle number of address space into eax
  mov dword ptr [{address_space_handle_storage}], eax # store handle number in global var
  mov r12, 1            # interface num for allocate_anon
  mov rdi, {stack_size} # request 16KiB
  mov rsi, 0b101        # request RW, no execute
  syscall
  cmp rax, 0
  js .startup_failure_stack_alloc
  lea rsp, [rax + {stack_size}] # load top of stack into rsp
  jmp {startup}

.startup_failure_magic:
  ud2
  .asciz "\033startup magic was incorrect"
.startup_failure_no_address_space:
  ud2
  .asciz "\033could not find address space handle"
.startup_failure_stack_alloc:
  ud2
  .asciz "\033failed to allocate stack"
"#,
    stack_size = const 32 * 1024,
    proc_info_storage = sym PROC_INFO,
    address_space_handle_storage = sym ADDRESS_SPACE_HANDLE,
    startup = sym startup,
);

extern "C" fn startup() -> ! {
    // Call main.
    unsafe extern "C" {
        fn main(_: isize, _: *const *const u8, _: u8) -> i32;
    }
    let _result = unsafe { main(0, core::ptr::null(), 0) };

    rtabort!("program unexpectedly returned");
}
