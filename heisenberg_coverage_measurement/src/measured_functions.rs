use std::collections::HashMap;

use ra_ap_hir::InFile;
use ra_ap_ide_db::FilePosition;
use ra_ap_syntax::SyntaxNodePtr;

use heisenberg_module_graph::source_location::SourceLocation;

#[derive(Default)]
pub struct MeasuredFunctions {
    pub braces: HashMap<InFile<SyntaxNodePtr>, FilePosition>,
    pub claims: HashMap<FilePosition, Vec<FilePosition>>,
    pub names: HashMap<FilePosition, SourceLocation>,
}

impl MeasuredFunctions {
    pub fn claim(
        &mut self,
        token: FilePosition,
        functions: impl IntoIterator<Item = FilePosition>,
    ) {
        for function in functions {
            let owners = self.claims.entry(token).or_default();

            if !owners.contains(&function) {
                owners.push(function);
            }
        }
    }

    pub fn merge(
        &mut self,
        Self {
            braces,
            claims,
            names,
        }: Self,
    ) {
        self.braces.extend(braces);

        for (token, owners) in claims {
            self.claim(token, owners);
        }

        for (function, name) in names {
            self.register(function, name);
        }
    }

    pub fn register(&mut self, function: FilePosition, name: SourceLocation) {
        self.names
            .entry(function)
            .and_modify(|current| *current = current.clone().min(name.clone()))
            .or_insert(name);
    }
}
