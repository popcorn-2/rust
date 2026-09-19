use crate::spec::{BinaryFormat, Os, Cc, FramePointer, LinkerFlavor, Lld, RelroLevel, RelocModel, TargetOptions, PanicStrategy, LinkArgs};

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

        pre_link_args: LinkArgs::from([
            (LinkerFlavor::Gnu(Cc::No, Lld::Yes), vec![
                std::borrow::Cow::Borrowed("-L=/system/lib"),
                std::borrow::Cow::Borrowed("-l:crt1.o"),
                std::borrow::Cow::Borrowed("-lrt_elf"),
            ]),
        ]),

        // temporary hack until threading is supported
        singlethread: true,
        ..Default::default()
    }
}
