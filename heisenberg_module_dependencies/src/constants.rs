use heisenberg_module_graph::violation_kind::ViolationKind;

pub const DESERIALIZE_OWNED_TRAIT_PATH: [&str; 2] = ["de", "DeserializeOwned"];

pub const DESERIALIZE_TRAIT_PATH: [&str; 2] = ["de", "Deserialize"];

pub const EQUALITY_ASSERTION_MACROS: [&str; 2] = ["assert_eq", "debug_assert_eq"];

pub const ERROR_DISPLAY_METHOD: &str = "as_display";

pub const ERROR_MESSAGE_HELPER: &str = "error";

pub const ERROR_SOURCE_FIELD: &str = "source";

pub const ERROR_SOURCE_HELPERS: [&str; 2] = ["from", "source"];

pub const ERROR_TRAIT_PATH: [&str; 2] = ["error", "Error"];

pub const LIFETIME_PREFIX: char = '\'';

pub const ROUND_TRIP_TRANSPARENT_METHODS: [&str; 5] =
    ["as_bytes", "as_slice", "as_str", "expect", "unwrap"];

pub const SERIALIZE_TRAIT_PATH: [&str; 2] = ["ser", "Serialize"];

pub const STANDARD_SLEEP_MS_PATH: [&str; 2] = ["thread", "sleep_ms"];

pub const STANDARD_SLEEP_PATH: [&str; 2] = ["thread", "sleep"];

pub const TEXT_DECODING_METHOD: &str = "parse";

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

pub const WORD_SEPARATOR: char = '_';
