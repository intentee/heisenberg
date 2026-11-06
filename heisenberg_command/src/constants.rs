use crate::issue_text::IssueText;

macro_rules! resolution_purpose {
    ($instruction:literal) => {
        concat!(
            "Every name must resolve for the analysis to be complete. ",
            $instruction
        )
    };
}

pub const ADDITIONAL_PUBLIC_ITEM: IssueText = IssueText {
    definition: "A second public item in a module, constants and modules aside, among the items written in module files. An item is public when its visibility reaches beyond its own module, so a `pub(self)` item is not public while a `pub` item of a private module is, and a `macro_rules!` macro is public when exported.",
    name: "AdditionalPublicItem",
    purpose: "Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.",
    summary: "A module with more than one public item",
};

pub const CLIPPY_CONFIGURATION_AMBIGUOUS: IssueText = IssueText {
    definition: "A directory that holds both clippy configuration files and is the nearest one with a clippy configuration for a workspace member, within the workspace.",
    name: "ClippyConfigurationAmbiguous",
    purpose: "Clippy reads only one of clippy.toml and .clippy.toml, so keeping both hides which applies. Keep one of them.",
    summary: "A directory that holds both clippy configuration files",
};

pub const CLIPPY_CONFIGURATION_MISSING: IssueText = IssueText {
    definition: "A workspace member with no clippy configuration file in its directory or an ancestor within the workspace.",
    name: "ClippyConfigurationMissing",
    purpose: "Tests may fail fast while production code may not, and clippy needs a configuration to tell them apart. Add a clippy.toml with the test allowances.",
    summary: "A workspace member without a clippy configuration",
};

pub const CLIPPY_LINT_NOT_DENIED: IssueText = IssueText {
    definition: "A workspace member whose lints, after workspace inheritance and in cargo's order, leave a required lint below deny, under either spelling of its name or a name clippy renamed to it, unless it is a test crate, which may panic and then declares its own [lints] table, since cargo cannot override single lints of an inherited table. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].",
    name: "ClippyLintNotDenied",
    purpose: "Production code reports failures as typed errors instead of panicking. Deny the lint in the [lints.clippy] or [workspace.lints.clippy] table the package uses; only a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists, may leave it undenied, in its own lints table.",
    summary: "A workspace member that leaves a required clippy lint below deny",
};

pub const CLIPPY_TEST_ALLOWANCE_MISSING: IssueText = IssueText {
    definition: "A clippy configuration that does not set each test allowance to true, which is the only way tests may use them.",
    name: "ClippyTestAllowanceMissing",
    purpose: "Tests may fail fast with expect, unwrap and panic. Set the allowance to true in the clippy configuration.",
    summary: "A clippy configuration that does not allow tests to fail fast",
};

pub const CODE_ACTIVE_ON_NO_TARGET: IssueText = IssueText {
    definition: "Code active on no declared target. A module file that its own attributes configure out is inactive code at its declaration. Code that a negated predicate leaves active on no declared target is listed within the predicate's NegatedConfiguration issue instead.",
    name: "CodeActiveOnNoTarget",
    purpose: "Code active on no declared target is never built or analysed. Remove it, declare a supported target it is active on, or make it active on a declared target.",
    summary: "Code that no declared target builds",
};

pub const CONFIGURATION_SELECTION_MACRO: IssueText = IssueText {
    definition: "A cfg_select! call, whose unselected arms escape the analysis.",
    name: "ConfigurationSelectionMacro",
    purpose: "cfg_select! discards the arms it does not select before they can be analysed. Write each arm as an item under its own #[cfg] attribute.",
    summary: "A cfg_select! call",
};

pub const DUPLICATED_FUNCTION_BODY: IssueText = IssueText {
    definition: "Functions written in the module files of workspace members whose signatures and bodies are identical once every name is replaced by the definition it resolves to, with names a function declares itself compared by declaration order and names that resolve to nothing never equal, functions with an empty body aside.",
    name: "DuplicatedFunctionBody",
    purpose: "Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.",
    summary: "Functions whose signatures and bodies are identical",
};

pub const FAILED_MACRO_EXPANSION: IssueText = IssueText {
    definition: "A macro call whose expansion fails. Every failing expansion is listed within the macro call written in the source.",
    name: "FailedMacroExpansion",
    purpose: "A macro call that fails to expand produces no code to analyse. Fix the call so it expands.",
    summary: "A macro call that fails to expand",
};

pub const FUNCTION_USED_ONLY_BY_TESTS: IssueText = IssueText {
    definition: "A free or inherent function outside test code whose references all lie in test code. Every function of a package with publish = false is checked; in a package that can be published, only those its crate does not expose to other crates.",
    name: "FunctionUsedOnlyByTests",
    purpose: "Production code holds only what production uses. Move a function that only tests use into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.",
    summary: "A production function that only tests use",
};

pub const GLOB_IMPORT: IssueText = IssueText {
    definition: "A glob import.",
    name: "GlobImport",
    purpose: "Glob imports hide where names come from. Import each name explicitly.",
    summary: "A glob import",
};

pub const IMPL_WITHOUT_LOCAL_TYPE: IssueText = IssueText {
    definition: "An impl of another crate's trait for a self type this crate does not define, with no single struct, enum or union of this crate among the trait's type arguments.",
    name: "ImplWithoutLocalType",
    purpose: "An impl of another crate's trait belongs beside the one type of this crate it serves. Implement the trait for a struct, enum or union of this crate, or for a foreign type with exactly one of them among the trait's type arguments.",
    summary: "An impl of another crate's trait with no home in this crate",
};

pub const INVALID_MACRO_DEFINITION: IssueText = IssueText {
    definition: "A macro definition that rust-analyzer reports as invalid.",
    name: "InvalidMacroDefinition",
    purpose: "An invalid macro definition produces no code to analyse. Fix the definition.",
    summary: "A macro definition that is not valid",
};

pub const ITEM_OUT_OF_ORDER: IssueText = IssueText {
    definition: "An item of a module file or inline module, among the items written in module files, that follows an item of a later group. The groups, in order: module declarations; imports from vendor crates, which are std, core, alloc and every crate outside the workspace; imports from other workspace members; imports from this crate; private functions, constants, statics, macro_rules! macros and macro calls; private structs, enums, unions, type aliases and traits with their impl blocks; public items with their impl blocks; inline modules. An import takes the group of the crate that the first segment of each of its paths resolves to, an impl block the group of the item of its file it belongs to, and an item is public when its visibility reaches beyond its own module. The item is reported at its name, with the first earlier item of a later group.",
    name: "ItemOutOfOrder",
    purpose: "Every file reads in the same order, from what it declares and imports down to what it exposes. Move the item before the item the message names.",
    summary: "An item that follows an item of a later group",
};

pub const LINT_ALLOWED_IN_MANIFEST: IssueText = IssueText {
    definition: "A workspace member whose lints, after workspace inheritance, allow any lint of any tool. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].",
    name: "LintAllowedInManifest",
    purpose: "Allowing a lint is a suppression, and suppressions are a slippery slope. Remove the allow; if the lint then reports code, fix that code.",
    summary: "A manifest lints table that allows a lint",
};

pub const LINT_SUPPRESSION: IssueText = IssueText {
    definition: "An allow or expect attribute of any lint, or a warn attribute that lowers a required lint or its group, including one that a cfg_attr applies or a workspace member's macro writes.",
    name: "LintSuppression",
    purpose: "Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.",
    summary: "An attribute that allows or expects a lint, or lowers a required one",
};

pub const METHOD_OUT_OF_ORDER: IssueText = IssueText {
    definition: "A function of an impl block, among the items written in module files, that follows a function of a later group, or of its own group with a later name. The groups, in order: constructors, which return Self, Option<Self> or Result<Self, _>, an async function by the output of its future, with or without a self receiver; then, by visibility, pub, pub(crate), pub(super) or pub(in path), and private functions. The functions of a trait impl other than its constructors form one group. Names compare as Rust strings. The function is reported at its name, with the first earlier function it belongs before.",
    name: "MethodOutOfOrder",
    purpose: "Constructors come first and wider visibility before narrower, so the interface of a type reads from the top and each function is found by its name. Move the function before the function the message names.",
    summary: "An impl function that follows one it belongs before",
};

pub const MISPLACED_BLANKET_IMPL: IssueText = IssueText {
    definition: "An impl of a trait of this crate for a type parameter, outside the file that defines the trait.",
    name: "MisplacedBlanketImpl",
    purpose: "A blanket impl reaches every type its bounds admit, so it belongs beside its trait. Move the impl into the file that defines the trait.",
    summary: "A blanket impl outside the file of its trait",
};

pub const MISPLACED_FOREIGN_TYPE_IMPL: IssueText = IssueText {
    definition: "An impl of a trait of this crate for a self type this crate does not define, other than a trait object or a type parameter, outside the file that defines the trait.",
    name: "MisplacedForeignTypeImpl",
    purpose: "An impl of a trait of this crate for a type it does not define belongs beside the trait, so every impl has exactly one place. Move the impl into the file that defines the trait.",
    summary: "A local trait's impl for a foreign type outside the trait's file",
};

pub const MISPLACED_IMPL: IssueText = IssueText {
    definition: "An impl block outside its single home, which is, in this order, the file of the struct, enum or union of its crate it implements, the file of the trait of its crate whose trait object it implements, the file of the trait of its crate it implements for any other self type, including a type parameter, and, for another crate's trait, the file of the single struct, enum or union of its crate among the trait's type arguments. This issue covers the impls whose home is the file of a struct, enum or union, the one they implement or the one among the type arguments of another crate's trait; the other homes have issues of their own.",
    name: "MisplacedImpl",
    purpose: "All behavior of a type is read in one place, the file that defines it. Move the impl block into that file.",
    summary: "An impl block outside the file of the type it belongs to",
};

pub const MISPLACED_TRAIT_OBJECT_IMPL: IssueText = IssueText {
    definition: "An impl for the trait object of a trait of this crate, outside the file that defines the trait.",
    name: "MisplacedTraitObjectImpl",
    purpose: "A trait object's behavior belongs beside its trait. Move the impl into the file that defines the trait.",
    summary: "An impl for a trait object outside the file of its trait",
};

pub const MODULE_CYCLE: IssueText = IssueText {
    definition: "Modules of a crate target that depend on each other, where a module depends on another when its own code names an item the other defines, and the paths written in a module declared inline inside a block are dependencies of the own code of the module that holds the block. Each dependency counts between the two sibling modules under their closest common ancestor, and the issue lists the dependencies that form every edge of the cycle.",
    name: "ModuleCycle",
    purpose: "Modules that depend on each other cannot be understood or changed in isolation. Break the cycle so the modules form layers.",
    summary: "Modules that depend on each other",
};

pub const MODULE_FILE_INSIDE_BLOCK: IssueText = IssueText {
    definition: "A module file declared inside a block, which is not analysed.",
    name: "ModuleFileInsideBlock",
    purpose: "A module file declared inside a block is outside the analysed module tree. Declare the module at module level.",
    summary: "A module file declared inside a block",
};

pub const MODULE_NOT_NAMED_AFTER_ITS_ITEM: IssueText = IssueText {
    definition: "A module other than a crate root whose file, or whose name for an inline module or the directory of a mod.rs, does not spell the words of its public item in order and in lowercase, each joined to the next by an underscore or by nothing. The words of a name are split at its underscores, before an uppercase letter that follows a lowercase letter or a digit, and before the last letter of an uppercase run that a lowercase letter follows, with digits kept in the word before them, so that pub struct OpenAIApiPath lives in open_ai_api_path.rs, openai_api_path.rs or openaiapipath.rs, while Ministral3ClusterParams does not live in ministral_3_cluster_params.rs.",
    name: "ModuleNotNamedAfterItsItem",
    purpose: "A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the words of its public item, keeping each word whole and joining it to the next by an underscore or by nothing, or rename the item.",
    summary: "A module that is not named after its public item",
};

pub const NEGATED_CONFIGURATION: IssueText = IssueText {
    definition: "A cfg or cfg_attr predicate that negates a feature, test or debug_assertions. Code that the predicate leaves active on no declared target is listed within its issue, and the predicate of a module file that its own attributes configure out is reported at the declaration unless an analysis of that same declaration reports it where it is written.",
    name: "NegatedConfiguration",
    purpose: "Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.",
    summary: "A cfg predicate that negates a feature, test or debug_assertions",
};

pub const NON_CONSTANT_IN_CONSTANTS_MODULE: IssueText = IssueText {
    definition: "A public item other than a constant or a module inside the crate's constants module, crate::constants.",
    name: "NonConstantInConstantsModule",
    purpose: "The constants module is the single home of public constants. Move every other public item out of it.",
    summary: "A public item other than a constant in crate::constants",
};

pub const NON_DOC_COMMENT: IssueText = IssueText {
    definition: "A comment other than a doc comment, in a module file or in a file it includes.",
    name: "NonDocComment",
    purpose: "Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an API.",
    summary: "A comment that is not a doc comment",
};

pub const NON_TEST_ITEM_IN_TEST_TARGET: IssueText = IssueText {
    definition: "An item of an integration test target other than a test, a constant, an import, a module or a private item that only the file defining it uses, among the items written in module files. A test is a function under the built-in test, bench or test_case attribute, and a target with harness = false enters through the main function of its crate root. The impl blocks of a flagged type are listed within the type's issue.",
    name: "NonTestItemInTestTarget",
    purpose: "A helper used beyond its own test file is a test library, and test libraries never live in a tests directory. Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.",
    summary: "An integration test item that is neither a test nor a file-local helper",
};

pub const PACKAGE_DEPENDENCY_CYCLE: IssueText = IssueText {
    definition: "Packages that depend on each other through dev-dependencies, each named with its version, where dependencies on packages without a library are ignored as cargo ignores them.",
    name: "PackageDependencyCycle",
    purpose: "Package dependencies never form a cycle, dev-dependencies included. When a test library depends on the package whose tests use it, move those tests and the helpers that need the package into a separate tests package that holds them in its library.",
    summary: "Packages that depend on each other through dev-dependencies",
};

pub const PACKAGE_DEPENDS_ON_ITSELF: IssueText = IssueText {
    definition: "A package that depends on itself through dev-dependencies.",
    name: "PackageDependsOnItself",
    purpose: "A package depending on itself is built twice and blurs what it owns. Remove the self-dependency.",
    summary: "A package that depends on itself through dev-dependencies",
};

pub const PROCEDURAL_FUNCTION: IssueText = IssueText {
    definition: "A function without a self receiver, free or associated, that takes a struct, enum or union of its own crate by value or by reference, unless it belongs to a trait impl, constructs the self type of its impl, directly, as the success of a Result or as the value of an Option, an async function by the output of its future, or is a private helper used only in the file that defines it.",
    name: "ProceduralFunction",
    purpose: "Behavior belongs with the data it works on. Make the function a method of the type it takes, a constructor of its own type, or a private helper used in a single file.",
    summary: "A function that takes a type of its crate without belonging to it",
};

pub const PUBLIC_ASSOCIATED_CONSTANT: IssueText = IssueText {
    definition: "A public associated constant of an inherent impl, among the items written in module files.",
    name: "PublicAssociatedConstant",
    purpose: "Public constants have a single home, crate::constants. Move the constant there as a free constant.",
    summary: "A public associated constant of an inherent impl",
};

pub const PUBLIC_CONSTANT_OUTSIDE_CONSTANTS_MODULE: IssueText = IssueText {
    definition: "A public constant outside the crate's constants module, crate::constants, among the items written in module files.",
    name: "PublicConstantOutsideConstantsModule",
    purpose: "Public constants have a single home, crate::constants. Move the constant there.",
    summary: "A public constant outside crate::constants",
};

pub const PUBLISHABLE_TEST_CRATE: IssueText = IssueText {
    definition: "A package that test_crates lists but that can be published. A test crate needs publish = false, so the package is analysed as production code.",
    name: "PublishableTestCrate",
    purpose: "A test crate holds only what tests use, so it is never published. Set publish = false in its manifest, or remove it from test_crates.",
    summary: "A declared test crate that can be published",
};

pub const REEXPORT: IssueText = IssueText {
    definition: "A re-export.",
    name: "Reexport",
    purpose: "Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it.",
    summary: "A re-export",
};

pub const SHARED_MODULE_FILE: IssueText = IssueText {
    definition: "A module file shared by several modules, or by two modules of one crate, which leaves the second unanalysed. A shared module subtree is reported at its root file.",
    name: "SharedModuleFile",
    purpose: "A file included as several modules has no single owner. Give each module its own file.",
    summary: "A module file included as several modules",
};

pub const SINGLE_LETTER_NAME: IssueText = IssueText {
    definition: "A lifetime, type or const parameter, those of a for<...> binder included, or a name in the pattern of a function, closure or function pointer parameter, whose name is a single character.",
    name: "SingleLetterName",
    purpose: "A single character says nothing about what a name stands for. Name it after its meaning, such as 'database for a lifetime or TNode for a type parameter.",
    summary: "A parameter name of a single character",
};

pub const TEST_CRATE_DECLARED_TWICE: IssueText = IssueText {
    definition: "A package that test_crates lists more than once.",
    name: "TestCrateDeclaredTwice",
    purpose: "Each test crate is declared once. Remove the repeated entry from test_crates.",
    summary: "A test crate listed more than once",
};

pub const TEST_CRATE_NOT_A_MEMBER: IssueText = IssueText {
    definition: "A test_crates entry that names no workspace member.",
    name: "TestCrateNotAMember",
    purpose: "Only a workspace member can be a test crate. Remove the entry from test_crates, or add the package to the workspace.",
    summary: "A declared test crate that is not a workspace member",
};

pub const TEST_CRATE_USED_BY_PRODUCTION: IssueText = IssueText {
    definition: "A package that test_crates lists and that a member other than a test crate depends on through a normal or build dependency, so the package is analysed as production code. The issue is reported at the manifest of each such dependent.",
    name: "TestCrateUsedByProduction",
    purpose: "Production code never depends on test code. Make the dependency a dev-dependency, or remove the package from test_crates.",
    summary: "A declared test crate that production code depends on",
};

pub const TEST_ONLY_CODE_IN_PRODUCTION: IssueText = IssueText {
    definition: "Code that exists only under cfg(test) outside test code, reported at its outermost gated item.",
    name: "TestOnlyCodeInProduction",
    purpose: "Production code holds only what production runs. Keep test-only code in a #[cfg(test)] module, such as the inline mod tests of the file it tests, and move test libraries shared by several packages into a test crate.",
    summary: "Code that exists only under cfg(test) outside test code",
};

pub const TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES: IssueText = IssueText {
    definition: "A tests package, whose targets are integration tests beside an absent or empty library, a library being empty when it defines nothing on any declared target, and whose integration test targets hold nothing but tests, constants, imports, modules and file-local private items, so that it holds no test library of its own. Integration tests belong to the package whose behaviour they verify, in its tests directory, and may use any dev-dependency; a helper used beyond its own test file is a test library, which belongs in a library crate; a separate tests package exists only to hold test libraries of its own, such as helpers that need the package under test, which that package cannot dev-depend on without a package cycle. The issue lists every workspace member the package depends on, either as a package that could take its tests or with the dependency cycle that moving them there would close, as guidance, not as the owner.",
    name: "TestsOnlyPackageWithoutTestLibraries",
    purpose: "Tests belong to the package whose behaviour they verify, in its tests directory. A separate tests package is justified only by test libraries of its own: shared helpers, which never live in a tests directory, and helpers that need the package under test, which that package cannot dev-depend on without a dependency cycle. Heisenberg cannot tell from the code which package the tests verify, so it lists every workspace member the tests use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they verify. If a test library they use depends on that package, the test library is entangled with the code it helps to test: move the helpers that need the package into this package's library, or make the test library independent of it.",
    summary: "A package of tests that holds no test library of its own",
};

pub const TUPLE_IN_SIGNATURE: IssueText = IssueText {
    definition: "A tuple type of two or more elements written in an item signature or type definition: the type of a function parameter, a return type, a field, a type alias, the type of a constant or static, an associated type, an impl header or a trait definition, nested types included. The functions and constants of a trait impl, whose types the trait dictates, and types written inside function bodies are not checked.",
    name: "TupleInSignature",
    purpose: "A tuple names none of its values, so its readers must remember what each position holds. Declare a struct with named fields instead.",
    summary: "A tuple type in an item signature",
};

pub const UNDESTRUCTURED_PARAMETER: IssueText = IssueText {
    definition: "A function or closure parameter bound by an identifier, other than self, whose every use, at least one, accesses a field of the struct or tuple the parameter holds, directly or behind references, with the field name written in a module file. Fields reached through Box or another Deref implementation, and fields of unions, do not count.",
    name: "UndestructuredParameter",
    purpose: "A parameter used only through its fields reads better as a pattern that names those fields. Destructure the parameter in its pattern.",
    summary: "A parameter used only through its fields",
};

pub const UNIMPLEMENTED_BUILTIN_MACRO: IssueText = IssueText {
    definition: "A call of a built-in macro that rust-analyzer does not implement.",
    name: "UnimplementedBuiltinMacro",
    purpose: "rust-analyzer cannot expand this built-in macro, so its output cannot be analysed. Replace the call with code that can be analysed.",
    summary: "A built-in macro that rust-analyzer cannot expand",
};

pub const UNMAPPED_ITEM: IssueText = IssueText {
    definition: "An item whose syntax rust-analyzer cannot map to its definition, such as an item of a block inside a type.",
    name: "UnmappedItem",
    purpose: "An item that cannot be mapped to its definition leaves the analysis incomplete. Write it so it maps to a definition.",
    summary: "An item that cannot be mapped to its definition",
};

pub const UNPARSABLE_MACRO_EXPANSION: IssueText = IssueText {
    definition: "A macro expansion that cannot be parsed. It is reported at the macro call written in the source, even when a macro of another crate wrote the call that produced it, listed with the macros whose expansions lead to it, the syntax errors rust-analyzer finds and the literal tokens it cannot read back, such as a negative number that a procedural macro emits as one literal token, whose sign belongs in a separate - token, and it is not analysed further.",
    name: "UnparsableMacroExpansion",
    purpose: "A macro expansion that does not parse cannot be analysed. Fix the macro so its output is valid Rust.",
    summary: "A macro expansion that does not parse",
};

pub const UNPREFIXED_TYPE_PARAMETER: IssueText = IssueText {
    definition: "A type parameter whose name of more than one character is not T followed by an uppercase letter, such as TNode. A single-character name is a SingleLetterName issue instead.",
    name: "UnprefixedTypeParameter",
    purpose: "The T prefix tells type parameters apart from concrete types. Rename the parameter to T followed by a capitalized word, such as TNode.",
    summary: "A type parameter not named T followed by a capitalized word",
};

pub const UNRESOLVED_EXTERN_CRATE: IssueText = IssueText {
    definition: "An extern crate declaration that does not resolve.",
    name: "UnresolvedExternCrate",
    purpose: resolution_purpose!("Declare the crate as a dependency or remove the declaration."),
    summary: "An extern crate that does not resolve",
};

pub const UNRESOLVED_MACRO_CALL: IssueText = IssueText {
    definition: "A macro call, attribute or derive whose macro does not resolve. A macro call whose name an unresolved import binds belongs to that import's issue.",
    name: "UnresolvedMacroCall",
    purpose: resolution_purpose!("Define or import the macro."),
    summary: "A macro call, attribute or derive whose macro does not resolve",
};

pub const UNRESOLVED_METHOD_CALL: IssueText = IssueText {
    definition: "A method call that does not resolve, unless its receiver holds a name that does not resolve, which is that name's issue.",
    name: "UnresolvedMethodCall",
    purpose: resolution_purpose!("Fix the call so its method resolves."),
    summary: "A method call that does not resolve",
};

pub const UNRESOLVED_MODULE: IssueText = IssueText {
    definition: "A module declaration whose file does not resolve.",
    name: "UnresolvedModule",
    purpose: resolution_purpose!("Create the module file or remove the declaration."),
    summary: "A module declaration whose file does not exist",
};

pub const UNRESOLVED_PATH: IssueText = IssueText {
    definition: "A path that does not resolve, imports included. A name bound by an unresolved import, whether a path or a macro call, belongs to that import's issue, and paths into a module of another crate that do not resolve are one issue per module, listed under a failed macro expansion that a workspace member reports in that module's file when there is one.",
    name: "UnresolvedPath",
    purpose: resolution_purpose!("Fix the path so it resolves."),
    summary: "A path that does not resolve",
};

pub const SUPPORTED_RUST_VERSION: &str = "1.99.1";
