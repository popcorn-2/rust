use crate::spec::{Target, TargetMetadata, base, cvs};

pub(crate) fn target() -> Target {
    let mut base = base::popcorn::opts();
    base.cpu = "x86-64".into();
    base.plt_by_default = false;
    base.features = "+cx16,+sse,+sse2".into();
    base.max_atomic_width = Some(128);
    base.env = "posix".into();
    base.families = cvs!["unix"];

    Target {
        llvm_target: "x86_64-unknown-popcorn".into(),
        metadata: TargetMetadata {
            description: Some("64-bit x86 Popcorn2 using libc backend".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
    }
}

