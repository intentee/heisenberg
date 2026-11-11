use ra_ap_hir::Crate;
use ra_ap_hir::attach_db;
use ra_ap_vfs::VfsPath;

use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::source_file::SourceFile;
use heisenberg_project_model::workspace_packages::WorkspacePackages;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::analysis_priming::AnalysisPriming;
use crate::member_crate::MemberCrate;
use crate::member_crates_error::MemberCratesError;
use crate::resolved_member_crate::ResolvedMemberCrate;

fn members(
    locator: &SourceLocator<'_>,
    WorkspacePackages { member_targets, .. }: &WorkspacePackages,
) -> Result<Vec<MemberCrate>, MemberCratesError> {
    let database = locator.database;

    attach_db(database, || {
        Crate::all(database)
            .into_iter()
            .filter_map(|krate| {
                locator
                    .source_file(krate.root_file(database))
                    .map(|SourceFile { path, .. }| {
                        member_targets
                            .get(&VfsPath::from(path.to_path_buf()))
                            .map(|crate_target| MemberCrate {
                                crate_target: crate_target.clone(),
                                krate,
                            })
                    })
                    .transpose()
            })
            .collect()
    })
}

/// # Errors
///
/// Returns [`MemberCratesError`] when rust-analyzer refers to a crate root that the loaded
/// project does not hold, or when name resolution and trait implementations cannot be
/// scheduled over the crate dependency graph.
pub fn member_crates(
    LoadedProject {
        database,
        files,
        packages,
        ..
    }: &LoadedProject<'_>,
) -> Result<Vec<ResolvedMemberCrate>, MemberCratesError> {
    members(&SourceLocator { database, files }, packages).and_then(|members| {
        AnalysisPriming {
            database,
            members: &members
                .iter()
                .map(|MemberCrate { krate, .. }| *krate)
                .collect::<Vec<Crate>>(),
        }
        .prime()
        .map(|()| {
            attach_db(database, || {
                members
                    .into_iter()
                    .map(|member| member.resolved(database))
                    .collect()
            })
        })
    })
}
