use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy};
use crate::spec::{RelocModel, Target, TargetOptions};

use super::SanitizerSet;

pub fn target() -> Target {
    let mut base = super::uefi_msvc_base::opts();
    base.cpu = "generic-rv64".into();
    base.max_atomic_width = Some(64);
    base.features = "+m,+a,+f,+d,+c".into();
    base.add_pre_link_args(LinkerFlavor::Msvc(Lld::No), &["/machine:riscv64"]);

    Target {
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        llvm_target: "riscv64-unknown-windows".into(),
        pointer_width: 64,
        arch: "riscv64".into(),

        options: TargetOptions {
            llvm_abiname: "lp64d".into(),
            features: "+m,+a,+f,+d,+c".into(),
            relocation_model: RelocModel::Static,
            code_model: Some(CodeModel::Medium),
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
