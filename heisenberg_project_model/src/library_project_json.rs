use std::collections::BTreeMap;

use cargo_metadata::DependencyKind;
use cargo_metadata::Metadata;
use cargo_metadata::PackageId;
use cargo_metadata::TargetKind;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::ProjectJson;
use ra_ap_project_model::ProjectJsonData;
use serde_json::Value;
use serde_json::from_value;
use serde_json::json;

use crate::library_crate::LibraryCrate;
use crate::project_model_error::ProjectModelError;

const LIBRARY_TARGET_KINDS: [TargetKind; 6] = [
    TargetKind::CDyLib,
    TargetKind::DyLib,
    TargetKind::Lib,
    TargetKind::ProcMacro,
    TargetKind::RLib,
    TargetKind::StaticLib,
];

pub struct LibraryProjectJson<'library> {
    pub library_directory: &'library AbsPathBuf,
    pub metadata: &'library Metadata,
}

impl LibraryProjectJson<'_> {
    pub fn project_json(&self) -> Result<ProjectJson, ProjectModelError> {
        let libraries: Vec<LibraryCrate<'_>> = self
            .metadata
            .packages
            .iter()
            .filter_map(|package| {
                package
                    .targets
                    .iter()
                    .find(|target| {
                        target
                            .kind
                            .iter()
                            .any(|kind| LIBRARY_TARGET_KINDS.contains(kind))
                    })
                    .map(|target| LibraryCrate { package, target })
            })
            .collect();
        let mut indices: BTreeMap<&PackageId, usize> = BTreeMap::new();

        for library in &libraries {
            indices.insert(&library.package.id, indices.len());
        }

        let crates: Vec<Value> = libraries
            .iter()
            .map(|library| self.crate_json(library, &indices))
            .collect();
        from_value::<ProjectJsonData>(json!({"crates": crates}))
            .map_err(ProjectModelError::LibraryProjectJson)
            .map(|data| ProjectJson::new(None, self.library_directory, data))
    }

    fn crate_json(
        &self,
        library: &LibraryCrate<'_>,
        indices: &BTreeMap<&PackageId, usize>,
    ) -> Value {
        let nodes = self
            .metadata
            .resolve
            .iter()
            .flat_map(|resolve| &resolve.nodes)
            .filter(|node| node.id == library.package.id);

        json!({
            "cfg": nodes
                .clone()
                .flat_map(|node| &node.features)
                .map(|feature| format!("feature=\"{feature}\""))
                .collect::<Vec<String>>(),
            "deps": nodes
                .flat_map(|node| &node.deps)
                .filter(|dependency| {
                    dependency
                        .dep_kinds
                        .iter()
                        .any(|kind| kind.kind == DependencyKind::Normal)
                })
                .filter_map(|dependency| {
                    indices
                        .get(&dependency.pkg)
                        .map(|index| json!({"crate": index, "name": dependency.name}))
                })
                .collect::<Vec<Value>>(),
            "display_name": library.target.name,
            "edition": library.target.edition.as_str(),
            "is_proc_macro": library.target.is_proc_macro(),
            "is_workspace_member": false,
            "root_module": library.target.src_path,
        })
    }
}
