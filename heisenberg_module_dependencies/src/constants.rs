use heisenberg_module_graph::violation_kind::ViolationKind;

pub const CARGO_BINARY_NAME_VARIABLE: &str = "CARGO_BIN_NAME";

pub const CARGO_BINARY_PATH_VARIABLE_PREFIX: &str = "CARGO_BIN_EXE_";

pub const CARGO_PRIMARY_PACKAGE_VARIABLE: &str = "CARGO_PRIMARY_PACKAGE";

pub const CARGO_TARGET_TEMPORARY_DIRECTORY_VARIABLE: &str = "CARGO_TARGET_TMPDIR";

pub const ERROR_DISPLAY_METHOD: &str = "as_display";

pub const ERROR_MESSAGE_FUNCTION: &str = "fmt";

pub const ERROR_MESSAGE_HELPER: &str = "error";

pub const ERROR_SOURCE_FIELD: &str = "source";

pub const ERROR_SOURCE_HELPERS: [&str; 2] = ["from", "source"];

pub const ERROR_TRAIT_PATH: [&str; 2] = ["error", "Error"];

pub const LIFETIME_PREFIX: char = '\'';

pub const OWNED_BOX_PATH: [&str; 2] = ["boxed", "Box"];

pub const VECTOR_PATH: [&str; 2] = ["vec", "Vec"];

pub const TEST_MODULE_NAME: &str = "tests";

pub const TRANSPARENT_ERROR_MESSAGE: &str = "transparent";

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

pub const UNSAFE_ATTRIBUTES: [&str; 3] = ["export_name", "link_section", "no_mangle"];

pub const WORD_SEPARATOR: char = '_';
