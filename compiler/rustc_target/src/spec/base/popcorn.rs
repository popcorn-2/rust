use crate::spec::{BinaryFormat, Os, Cc, FramePointer, LinkerFlavor, Lld, RelroLevel, RelocModel, TargetOptions, PanicStrategy};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Popcorn,
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        dynamic_linking: false,
        relocation_model: RelocModel::Static,
        frame_pointer: FramePointer::Always,
        dll_prefix: "lib".into(),
        dll_suffix: ".dyn.elf".into(),
        exe_suffix: ".elf".into(),
        staticlib_prefix: "lib".into(),
        staticlib_suffix: ".stat.elf".into(),
        binary_format: BinaryFormat::Elf,
        position_independent_executables: false,
        relro_level: RelroLevel::Off,
        plt_by_default: false,
        main_needs_argc_argv: false,
        has_thread_local: true,
        crt_static_default: true,
        crt_static_respected: true,
        crt_static_allows_dylibs: true,
        panic_strategy: PanicStrategy::Unwind,
        // HACK
        singlethread: true,
        ..Default::default()
    }
}
