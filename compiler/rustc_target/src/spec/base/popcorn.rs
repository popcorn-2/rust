use crate::spec::{Cc, LinkerFlavor, Lld, RelroLevel, TargetOptions, PanicStrategy};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "popcorn".into(),
        dynamic_linking: false,
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::Yes),
        exe_suffix: ".exec".into(),
        position_independent_executables: false,
        relro_level: RelroLevel::Off,
        has_thread_local: true,
        crt_static_default: true,
        crt_static_respected: true,
        crt_static_allows_dylibs: false,
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    }
}

