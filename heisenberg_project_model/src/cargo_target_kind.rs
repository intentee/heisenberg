use cargo::workspace::TargetKind;

use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

pub struct CargoTargetKind<'target> {
    pub kind: &'target TargetKind,
}

impl CargoTargetKind<'_> {
    pub const fn crate_target_kind(&self) -> CrateTargetKind {
        match self.kind {
            TargetKind::Bench => CrateTargetKind::Benchmark,
            TargetKind::Bin => CrateTargetKind::Binary,
            TargetKind::CustomBuild => CrateTargetKind::BuildScript,
            TargetKind::ExampleBin | TargetKind::ExampleLib(_) => CrateTargetKind::Example,
            TargetKind::Lib(_) => CrateTargetKind::Library,
            TargetKind::Test => CrateTargetKind::IntegrationTest,
        }
    }
}
