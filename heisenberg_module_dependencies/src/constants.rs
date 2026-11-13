use heisenberg_module_graph::violation_kind::ViolationKind;

pub const LIFETIME_PREFIX: char = '\'';

pub const TYPE_PARAMETER_PREFIX: char = 'T';

pub const UNRESOLVED_CODE_KINDS: [ViolationKind; 8] = [
    ViolationKind::FailedMacroExpansion,
    ViolationKind::UnimplementedBuiltinMacro,
    ViolationKind::UnparsableMacroExpansion,
    ViolationKind::UnresolvedExternCrate,
    ViolationKind::UnresolvedMacroCall,
    ViolationKind::UnresolvedMethodCall,
    ViolationKind::UnresolvedModule,
    ViolationKind::UnresolvedPath,
];

pub const WORD_SEPARATOR: char = '_';
