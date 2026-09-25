#[repr(C)]
struct Elf64_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

const PT_GNU_EH_FRAME: u32 = 0x6474e550;

#[repr(C)]
struct ElfLoaderInfo {
    magic: [u8; 4],
    at_entry: usize,
    at_base: usize,
    at_phdr: *const Elf64_Phdr,
    at_phnum: usize,
}

struct PopcornEhFrameFinder;
static mut EH_FRAME_HDR: usize = 0;
static mut TEXT_BASE: usize = 0;
static EH_FRAME_FINDER: PopcornEhFrameFinder = PopcornEhFrameFinder;

unsafe impl unwind::EhFrameFinder for PopcornEhFrameFinder {
    fn find(&self, _pc: usize) -> Option<unwind::FrameInfo> {
        let hdr = unsafe { EH_FRAME_HDR };
        if hdr == 0 {
            None
        } else {
            Some(unwind::FrameInfo {
                text_base: Some(unsafe { TEXT_BASE }),
                kind: unwind::FrameInfoKind::EhFrameHdr(hdr),
            })
        }
    }
}

#[used]
#[unsafe(link_section = ".init_array.0")]
static STARTUP_INIT_ARRAY: extern "C" fn(
    *const super::startup::ProcInfo
) = {
    extern "C" fn init_wrapper(
        proc_info: *const super::startup::ProcInfo,
    ) {
        unsafe { init_unwind(&*proc_info) };
    }
    init_wrapper
};

extern "C" fn init_unwind(proc_info: &super::startup::ProcInfo) {
    if proc_info.info_ty == 1 /* ELF */ {
        let elf_info = unsafe { &*proc_info.info_ptr.cast::<ElfLoaderInfo>() };

        if elf_info.magic == *b"\x7fELF" {
            let phdrs = unsafe {
                core::slice::from_raw_parts(elf_info.at_phdr, elf_info.at_phnum)
            };

            for phdr in phdrs {
                if phdr.p_type == PT_GNU_EH_FRAME {
                    unsafe {
                        EH_FRAME_HDR = elf_info.at_base + phdr.p_vaddr as usize;
                        TEXT_BASE = elf_info.at_base;
                    }
                    let _ = unwind::set_custom_eh_frame_finder(&EH_FRAME_FINDER);
                    break;
                }
            }
        }
    }
}
