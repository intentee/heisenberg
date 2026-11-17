use std::num::NonZeroU16;

use crate::issue_text::IssueText;
use crate::rule_evaluation::RuleEvaluation;
use crate::rule_scope::RuleScope;
use crate::rule_section::RuleSection;

const RESOLUTION_RATIONALE: &str =
    "The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis.";

pub const ADDITIONAL_PUBLIC_ITEM: IssueText = IssueText {
    definition: "A public item of a module beyond the one that the module's name spells, or beyond its first public item when the name spells none, among the items of module files, including those that a macro_rules! macro of the crate writes, constants, modules and the functions that Rust requires for procedural macros at the crate root aside. An item is public when its visibility reaches beyond its own module, so a `pub(self)` item is not public while a `pub` item of a private module is, and a `macro_rules!` macro is public when exported.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "AdditionalPublicItem",
    ordinal: 1,
    rationale: "Each module exposes a single public item, so every file has one responsibility.",
    remedy: "Move each additional public item into its own module.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A module with more than one public item",
};

pub const ATTRIBUTE_APPLIED_IN_NO_CONFIGURATION: IssueText = IssueText {
    definition: "A cfg_attr whose predicate, together with the predicates of the cfg_attr attributes that enclose it, holds in no declared configuration that reaches it, such as cfg_attr(rustfmt, rustfmt::skip) or cfg_attr(clippy, allow(...)). Evaluated only when every declared configuration is analysed.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "AttributeAppliedInNoConfiguration",
    ordinal: 1,
    rationale: "An attribute that applies in no declared configuration changes nothing that Heisenberg analyses, so it only changes what a tool outside the declared configurations sees.",
    remedy: "Remove the attribute, or declare a configuration it applies in.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Configurations,
    summary: "A cfg_attr that applies in no declared configuration",
};

pub const BENCHMARKS_ONLY_PACKAGE_WITHOUT_LIBRARIES: IssueText = IssueText {
    definition: "A benchmarks package, whose targets are benchmark targets beside an absent or empty library, a library being empty when it defines nothing in any declared configuration, and whose benchmark targets hold nothing but their entry points, constants, imports, modules and file-local private items, so that it holds no library of its own. Benchmarks belong to the package whose code they measure, in its benches directory, and may use any dev-dependency, test crates included; a helper used beyond its own benchmark file belongs in a library crate; a separate package exists only to hold a library of its own, such as helpers that need the measured package, which that package cannot dev-depend on without a package cycle. The issue lists every workspace member the package depends on, either as a package that could take its benchmarks or with the dependency cycle that moving them there would close, as guidance, not as the owner.",
    evaluation: RuleEvaluation::EveryDeclaredConfigurationInPart,
    exceptions: &[],
    name: "BenchmarksOnlyPackageWithoutLibraries",
    ordinal: 5,
    rationale: "Benchmarks belong to the package whose code they measure, in its benches directory, and a benchmark that needs a library which depends on that package, such as a test harness, belongs beside that library in its package.",
    remedy: "Heisenberg cannot tell from the code which package the benchmarks measure, so it lists every workspace member they use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they measure.",
    scope: RuleScope::Package,
    section: RuleSection::Benchmarks,
    summary: "A package of benchmarks that holds no library of its own",
};

pub const BENCHMARK_TARGET_EXCLUDED_FROM_BENCHMARKS: IssueText = IssueText {
    definition: "A benchmark target whose manifest sets bench = false, so cargo bench never runs it.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "BenchmarkTargetExcludedFromBenchmarks",
    ordinal: 1,
    rationale: "Every benchmark runs under cargo bench.",
    remedy: "Remove bench = false, or remove the benchmark target.",
    scope: RuleScope::Package,
    section: RuleSection::Benchmarks,
    summary: "A benchmark target that cargo bench never runs",
};

pub const BENCHMARK_TARGET_RUN_BY_TESTS: IssueText = IssueText {
    definition: "A benchmark target whose manifest sets test = true, so cargo test and nextest build and run it with the tests.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "BenchmarkTargetRunByTests",
    ordinal: 2,
    rationale: "Benchmarks measure, and the test run never runs them.",
    remedy: "Remove test = true, and verify the behaviour that a benchmark relies on in a test.",
    scope: RuleScope::Package,
    section: RuleSection::Benchmarks,
    summary: "A benchmark target that the test run runs",
};

pub const BENCHMARK_TARGET_WITHOUT_CRITERION_BENCHMARK: IssueText = IssueText {
    definition: "A benchmark target with harness = false whose module files call none of criterion's benchmark registrations, Criterion::bench_function, Criterion::bench_with_input, BenchmarkGroup::bench_function and BenchmarkGroup::bench_with_input, criterion being the criterion package from crates.io, so that a path, git or alternative registry package named criterion is not criterion. It is reported at the start of the target's crate root, and not when the target's code holds an unresolved path, method call, macro call or macro expansion, whose issue is reported instead.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "BenchmarkTargetWithoutCriterionBenchmark",
    ordinal: 4,
    rationale: "Heisenberg analyses benchmarks through criterion, the benchmark harness whose measurement it understands, so a benchmark measured any other way escapes its rules.",
    remedy: "Register every benchmark of the target with criterion in the target's own module files.",
    scope: RuleScope::CrateTarget,
    section: RuleSection::Benchmarks,
    summary: "A benchmark target that registers no criterion benchmark",
};

pub const BENCHMARK_TARGET_WITH_LIBTEST_HARNESS: IssueText = IssueText {
    definition: "A benchmark target built with libtest's harness, harness = true, including a file of the benches directory that cargo discovers without a [[bench]] table. libtest's #[bench] is available only on the nightly channel, so on stable Rust such a target holds no benchmark, only tests that the test run never builds.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "BenchmarkTargetWithLibtestHarness",
    ordinal: 3,
    rationale: "A benchmark target measures through criterion from its own entry point.",
    remedy: "Declare the target in a [[bench]] table with harness = false and register its benchmarks with criterion, and move its tests into a tests directory.",
    scope: RuleScope::Package,
    section: RuleSection::Benchmarks,
    summary: "A benchmark target built with libtest's harness",
};

pub const CLIPPY_CONFIGURATION_AMBIGUOUS: IssueText = IssueText {
    definition: "A directory that holds both clippy configuration files and is the nearest one with a clippy configuration for a workspace member, within the workspace.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ClippyConfigurationAmbiguous",
    ordinal: 1,
    rationale: "Clippy reads only one of clippy.toml and .clippy.toml, so keeping both hides which applies.",
    remedy: "Keep one of them.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A directory that holds both clippy configuration files",
};

pub const CLIPPY_CONFIGURATION_MISSING: IssueText = IssueText {
    definition: "A workspace member with no clippy configuration file in its directory or an ancestor within the workspace.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ClippyConfigurationMissing",
    ordinal: 2,
    rationale: "Tests may fail fast while production code may not, and clippy needs a configuration to tell them apart.",
    remedy: "Add a clippy.toml with the test allowances.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A workspace member without a clippy configuration",
};

pub const CODE_ACTIVE_IN_NO_CONFIGURATION: IssueText = IssueText {
    definition: "Code active in no declared configuration, a configuration being a supported target with a selection of features. A module file that its own attributes configure out is inactive code at its declaration. Code that a negated predicate leaves active in no declared configuration is listed within the predicate's NegatedConfiguration issue instead, and a compile_error! call, which exists only to fail the builds no configuration declares, is not inactive code.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "CodeActiveInNoConfiguration",
    ordinal: 2,
    rationale: "Code active in no declared configuration is never built or analysed.",
    remedy: "Remove it, declare a configuration it is active in, or make it active in a declared configuration.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Configurations,
    summary: "Code that no declared configuration builds",
};

pub const CODE_WITHOUT_COVERAGE_DATA: IssueText = IssueText {
    definition: "A measured function, one with a body whose opening brace is production code as UncoveredCode defines it, written directly or by a macro, that no region of the coverage export falls into, because no instrumented build of the measured run compiled it.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "CodeWithoutCoverageData",
    ordinal: 1,
    rationale: "Every production function is measured.",
    remedy: "Build and run the target that holds the function under the instrumented tests, and keep the function instrumented: rustc records no coverage for #[automatically_derived] code, and cfg(coverage) leaves code out of the instrumented build.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Coverage,
    summary: "A production function without coverage data",
};

pub const CONFIGURATION_SELECTION_MACRO: IssueText = IssueText {
    definition: "A cfg_select! call, whose unselected arms escape the analysis.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ConfigurationSelectionMacro",
    ordinal: 1,
    rationale: "cfg_select! discards the arms it does not select before they can be analysed.",
    remedy: "Write each arm as an item under its own #[cfg] attribute.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A cfg_select! call",
};

pub const CONFINED_PACKAGE_REACHED_OUTSIDE_ITS_USERS: IssueText = IssueText {
    definition: "A workspace member that confined_packages does not list as a user of a package, while cargo builds the package for the member in an analysed configuration: through the normal and build dependencies of every package on the way, other workspace members and third-party packages alike, and through the member's own dev-dependencies, starting from the side its own targets build on, the host for a procedural macro crate and the target otherwise, each dependency on the target or the host where cargo builds it; a member never reaches its own package. The issue is reported once per member and package at the manifest of the member, with a shortest dependency path to the package, and names the configurations it holds in when it does not hold in every analysed one.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ConfinedPackageReachedOutsideItsUsers",
    ordinal: 1,
    rationale: "A confined package is built and used only by its declared users, so nothing else reaches what it provides, even through another member.",
    remedy: "Remove the dependency that the path names; code that needs the package belongs in one of its users.",
    scope: RuleScope::Workspace,
    section: RuleSection::Packages,
    summary: "A member that reaches a confined package without being one of its users",
};

pub const CONFINED_PACKAGE_USER_LISTED_TWICE: IssueText = IssueText {
    definition: "A name that confined_packages lists more than once among the users of one package.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ConfinedPackageUserListedTwice",
    ordinal: 2,
    rationale: "Each user of a confined package is listed once.",
    remedy: "Remove the repeated entry from confined_packages.",
    scope: RuleScope::Workspace,
    section: RuleSection::Packages,
    summary: "A confined package user listed more than once",
};

pub const CONFINED_PACKAGE_USER_NOT_A_MEMBER: IssueText = IssueText {
    definition: "A user of a confined package, in confined_packages, that names no workspace member.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ConfinedPackageUserNotAMember",
    ordinal: 3,
    rationale: "Only a workspace member can use a confined package.",
    remedy: "Remove the entry from confined_packages, or add the package to the workspace.",
    scope: RuleScope::Workspace,
    section: RuleSection::Packages,
    summary: "A confined package user that is not a workspace member",
};

pub const CONFINED_PACKAGE_USER_NOT_REACHING_IT: IssueText = IssueText {
    definition: "A workspace member that confined_packages lists as a user of a package that cargo builds for it in no declared configuration, reported at the workspace manifest. A partial run does not evaluate it.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "ConfinedPackageUserNotReachingIt",
    ordinal: 4,
    rationale: "The users of a confined package are exactly the members that reach it, so the list states where the package is used.",
    remedy: "Remove the member from the users of the package.",
    scope: RuleScope::Workspace,
    section: RuleSection::Packages,
    summary: "A confined package user that never reaches the package",
};

pub const DOC_TESTS_ENABLED: IssueText = IssueText {
    definition: "A library or procedural macro target of a workspace member whose doc tests are enabled, as cargo decides it from the doctest key of the target, which is on unless the manifest turns it off. Whether a documentation comment holds a code block is not examined.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "DocTestsEnabled",
    ordinal: 17,
    rationale: "A doc test is Rust code that cargo test compiles and runs out of a documentation comment, where it is text that no analysis reads, so a target with doc tests enabled can hold code that escapes every rule.",
    remedy: "Set doctest = false in the [lib] table of the package, and keep code that must compile in an example or a test, which are analysed.",
    scope: RuleScope::Package,
    section: RuleSection::Analysis,
    summary: "A target whose doc tests are enabled",
};

pub const DUPLICATED_FUNCTION_BODY: IssueText = IssueText {
    definition: "Functions written in the module files of workspace members whose signatures and bodies are identical once every name is replaced by the definition it resolves to, with names a function declares itself compared by declaration order and names that resolve to nothing never equal; a format or assembly template compares by its written text and by the definitions its placeholders capture, and the name of a named format argument by its spelling. Functions with an empty body are not compared, and neither are the entry points that Rust requires one of per target or per macro: the main function of a binary, a build script, an example, or a test or benchmark target without libtest's harness, and the functions of procedural macros.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "DuplicatedFunctionBody",
    ordinal: 1,
    rationale: "Duplicated code drifts apart when one copy changes.",
    remedy: "Keep one function and call it from every place that needs it.",
    scope: RuleScope::Workspace,
    section: RuleSection::Code,
    summary: "Functions whose signatures and bodies are identical",
};

pub const ERROR_MESSAGE_FUNCTION: IssueText = IssueText {
    definition: "An error attribute of thiserror, on an error enum or one of its variants, that names a function to format the message, as #[error(fmt = path)] does.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ErrorMessageFunction",
    ordinal: 1,
    rationale: "The message of an error variant is read where the variant is declared, and a function hides it from the rules that check messages.",
    remedy: "Write the message as a format string in the #[error] attribute of the variant.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error message formatted by a function",
};

pub const ERROR_MESSAGE_ON_THE_ENUM: IssueText = IssueText {
    definition: "An error attribute of thiserror written on an error enum itself, which gives every variant without an attribute of its own the same message.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ErrorMessageOnTheEnum",
    ordinal: 2,
    rationale: "Each failure has its own message, written on its variant.",
    remedy: "Move the message to the variants, one #[error] attribute each.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error message written on the enum instead of its variants",
};

pub const ERROR_SOURCE_OUTSIDE_CHAIN: IssueText = IssueText {
    definition: "A field of a variant of an enum that derives thiserror::Error whose type implements std::error::Error, that is not the source of its variant, and that its variant does not report beside a source: the variant has no source, or its #[error(...)] message does not format the field. The source is as thiserror decides it: the field marked #[source] or #[from], otherwise the field named source, or the single field of an #[error(transparent)] variant.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[
        "A field that holds a Vec of errors, which accumulates failures: the source chain has room for one cause, and a collection cannot be a source.",
    ],
    name: "ErrorSourceOutsideChain",
    ordinal: 3,
    rationale: "An error kept outside the source never reaches the error chain, so whoever reports the error loses its cause.",
    remedy: "Make the field the source of its variant, with #[source] or #[from]. Where a failure has several causes, keep one as the source and format each other one in the message of the variant.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error held outside the source chain",
};

pub const ERROR_TYPE_NOT_AN_ENUM: IssueText = IssueText {
    definition: "An implementation of std::error::Error that the Error derive of thiserror generates for a type that is not an enum, such as a struct. thiserror is the thiserror-impl package from crates.io, whose Error derive generates the implementation.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ErrorTypeNotAnEnum",
    ordinal: 4,
    rationale: "A type that is not an enum cannot name which failure happened, so its callers cannot match on the failure.",
    remedy: "Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error type deriving thiserror::Error that is not an enum",
};

pub const ERROR_TYPE_OUTSIDE_ITS_MODULE: IssueText = IssueText {
    definition: "An enum that derives thiserror's Error and is defined in a crate root, in an inline module, or in a module file whose name does not spell the name of the enum. An error type inside an item that cfg(test) gates, which has no module file of its own, is not checked; the error types of integration test targets and test crates are.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ErrorTypeOutsideItsModule",
    ordinal: 5,
    rationale: "The error type of a crate lives in a module file of its own, found from its name alone.",
    remedy: "Move the enum into a module file named after it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error type outside a module file named after it",
};

pub const FAILED_MACRO_EXPANSION: IssueText = IssueText {
    definition: "A macro call whose expansion fails. Every failing expansion is listed within the macro call written in the source.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "FailedMacroExpansion",
    ordinal: 2,
    rationale: "A macro call that fails to expand produces no code to analyse.",
    remedy: "Fix the call so it expands.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A macro call that fails to expand",
};

pub const GLOB_IMPORT: IssueText = IssueText {
    definition: "A glob import.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "GlobImport",
    ordinal: 2,
    rationale: "Glob imports hide where names come from.",
    remedy: "Import each name explicitly.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A glob import",
};

pub const HANDWRITTEN_ERROR_DISPLAY: IssueText = IssueText {
    definition: "A hand-written implementation of Display for a type that implements std::error::Error, which thiserror accepts when the type carries no #[error] attribute.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "HandwrittenErrorDisplay",
    ordinal: 6,
    rationale: "A message written in a Display implementation escapes the rules that check error messages, and is not read where the variant is declared.",
    remedy: "Remove the implementation and give each variant an #[error] attribute.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error type that formats its message by hand",
};

pub const HANDWRITTEN_ERROR_TYPE: IssueText = IssueText {
    definition: "An implementation of std::error::Error written by hand in a module file, for a type of any kind.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "HandwrittenErrorType",
    ordinal: 7,
    rationale: "An error type written by hand repeats what thiserror generates, its messages, source chain and conversions, and drifts from it.",
    remedy: "Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error type implemented by hand",
};

pub const IGNORED_TEST: IssueText = IssueText {
    definition: "An ignore attribute, also one that a cfg_attr applies, written in the workspace or generated by one of its macros, which makes the test run skip its test.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "IgnoredTest",
    ordinal: 1,
    rationale: "A skipped test looks like part of the suite while it verifies nothing.",
    remedy: "Make the test pass, or remove it together with what it was meant to verify.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "A test that the test run skips",
};

pub const IMPL_WITHOUT_LOCAL_TYPE: IssueText = IssueText {
    definition: "An impl of another crate's trait for a self type this crate does not define, with no single struct, enum or union of this crate among the trait's type arguments.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ImplWithoutLocalType",
    ordinal: 3,
    rationale: "An impl of another crate's trait belongs beside the one type of this crate it serves.",
    remedy: "Implement the trait for a struct, enum or union of this crate, or for a foreign type with exactly one of them among the trait's type arguments.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "An impl of another crate's trait with no home in this crate",
};

pub const IMPORT_INSIDE_BLOCK: IssueText = IssueText {
    definition: "A use declaration written inside a block, such as a function body.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ImportInsideBlock",
    ordinal: 4,
    rationale: "The imports of a file are read in one place, at its top, and a name means the same thing throughout the file.",
    remedy: "Move the import to the top of the module.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "An import written inside a block",
};

pub const INCLUDED_SOURCE_FILE: IssueText = IssueText {
    definition: "An include! of a file that no build script provides: one outside the OUT_DIR of its crate, which a build script writes or an out_dirs declaration stands in for. Included files are trusted as generated text and not analysed.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "IncludedSourceFile",
    ordinal: 3,
    rationale: "An included file is trusted as generated text, so code written by hand escapes every rule once it is included.",
    remedy: "Make the file a module, or have a build script generate it into OUT_DIR.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "An include! of a file that is not a build script output",
};

pub const INVALID_MACRO_DEFINITION: IssueText = IssueText {
    definition: "A macro definition that rust-analyzer reports as invalid.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "InvalidMacroDefinition",
    ordinal: 4,
    rationale: "An invalid macro definition produces no code to analyse.",
    remedy: "Fix the definition.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A macro definition that is not valid",
};

pub const ITEM_MORE_VISIBLE_THAN_ITS_USES: IssueText = IssueText {
    definition: "A function, inherent method, struct, enum, union, constant, static, type alias or field of a package that cannot be published, whose visibility reaches outside its crate, that no code of another crate target refers to in any declared configuration, and that its own crate names only inside function bodies and imports. A reference inside the definition of a type, its impl blocks and the code its derives generate does not count for the type. Evaluated only when every declared configuration is analysed.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[
        "An item of a package that can be published, whose users are outside the workspace.",
        "An item that its crate names in a signature, a type definition or an impl header, which an exported interface may need.",
        "A trait, a trait item, an enum variant and a module, whose visibility is not their own or is not narrowed alone.",
    ],
    name: "ItemMoreVisibleThanItsUses",
    ordinal: 17,
    rationale: "An unpublishable package has every user inside the workspace, so an item exported to crates that never use it declares an interface that does not exist: it hides which items are the crate's surface, and keeps the compiler from reporting the item once nothing in its crate uses it either.",
    remedy: "Narrow the visibility to pub(crate), or remove the item when its own crate does not use it.",
    scope: RuleScope::Workspace,
    section: RuleSection::Modules,
    summary: "An exported item that no other crate target uses",
};

pub const ITEM_OUT_OF_ORDER: IssueText = IssueText {
    definition: "An item of a module file or inline module, among the items written in module files, that follows an item of a later group. The groups, in order: module declarations; imports from vendor crates, which are std, core, alloc and every crate outside the workspace; imports from other workspace members; imports from this crate; private functions, constants, statics, macro_rules! macros and macro calls; private structs, enums, unions, type aliases and traits with their impl blocks; public items with their impl blocks, and the main function that its crate target enters through; tests, the functions under the built-in test, bench or test_case attribute; inline modules. An import, like an extern crate declaration, takes the group of the crate that the first segment of each of its paths resolves to, except that an import of a macro_rules! macro of its own module, which Rust allows only after the macro, takes the group of the macro; an impl block takes the group of the type or trait it belongs to, the items of an extern block take their places one by one, and an item is public when its visibility reaches beyond its own module. The item is reported at its name, with the first earlier item of a later group.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ItemOutOfOrder",
    ordinal: 5,
    rationale: "Every file reads in the same order, from what it declares and imports down to what it exposes.",
    remedy: "Move the item before the item the message names. A module that calls a macro_rules! macro of its parent by its bare name compiles only when it is declared after the macro, so declare the module first and hand it the macro by path: re-export the macro after its definition with pub(crate) use, and import it in the module.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "An item that follows an item of a later group",
};

pub const ITEM_USED_ONLY_BY_BENCHMARKS: IssueText = IssueText {
    definition: "A function, a method outside trait impls, a struct, enum, union, constant, static or type alias, an enum variant or a named field, outside test and benchmark code, that is referenced and whose references all lie in test or benchmark code, at least one of them in benchmark code, benchmark code being every benchmark target and test code being every integration test target, every target of a test crate and every item that cfg(test) gates. References inside the definition of a type, its impl blocks and the code its derives generate do not count for the type; a variant counts where it is constructed, since matching it is forced by exhaustiveness; a field counts where it is read, a struct update expression reading every field it leaves out. Items of trait declarations and trait impls, traits and positional fields are not checked, because their uses do not name them. Every item of a package with publish = false is checked; in a package that can be published, only those its crate does not expose to other crates.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ItemUsedOnlyByBenchmarks",
    ordinal: 6,
    rationale: "Production code holds only what production uses.",
    remedy: "Move an item that only benchmarks use, or benchmarks and tests, into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.",
    scope: RuleScope::Workspace,
    section: RuleSection::Benchmarks,
    summary: "A production item that only benchmarks use",
};

pub const ITEM_USED_ONLY_BY_TESTS: IssueText = IssueText {
    definition: "A function, a method outside trait impls, a struct, enum, union, constant, static or type alias, an enum variant or a named field, outside test and benchmark code, that is referenced and whose references all lie in test code, test code being every integration test target, every target of a test crate and every item that cfg(test) gates. References inside the definition of a type, its impl blocks and the code its derives generate do not count for the type; a variant counts where it is constructed, since matching it is forced by exhaustiveness; a field counts where it is read, a struct update expression reading every field it leaves out. Items of trait declarations and trait impls, traits and positional fields are not checked, because their uses do not name them. Every item of a package with publish = false is checked; in a package that can be published, only those its crate does not expose to other crates.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ItemUsedOnlyByTests",
    ordinal: 2,
    rationale: "Production code holds only what production uses.",
    remedy: "Move an item that only tests use into the test module that uses it, or into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A production item that only tests use",
};

pub const LINT_ALLOWED_IN_MANIFEST: IssueText = IssueText {
    definition: "A workspace member whose lints, after workspace inheritance, allow any lint of any tool. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "LintAllowedInManifest",
    ordinal: 3,
    rationale: "Allowing a lint is a suppression, and suppressions are a slippery slope.",
    remedy: "Remove the allow; if the lint then reports code, fix that code.",
    scope: RuleScope::Package,
    section: RuleSection::Tools,
    summary: "A manifest lints table that allows a lint",
};

pub const LINT_SUPPRESSION: IssueText = IssueText {
    definition: "An allow or expect attribute of any lint, or a warn attribute that lowers a required lint or lint group, that the workspace writes, in module files, through a cfg_attr, through a macro of a workspace member or as output of a procedural macro of a workspace member, over code that holds text written in the module files of workspace members once its macros are expanded.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[
        "A suppression that a vendored macro writes.",
        "A suppression over code that only a generator wrote, such as bindings that a build script writes and include! brings in.",
    ],
    name: "LintSuppression",
    ordinal: 4,
    rationale: "Suppressing lints is a slippery slope.",
    remedy: "Remove the attribute and fix the code the lint reports.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tools,
    summary: "An attribute that suppresses or lowers lints over code written in the workspace",
};

pub const MACRO_GENERATED_ERROR_TYPE: IssueText = IssueText {
    definition: "An implementation of std::error::Error that a macro other than the Error derive of thiserror generates, whether a macro_rules! macro or a procedural macro. thiserror is the thiserror-impl package from crates.io, whose Error derive generates the implementation.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MacroGeneratedErrorType",
    ordinal: 8,
    rationale: "The error type rules read the source and the message of each failure from what the Error derive of thiserror declares, and another macro declares neither where they can be checked.",
    remedy: "Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error type implemented by a macro other than thiserror::Error",
};

pub const METHOD_OUT_OF_ORDER: IssueText = IssueText {
    definition: "A function of an impl block or of a trait definition, among the items written in module files, that follows a function it belongs before. The functions of an inherent impl and of a trait definition form groups, in order: constructors, which return Self, Option<Self> or Result<Self, _>, an async function by the output of its future, without a receiver or with a self receiver taken by value, by visibility, pub, pub(crate), pub(super) or pub(in path), and private; then the other functions, by the same visibilities. The functions of a trait definition are public. Within a group, names compare as Rust strings. The functions of a trait impl follow the order in which their trait declares them. The function is reported at its name, with the first earlier function it belongs before.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MethodOutOfOrder",
    ordinal: 6,
    rationale: "Constructors come first and wider visibility before narrower, so the interface of a type reads from the top and each function is found by its name, and a trait impl reads in the order of its trait.",
    remedy: "Move the function before the function the message names.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A function of an impl block or trait that follows one it belongs before",
};

pub const MISPLACED_BLANKET_IMPL: IssueText = IssueText {
    definition: "An impl of a trait of this crate for a type parameter, outside the file that defines the trait.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MisplacedBlanketImpl",
    ordinal: 7,
    rationale: "A blanket impl reaches every type its bounds admit, so it belongs beside its trait.",
    remedy: "Move the impl into the file that defines the trait.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A blanket impl outside the file of its trait",
};

pub const MISPLACED_FOREIGN_TYPE_IMPL: IssueText = IssueText {
    definition: "An impl of a trait of this crate for a self type this crate does not define, other than a trait object or a type parameter, outside the file that defines the trait.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MisplacedForeignTypeImpl",
    ordinal: 8,
    rationale: "An impl of a trait of this crate for a type it does not define belongs beside the trait, so every impl has exactly one place.",
    remedy: "Move the impl into the file that defines the trait.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A local trait's impl for a foreign type outside the trait's file",
};

pub const MISPLACED_IMPL: IssueText = IssueText {
    definition: "An impl block outside its single home, which is, in this order, the file of the struct, enum or union of its crate it implements, the file of the trait of its crate whose trait object it implements, the file of the trait of its crate it implements for any other self type, including a type parameter, and, for another crate's trait, the file of the single struct, enum or union of its crate among the trait's type arguments. This issue covers the impls whose home is the file of a struct, enum or union, the one they implement or the one among the type arguments of another crate's trait; the other homes have issues of their own.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MisplacedImpl",
    ordinal: 9,
    rationale: "All behavior of a type is read in one place, the file that defines it.",
    remedy: "Move the impl block into that file.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "An impl block outside the file of the type it belongs to",
};

pub const MISPLACED_TRAIT_OBJECT_IMPL: IssueText = IssueText {
    definition: "An impl for the trait object of a trait of this crate, outside the file that defines the trait.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MisplacedTraitObjectImpl",
    ordinal: 10,
    rationale: "A trait object's behavior belongs beside its trait.",
    remedy: "Move the impl into the file that defines the trait.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "An impl for a trait object outside the file of its trait",
};

pub const MODULE_CYCLE: IssueText = IssueText {
    definition: "Modules of a crate target that depend on each other, where a module depends on another when its own code names an item the other defines, and the paths written in a module declared inline inside a block are dependencies of the own code of the module that holds the block. Each dependency counts between the two sibling modules under their closest common ancestor, and the issue lists the dependencies that form every edge of the cycle.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ModuleCycle",
    ordinal: 11,
    rationale: "Modules that depend on each other cannot be understood or changed in isolation.",
    remedy: "Break the cycle so the modules form layers.",
    scope: RuleScope::CrateTarget,
    section: RuleSection::Modules,
    summary: "Modules that depend on each other",
};

pub const MODULE_FILE_INSIDE_BLOCK: IssueText = IssueText {
    definition: "A module file declared inside a block, which is not analysed.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ModuleFileInsideBlock",
    ordinal: 5,
    rationale: "A module file declared inside a block is outside the analysed module tree.",
    remedy: "Declare the module at module level.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A module file declared inside a block",
};

pub const MODULE_NOT_NAMED_AFTER_ITS_ITEM: IssueText = IssueText {
    definition: "A module other than a crate root whose file, or whose name for an inline module or the directory of a mod.rs, spells the words of none of its public items, reported at its first public item. A module spells the words of an item in order and in lowercase, each joined to the next by an underscore or by nothing. The words of a name are split at its underscores, before an uppercase letter that follows a lowercase letter or a digit, and before the last letter of an uppercase run that a lowercase letter follows, with digits kept in the word before them, so that pub struct OpenAIApiPath lives in open_ai_api_path.rs, openai_api_path.rs or openaiapipath.rs, while Ministral3ClusterParams does not live in ministral_3_cluster_params.rs.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ModuleNotNamedAfterItsItem",
    ordinal: 12,
    rationale: "A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone.",
    remedy: "Rename the module to the words of its public item, keeping each word whole and joining it to the next by an underscore or by nothing, or rename the item.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A module that is not named after its public item",
};

pub const MULTIPLE_ERROR_TYPES: IssueText = IssueText {
    definition: "A crate target that implements std::error::Error more than once, by hand or through derives and other macros, each implementation reported where it is written.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[
        "An implementation in test code: one in a module that only the test configuration enables, in an integration test target or in a declared test crate.",
    ],
    name: "MultipleErrorTypes",
    ordinal: 9,
    rationale: "A crate whose failures one error enum cannot describe does more than one thing, and its callers cannot tell from its signatures which concern failed.",
    remedy: "Split the crate along its error boundaries: move each error enum, with the code that fails with it, into its own crate, and wrap it in the crates that depend on it as a #[source] or #[from] variant of their own error enum.",
    scope: RuleScope::CrateTarget,
    section: RuleSection::Errors,
    summary: "A crate with more than one error type",
};

pub const MUST_USE_BENCHMARK_OUTPUT: IssueText = IssueText {
    definition: "A call or reference, in the code of a workspace member, to one of criterion's measurement loops that black-box and drop what the measured routine returns, Bencher::iter, Bencher::iter_batched, Bencher::iter_batched_ref, Bencher::iter_with_large_drop and their AsyncBencher counterparts, whose routine returns a must-use value: a value of a type whose definition carries #[must_use], such as Result, of an impl Trait or dyn Trait type whose trait or one of its supertraits carries #[must_use], such as Future or Iterator, a closure, or a Box, Pin, tuple or non-empty array holding one. The returned type is the loop's output type parameter as rust-analyzer infers it at the call, and it is reported at the name of the loop.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "MustUseBenchmarkOutput",
    ordinal: 7,
    rationale: "A must-use value that criterion drops hides an error or work that never ran, so the benchmark can measure a failure path or the construction of lazy work instead of the code.",
    remedy: "Handle the outcome inside the routine, for example by recording errors and failing the benchmark after measuring, or validate the input before measuring.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Benchmarks,
    summary: "A measured routine whose must-use result criterion drops",
};

pub const NEGATED_CONFIGURATION: IssueText = IssueText {
    definition: "A cfg or cfg_attr predicate that negates a feature, test or debug_assertions. Code that the predicate leaves active in no declared configuration is listed within its issue, and the predicate of a module file that its own attributes configure out is reported at the declaration unless an analysis of that same declaration reports it where it is written.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NegatedConfiguration",
    ordinal: 3,
    rationale: "Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive.",
    remedy: "Express the condition positively.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Configurations,
    summary: "A cfg predicate that negates a feature, test or debug_assertions",
};

pub const NEXTEST_CONFIGURATION_MISSING: IssueText = IssueText {
    definition: "A workspace whose root holds no .config/nextest.toml, the only repository configuration that nextest reads, while its analysed code has an active #[test] that libtest's harness runs, so that its tests run under nextest's defaults, which never terminate a test. A partial run in which no analysed configuration runs such a test does not evaluate it.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "NextestConfigurationMissing",
    ordinal: 3,
    rationale: "Every test finishes within the limits, so that a slow test shows slow code instead of stalling the run.",
    remedy: "Add a .config/nextest.toml whose default profile sets a slow-timeout within the limits, and make the tests that exceed them faster.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A workspace without a nextest configuration",
};

pub const NEXTEST_OVERRIDE: IssueText = IssueText {
    definition: "A per-test override in a nextest profile, which changes the timeouts, retries or threads of the tests it selects ahead of every profile setting.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NextestOverride",
    ordinal: 4,
    rationale: "Every test meets the same limits.",
    remedy: "Remove the override and make the tests it singles out faster.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A per-test override in the nextest configuration",
};

pub const NEXTEST_TIMEOUT_NOT_ENFORCED: IssueText = IssueText {
    definition: "A nextest profile whose slow-timeout flags a test as slow later than the slow limit, terminates it later than the termination limit or never, or counts a timed out test as passing. The default profile sets a slow-timeout, since the one nextest ships never terminates a test, and a profile that sets none inherits the slow-timeout of its parent.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NextestTimeoutNotEnforced",
    ordinal: 5,
    rationale: "A test that runs longer than the limits shows slow code or a slow test.",
    remedy: "Make the code and the test faster instead of relaxing the timeout.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A nextest profile whose slow-timeout exceeds the limits",
};

pub const NON_BENCHMARK_ITEM_IN_BENCHMARK_TARGET: IssueText = IssueText {
    definition: "An item of a benchmark target other than its entry point, a test, a constant, an import, a module or a private item that only the file defining it uses, among the items of module files, including those that a macro_rules! macro of the crate writes: a macro call is judged by the items it expands to, and the items that procedural macros and the macros of other crates generate are trusted. A target with harness = false enters through the main function of its crate root. The impl blocks of a flagged type are listed within the type's issue.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NonBenchmarkItemInBenchmarkTarget",
    ordinal: 8,
    rationale: "A helper used beyond its own benchmark file is a library, and libraries never live in a benches directory.",
    remedy: "Move it into a library crate, a test crate when only tests and benchmarks use it; when that crate depends on the package the benchmarks measure, move the benchmarks that use it there too, so package dependencies never form a cycle.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Benchmarks,
    summary: "A benchmark target item that is neither its entry point nor a file-local helper",
};

pub const NON_CONSTANT_IN_CONSTANTS_MODULE: IssueText = IssueText {
    definition: "A public item other than a constant or a module inside the crate's constants module, crate::constants.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NonConstantInConstantsModule",
    ordinal: 13,
    rationale: "The constants module is the single home of public constants.",
    remedy: "Move every other public item out of it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A public item other than a constant in crate::constants",
};

pub const NON_DOC_COMMENT: IssueText = IssueText {
    definition: "A comment other than a doc comment, in a module file.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NonDocComment",
    ordinal: 2,
    rationale: "Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed, and only on the items they document.",
    remedy: "Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an item; a statement, an expression or an unsafe block has no comment form, and unsafe code exists only where a crate integrates foreign code.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A comment that is not a doc comment",
};

pub const NON_TEST_ITEM_IN_TEST_TARGET: IssueText = IssueText {
    definition: "An item of an integration test target other than a test, a constant, an import, a module or a private item that only the file defining it uses, among the items of module files, including those that a macro_rules! macro of the crate writes: a macro call is judged by the items it expands to, and the items that procedural macros and the macros of other crates generate are trusted. A test is a function under the built-in test, bench or test_case attribute, and a target with harness = false enters through the main function of its crate root. The impl blocks of a flagged type are listed within the type's issue.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "NonTestItemInTestTarget",
    ordinal: 6,
    rationale: "A helper used beyond its own test file is a test library, and test libraries never live in a tests directory.",
    remedy: "Move it into a library crate; when that crate depends on the package under test, move the tests that use it there too, so package dependencies never form a cycle.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "An integration test item that is neither a test nor a file-local helper",
};

pub const PACKAGE_DEPENDENCY_CYCLE: IssueText = IssueText {
    definition: "Packages that depend on each other through dev-dependencies, each named with its version, where dependencies on packages without a library are ignored as cargo ignores them.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "PackageDependencyCycle",
    ordinal: 5,
    rationale: "Package dependencies never form a cycle, dev-dependencies included.",
    remedy: "When a test library depends on the package whose tests use it, move those tests and the helpers that need the package into a separate tests package that holds them in its library.",
    scope: RuleScope::Workspace,
    section: RuleSection::Packages,
    summary: "Packages that depend on each other through dev-dependencies",
};

pub const PACKAGE_DEPENDS_ON_ITSELF: IssueText = IssueText {
    definition: "A package that depends on itself through dev-dependencies.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "PackageDependsOnItself",
    ordinal: 6,
    rationale: "A package depending on itself is built twice and blurs what it owns.",
    remedy: "Remove the self-dependency.",
    scope: RuleScope::Package,
    section: RuleSection::Packages,
    summary: "A package that depends on itself through dev-dependencies",
};

pub const POSITIONAL_FIELDS: IssueText = IssueText {
    definition: "A struct or enum variant written with two or more positional fields.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &["A struct or variant with one positional field, which wraps a single value."],
    name: "PositionalFields",
    ordinal: 3,
    rationale: "A positional field has no name, so every use relies on the order of the fields and nothing says what each one means.",
    remedy: "Give the fields names.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A struct or variant with two or more positional fields",
};

pub const PROCEDURAL_FUNCTION: IssueText = IssueText {
    definition: "A function that takes a struct, enum or union of its own crate, by value, by reference or behind a type that dereferences to it, such as Box, Rc or Arc, without belonging to it: it has no self receiver that its body reads, is neither declared by a trait nor part of a trait impl, does not construct the self type of its impl, directly, as the success of a Result or as the value of an Option, an async function by the output of its future, and is not a private helper used only in the file that defines it.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ProceduralFunction",
    ordinal: 4,
    rationale: "Behavior belongs with the data it works on.",
    remedy: "Restructure the code so that the function no longer needs a value of that type, or move the function into a method of the type. A constructor of its own type and a private helper used in a single file are accepted as they are.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A function that takes a type of its crate without belonging to it",
};

pub const PUBLIC_ASSOCIATED_CONSTANT: IssueText = IssueText {
    definition: "A public associated constant of an inherent impl, among the items written in module files.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "PublicAssociatedConstant",
    ordinal: 14,
    rationale: "Public constants have a single home, crate::constants.",
    remedy: "Move the constant there as a free constant.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A public associated constant of an inherent impl",
};

pub const PUBLIC_CONSTANT_OUTSIDE_CONSTANTS_MODULE: IssueText = IssueText {
    definition: "A public constant outside the crate's constants module, crate::constants, among the items written in module files.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "PublicConstantOutsideConstantsModule",
    ordinal: 15,
    rationale: "Public constants have a single home, crate::constants.",
    remedy: "Move the constant there.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A public constant outside crate::constants",
};

pub const PUBLISHABLE_TEST_CRATE: IssueText = IssueText {
    definition: "A package that test_crates lists but that can be published. A test crate needs publish = false, so the package is analysed as production code.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "PublishableTestCrate",
    ordinal: 7,
    rationale: "A test crate holds only what tests use, so it is never published.",
    remedy: "Set publish = false in its manifest, or remove it from test_crates.",
    scope: RuleScope::Package,
    section: RuleSection::Tests,
    summary: "A declared test crate that can be published",
};

pub const REFERENCED_TEST: IssueText = IssueText {
    definition: "A function under the built-in test, bench or test_case attribute that other code references.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ReferencedTest",
    ordinal: 8,
    rationale: "The test harness calls every test, and a test that other code calls ties tests to each other.",
    remedy: "Move the code they share into a helper that each test calls.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "A test that other code references",
};

pub const REEXPORT: IssueText = IssueText {
    definition: "A re-export, apart from the import that gives a macro_rules! macro of its own module that is not exported its only path.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "Reexport",
    ordinal: 16,
    rationale: "Every item is reachable through exactly one path.",
    remedy: "Import the item from where it is defined instead of re-exporting it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Modules,
    summary: "A re-export",
};

pub const REQUIRED_CLIPPY_SETTING_MISSING: IssueText = IssueText {
    definition: "A clippy configuration that does not give each required setting the value it requires: true for the test allowances, which are the only way tests may use expect and unwrap, and false for avoid-breaking-exported-api, which clippy otherwise sets to true and so leaves the lints that would change a public signature silent on public items.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RequiredClippySettingMissing",
    ordinal: 5,
    rationale: "Tests may fail fast with expect and unwrap, and the lints that clippy relaxes for public items by default must hold for them too.",
    remedy: "Set each required setting to the value it names.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A clippy configuration without a required setting",
};

pub const REQUIRED_LINT_NOT_DENIED: IssueText = IssueText {
    definition: "A workspace member whose lints, after workspace inheritance and in cargo's order, leave a required lint or lint group below deny, under either spelling of its name or a name clippy renamed to it, a warn level counting as deny while the warnings group is denied. Every member, a test crate included, denies the warnings group, the clippy all, nursery and pedantic groups and the panicking lints, the clippy lints that report code that can panic. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RequiredLintNotDenied",
    ordinal: 6,
    rationale: "Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items.",
    remedy: "Deny the lint in the [lints] or [workspace.lints] table the package uses.",
    scope: RuleScope::Package,
    section: RuleSection::Tools,
    summary: "A workspace member that leaves a required lint or lint group below deny",
};

pub const RESULT_ERROR_NOT_AN_ERROR: IssueText = IssueText {
    definition: "A Result written in an item signature or type definition, such as the type of a function parameter, a return type, a field or a type alias, whose error type is a struct, enum or union defined in the workspace that does not implement std::error::Error, and an associated type of a trait impl that the trait names as the error of a Result set to such a type. A type parameter is checked where it is given a type.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &["A Result whose type the trait of an impl dictates."],
    name: "ResultErrorNotAnError",
    ordinal: 12,
    rationale: "Err is reserved for failures. A value that is not an error type is either an outcome of the regular flow handed back through Err, which hides an expected result where failures belong, or a failure without the contract of an error, its message and its source chain.",
    remedy: "When the value is a failure, make it the error type of its crate, or a variant of it, deriving thiserror::Error. When it is an outcome of the regular flow, return it in Ok, as a variant of an enum of outcomes.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "A Result whose error type is a workspace type that is not an error",
};

pub const RUSTFMT_CONFIGURATION_AMBIGUOUS: IssueText = IssueText {
    definition: "A directory, the workspace root or a directory between the workspace root and a module file of a workspace member, that holds both rustfmt.toml and .rustfmt.toml. Rustfmt reads .rustfmt.toml and ignores the other without a warning.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RustfmtConfigurationAmbiguous",
    ordinal: 8,
    rationale: "One configuration decides how the code of a directory is formatted.",
    remedy: "Keep one of the two files.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A directory with both rustfmt configuration files",
};

pub const RUSTFMT_CONFIGURATION_INVALID: IssueText = IssueText {
    definition: "A rustfmt.toml or .rustfmt.toml, in the workspace root or a directory between the workspace root and a module file of a workspace member, that is not valid TOML.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RustfmtConfigurationInvalid",
    ordinal: 9,
    rationale: "Rustfmt cannot format the code under a configuration it cannot parse, so that code is held to no formatting.",
    remedy: "Fix the configuration.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A rustfmt configuration that is not valid TOML",
};

pub const RUSTFMT_CONFIGURATION_MISSING: IssueText = IssueText {
    definition: "A workspace root that holds neither rustfmt.toml nor .rustfmt.toml. Rustfmt searches the directory of each file and its parents for a configuration and, when it finds none, reads one from the home or user configuration directory.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RustfmtConfigurationMissing",
    ordinal: 10,
    rationale: "Without a configuration in the workspace root, the formatting of the workspace depends on files outside it.",
    remedy: "Add a rustfmt.toml to the workspace root; an empty one selects the defaults.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A workspace without a rustfmt configuration",
};

pub const RUSTFMT_CONFIGURATION_UNREADABLE: IssueText = IssueText {
    definition: "A rustfmt.toml or .rustfmt.toml, in the workspace root or a directory between the workspace root and a module file of a workspace member, that cannot be read as text, such as a directory of that name.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RustfmtConfigurationUnreadable",
    ordinal: 11,
    rationale: "A configuration that cannot be read cannot be vetted, and what rustfmt makes of it is not a declared input.",
    remedy: "Replace it with a configuration file or remove it.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A rustfmt configuration that cannot be read",
};

pub const RUST_VERSION_BELOW_SUPPORTED: IssueText = IssueText {
    definition: "A rust-version of a workspace member, written in its manifest or inherited from the workspace manifest, that is older than the Rust version Heisenberg supports. Clippy reads rust-version as the minimum supported Rust version and silences or weakens the lints whose fixes need a newer Rust. An inherited rust-version is reported once, at the workspace manifest.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "RustVersionBelowSupported",
    ordinal: 7,
    rationale: "The workspace is analysed and linted as the supported Rust compiles it, so a lower rust-version only relaxes the lints that Heisenberg requires.",
    remedy: "Remove rust-version or set it to the supported Rust version.",
    scope: RuleScope::Package,
    section: RuleSection::Tools,
    summary: "A rust-version older than the supported Rust",
};

pub const SHARED_MODULE_FILE: IssueText = IssueText {
    definition: "A module file shared by several modules, or by two modules of one crate, which leaves the second unanalysed. A shared module subtree is reported at its root file.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "SharedModuleFile",
    ordinal: 6,
    rationale: "A file included as several modules has no single owner.",
    remedy: "Give each module its own file.",
    scope: RuleScope::Workspace,
    section: RuleSection::Analysis,
    summary: "A module file included as several modules",
};

pub const SINGLE_LETTER_NAME: IssueText = IssueText {
    definition: "A lifetime, type or const parameter, those of a for<...> binder included, or a name in the pattern of a function, closure or function pointer parameter, whose name is a single character.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "SingleLetterName",
    ordinal: 5,
    rationale: "A single character says nothing about what a name stands for.",
    remedy: "Name it after its meaning, such as 'database for a lifetime or TNode for a type parameter.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A parameter name of a single character",
};

pub const SOURCE_IN_ERROR_MESSAGE: IssueText = IssueText {
    definition: "An #[error(...)] message of an enum that derives thiserror::Error that formats the source of its variant itself, rather than a value that the source holds.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "SourceInErrorMessage",
    ordinal: 10,
    rationale: "The error chain reports the source after the message, so a message that formats the source as well prints the cause twice.",
    remedy: "Leave the source out of the message and let the error chain report it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "An error message that formats its own source",
};

pub const SPLIT_INTEGRATION_TESTS: IssueText = IssueText {
    definition: "A workspace member with integration test targets that libtest's harness runs, whose [package] table does not set autotests = false, or that has more than one such target.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &["A target with harness = false, which brings its own harness."],
    name: "SplitIntegrationTests",
    ordinal: 9,
    rationale: "Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules.",
    remedy: "Set autotests = false, declare one [[test]] target, and include the other test files as its modules.",
    scope: RuleScope::Package,
    section: RuleSection::Tests,
    summary: "Integration tests spread over several targets or discovered automatically",
};

pub const TEST_CRATE_DECLARED_TWICE: IssueText = IssueText {
    definition: "A package that test_crates lists more than once.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestCrateDeclaredTwice",
    ordinal: 10,
    rationale: "Each test crate is declared once.",
    remedy: "Remove the repeated entry from test_crates.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A test crate listed more than once",
};

pub const TEST_CRATE_NOT_A_MEMBER: IssueText = IssueText {
    definition: "A test_crates entry that names no workspace member.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestCrateNotAMember",
    ordinal: 11,
    rationale: "Only a workspace member can be a test crate.",
    remedy: "Remove the entry from test_crates, or add the package to the workspace.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A declared test crate that is not a workspace member",
};

pub const TEST_CRATE_USED_BY_PRODUCTION: IssueText = IssueText {
    definition: "A package that test_crates lists and that a member other than a test crate depends on through a normal or build dependency, so the package is analysed as production code. The issue is reported at the manifest of each such dependent.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestCrateUsedByProduction",
    ordinal: 12,
    rationale: "Production code never depends on test code.",
    remedy: "Make the dependency a dev-dependency, or remove the package from test_crates.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A declared test crate that production code depends on",
};

pub const TEST_MODULE_IN_ITS_OWN_FILE: IssueText = IssueText {
    definition: "A module declaration without a body, in production code, whose cfg or cfg_attr predicate requires test, so that the tests or test helpers of a production crate live in a file of their own. Production code is every crate target that is neither an integration test target, a benchmark target nor a target of a test crate.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestModuleInItsOwnFile",
    ordinal: 13,
    rationale: "The unit tests of a file are part of that file, and code that several tests share is a test library, which belongs in a test crate.",
    remedy: "Write the tests in an inline #[cfg(test)] mod tests of the file they test, and move shared test code to a test crate.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "A test module declared in a file of its own",
};

pub const TEST_MODULE_NOT_NAMED_TESTS: IssueText = IssueText {
    definition: "An inline module of production code under #[cfg(test)] that is not named tests. A file can hold one module of that name, so each file holds one test module.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestModuleNotNamedTests",
    ordinal: 14,
    rationale: "One name marks the unit tests of every file, so they are found without reading attributes.",
    remedy: "Name the module tests, and merge the test modules of a file into it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "A test module that is not named tests",
};

pub const TEST_ONLY_CODE_IN_PRODUCTION: IssueText = IssueText {
    definition: "An item, field, variant, statement, expression or parameter of production code, other than a module, whose cfg or cfg_attr predicate requires test, an inline module whose predicate requires test in any form other than #[cfg(test)] alone, and a cfg! call that names test, each outside every item that cfg(test) gates. Production code is every crate target that is neither an integration test target, a benchmark target nor a target of a test crate. The code is reported at its outermost such item.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestOnlyCodeInProduction",
    ordinal: 15,
    rationale: "Production code holds only what production runs, and behaves the same whether or not it is under test.",
    remedy: "Keep test-only code in the inline #[cfg(test)] mod tests of the file it tests, and move test libraries shared by several packages into a test crate.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "Code that depends on cfg(test) outside a test module",
};

pub const TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES: IssueText = IssueText {
    definition: "A tests package, whose targets are integration tests, possibly with benchmark targets, beside an absent or empty library, a library being empty when it defines nothing in any declared configuration, and whose integration test and benchmark targets hold nothing but tests, entry points, constants, imports, modules and file-local private items, so that it holds no test library of its own. Integration tests belong to the package whose behaviour they verify, in its tests directory, and may use any dev-dependency; a helper used beyond its own test file is a test library, which belongs in a library crate; a separate tests package exists only to hold test libraries of its own, such as helpers that need the package under test, which that package cannot dev-depend on without a package cycle. The issue lists every workspace member the package depends on, either as a package that could take its tests or with the dependency cycle that moving them there would close, as guidance, not as the owner.",
    evaluation: RuleEvaluation::EveryDeclaredConfigurationInPart,
    exceptions: &[],
    name: "TestsOnlyPackageWithoutTestLibraries",
    ordinal: 17,
    rationale: "Tests belong to the package whose behaviour they verify, in its tests directory. A separate tests package is justified only by test libraries of its own: shared helpers, which never live in a tests directory, and helpers that need the package under test, which that package cannot dev-depend on without a dependency cycle.",
    remedy: "Heisenberg cannot tell from the code which package the tests verify, so it lists every workspace member the tests use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they verify. If a test library they use depends on that package, the test library is entangled with the code it helps to test: move the helpers that need the package into this package's library, or make the test library independent of it.",
    scope: RuleScope::Package,
    section: RuleSection::Tests,
    summary: "A package of tests that holds no test library of its own",
};

pub const TEST_OUTSIDE_TEST_MODULE: IssueText = IssueText {
    definition: "A function under the built-in test, bench or test_case attribute, in production code, outside every item that cfg(test) gates.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TestOutsideTestModule",
    ordinal: 16,
    rationale: "A test outside a test module is compiled into the same scope as production code, so nothing separates the code that runs in production from the code that tests it.",
    remedy: "Move the test into the #[cfg(test)] mod tests of its file.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tests,
    summary: "A test written outside a test module",
};

pub const TOOL_ATTRIBUTE: IssueText = IssueText {
    definition: "An attribute of the clippy or rustfmt tool that the workspace writes, in module files, through a cfg_attr, through a macro of a workspace member or as output of a procedural macro of a workspace member, such as clippy::msrv, clippy::cognitive_complexity, rustfmt::skip or rustfmt::skip::macros.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "ToolAttribute",
    ordinal: 12,
    rationale: "A tool attribute changes how clippy lints or how rustfmt formats the code it is written on, so it relaxes the lints and the formatting that the whole workspace is held to.",
    remedy: "Remove the attribute and fix the code it excused.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Tools,
    summary: "An attribute of the clippy or rustfmt tool",
};

pub const TUPLE_IN_SIGNATURE: IssueText = IssueText {
    definition: "A tuple type of two or more elements written in an item signature or type definition: the type of a function parameter, a return type, a field, a type alias, the type of a constant or static, an associated type, an impl header or a trait definition, nested types included. The functions and constants of a trait impl, whose types the trait dictates, and types written inside function bodies are not checked.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "TupleInSignature",
    ordinal: 6,
    rationale: "A tuple names none of its values, so its readers must remember what each position holds.",
    remedy: "Declare a struct with named fields instead.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A tuple type in an item signature",
};

pub const UNCOVERED_CODE: IssueText = IssueText {
    definition: "A region of production code that the coverage export records and that no test executed. A function that no test executes is reported at its name, a function that tests execute in part once at its name with every unexecuted region, and a region outside every function, such as one in a macro_rules! definition that only test code calls, at the region itself. A region counts as executed when any instrumented build executed it, whichever instantiation, crate type or function written from the same text it belongs to. Production code is the text written in the module files of every crate target that is neither an integration test target, a benchmark target nor a target of a test crate, active in the measured configuration and outside the modules that cfg(test) gates.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UncoveredCode",
    ordinal: 2,
    rationale: "Every line of production code runs under the tests.",
    remedy: "Add tests that execute the code, or remove the code that nothing needs.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Coverage,
    summary: "Production code that no test executes",
};

pub const UNDESTRUCTURED_PARAMETER: IssueText = IssueText {
    definition: "A function or closure parameter bound by an identifier, other than self, whose every use, at least one, accesses a field of the struct or tuple the parameter holds, directly or behind references, with the field name written in a module file.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[
        "A field reached through Box or another Deref implementation, and a field of a union.",
        "A parameter that holds by value a type implementing Drop and reads a field that is not Copy, which a pattern cannot move out of.",
    ],
    name: "UndestructuredParameter",
    ordinal: 7,
    rationale: "A parameter used only through its fields reads better as a pattern that names those fields.",
    remedy: "Destructure the parameter in its pattern.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A parameter used only through its fields",
};

pub const UNDETERMINED_BENCHMARK_OUTPUT: IssueText = IssueText {
    definition: "A call or reference to one of the measurement loops that MustUseBenchmarkOutput checks, whose routine returns a type that leaves undetermined whether it is a must-use value: a type parameter, or a type that rust-analyzer cannot infer, alone or where the must-use rule looks into it, such as inside a Box or a tuple.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UndeterminedBenchmarkOutput",
    ordinal: 9,
    rationale: "Whether criterion drops a must-use value is decided where the routine is measured.",
    remedy: "Write the measured routine where it is measured, so that the type it returns is concrete.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Benchmarks,
    summary: "A measured routine whose return type the analysis cannot determine",
};

pub const UNIMPLEMENTED_BUILTIN_MACRO: IssueText = IssueText {
    definition: "A call of a built-in macro that rust-analyzer does not implement.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnimplementedBuiltinMacro",
    ordinal: 7,
    rationale: "rust-analyzer cannot expand this built-in macro, so its output cannot be analysed.",
    remedy: "Replace the call with code that can be analysed.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A built-in macro that rust-analyzer cannot expand",
};

pub const UNKNOWN_CONFINED_PACKAGE: IssueText = IssueText {
    definition: "A confined_packages entry that names no package that the workspace resolves, member or dependency.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnknownConfinedPackage",
    ordinal: 7,
    rationale: "A confinement holds only for a package that the workspace resolves.",
    remedy: "Remove the entry from confined_packages, or correct the package name.",
    scope: RuleScope::Workspace,
    section: RuleSection::Packages,
    summary: "A confined package that the workspace does not resolve",
};

pub const UNMAPPED_ITEM: IssueText = IssueText {
    definition: "An item whose syntax rust-analyzer cannot map to its definition, such as an item of a block inside a type.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnmappedItem",
    ordinal: 8,
    rationale: "An item that cannot be mapped to its definition leaves the analysis incomplete.",
    remedy: "Write it so it maps to a definition.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "An item that cannot be mapped to its definition",
};

pub const UNNEEDED_NEXTEST_CONFIGURATION: IssueText = IssueText {
    definition: "A .config/nextest.toml in a workspace whose analysed code has no active #[test] that libtest's harness runs, in any declared configuration: every test target brings its own harness. A partial run in which no analysed configuration runs such a test does not evaluate it.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "UnneededNextestConfiguration",
    ordinal: 18,
    rationale: "Nextest runs only the tests that libtest's harness defines, so a workspace whose tests all bring their own harness never uses its nextest configuration, and a configuration that nothing reads only suggests limits that nothing enforces.",
    remedy: "Remove the configuration.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A nextest configuration in a workspace without libtest tests",
};

pub const UNPARSABLE_MACRO_EXPANSION: IssueText = IssueText {
    definition: "A macro expansion that cannot be parsed. It is reported at the macro call written in the source, even when a macro of another crate wrote the call that produced it, listed with the macros whose expansions lead to it, the syntax errors rust-analyzer finds and the literal tokens it cannot read back, such as a negative number that a procedural macro emits as one literal token, whose sign belongs in a separate - token, and it is not analysed further.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnparsableMacroExpansion",
    ordinal: 9,
    rationale: "A macro expansion that does not parse cannot be analysed.",
    remedy: "Fix the macro so its output is valid Rust.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A macro expansion that does not parse",
};

pub const UNPINNED_DEPENDENCY_REQUIREMENT: IssueText = IssueText {
    definition: "A dependency that the manifest of a workspace member or the workspace manifest writes, in a dependencies, dev-dependencies or build-dependencies table, a target table or the workspace.dependencies table, with a version requirement other than a single caret requirement of MAJOR.MINOR, or of MAJOR.MINOR.PATCH for a 0.0 or pre-release version, whose every release may break; or as a git dependency without a rev. A dependency inherited from the workspace is checked where the workspace writes it.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &["A path dependency without a version."],
    name: "UnpinnedDependencyRequirement",
    ordinal: 8,
    rationale: "A requirement of MAJOR.MINOR accepts the compatible releases that Cargo.lock then fixes, a wider one lets a build move to a release that breaks it, and a narrower one blocks the compatible fixes.",
    remedy: "Write the requirement as MAJOR.MINOR, and give a git dependency the rev it is built from.",
    scope: RuleScope::Package,
    section: RuleSection::Packages,
    summary: "A dependency requirement that is not a MAJOR.MINOR version or a git rev",
};

pub const UNPREFIXED_TYPE_PARAMETER: IssueText = IssueText {
    definition: "A type parameter whose name of more than one character is not T followed by an uppercase letter, such as TNode. A single-character name is a SingleLetterName issue instead.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnprefixedTypeParameter",
    ordinal: 8,
    rationale: "The T prefix tells type parameters apart from concrete types.",
    remedy: "Rename the parameter to T followed by a capitalized word, such as TNode.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Code,
    summary: "A type parameter not named T followed by a capitalized word",
};

pub const UNREACHABLE_SOURCE_FILE: IssueText = IssueText {
    definition: "A Rust source file in the directory of a workspace member, outside the directories of the packages nested in it, that rust-analyzer loads and that is a module file of no crate target in any declared configuration. Evaluated only when every declared configuration is analysed.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "UnreachableSourceFile",
    ordinal: 10,
    rationale: "A source file that no target includes is never compiled or analysed, so its code and its tests never run and nothing reports it.",
    remedy: "Declare the file as a module of a crate target, or remove it.",
    scope: RuleScope::Package,
    section: RuleSection::Analysis,
    summary: "A source file that no crate target includes",
};

pub const UNRESOLVED_EXTERN_CRATE: IssueText = IssueText {
    definition: "An extern crate declaration that rust-analyzer cannot resolve.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnresolvedExternCrate",
    ordinal: 11,
    rationale: RESOLUTION_RATIONALE,
    remedy: "Declare the crate as a dependency or remove the declaration.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "An extern crate that rust-analyzer cannot resolve",
};

pub const UNRESOLVED_MACRO_CALL: IssueText = IssueText {
    definition: "A macro call, attribute or derive whose macro rust-analyzer cannot resolve. A macro call whose name an unresolved import binds belongs to that import's issue.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnresolvedMacroCall",
    ordinal: 12,
    rationale: RESOLUTION_RATIONALE,
    remedy: "Define or import the macro so that rust-analyzer resolves it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A macro call, attribute or derive whose macro rust-analyzer cannot resolve",
};

pub const UNRESOLVED_METHOD_CALL: IssueText = IssueText {
    definition: "A method call whose method rust-analyzer cannot resolve, unless its receiver holds a name that rust-analyzer cannot resolve, which is that name's issue.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnresolvedMethodCall",
    ordinal: 13,
    rationale: RESOLUTION_RATIONALE,
    remedy: "Write the call so that rust-analyzer resolves its method, such as by naming the type of its receiver.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A method call that rust-analyzer cannot resolve",
};

pub const UNRESOLVED_MODULE: IssueText = IssueText {
    definition: "A module declaration whose file rust-analyzer cannot resolve.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnresolvedModule",
    ordinal: 14,
    rationale: RESOLUTION_RATIONALE,
    remedy: "Create the module file or remove the declaration.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A module declaration whose file rust-analyzer cannot resolve",
};

pub const UNRESOLVED_PATH: IssueText = IssueText {
    definition: "A path that rust-analyzer cannot resolve, imports included. A name bound by an unresolved import, whether a path or a macro call, belongs to that import's issue, and paths into a module of another crate that do not resolve are one issue per module, listed under a failed macro expansion that a workspace member reports in that module's file when there is one.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnresolvedPath",
    ordinal: 15,
    rationale: RESOLUTION_RATIONALE,
    remedy: "Write the path so that rust-analyzer resolves it, such as by importing the item it names.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "A path that rust-analyzer cannot resolve",
};

pub const UNSAFE_CODE: IssueText = IssueText {
    definition: "An unsafe block, function, trait, impl, extern block or attribute, written in the module files of a crate whose own code neither declares foreign functions or statics in an extern block, nor uses one, nor defines a function with a foreign ABI.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnsafeCode",
    ordinal: 16,
    rationale: "Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it.",
    remedy: "Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Analysis,
    summary: "Unsafe code in a crate that integrates no foreign code",
};

pub const UNTYPED_RESULT_ERROR: IssueText = IssueText {
    definition: "A Result written in an item signature or type definition, such as the type of a function parameter, a return type, a field or a type alias, whose error type does not implement std::error::Error and is not a struct, enum or union defined in the workspace, such as String, &str, (), an integer, a collection, Box<dyn Error> or anyhow::Error, and an associated type of a trait impl that the trait names as the error of a Result, such as the Err of FromStr, set to such a type. A type parameter is checked where it is given a type.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[
        "A Result whose type the trait of an impl dictates.",
        "A Vec whose element type implements std::error::Error, which accumulates failures that are each typed.",
    ],
    name: "UntypedResultError",
    ordinal: 11,
    rationale: "An error type that erases which failure happened names no failure in the signature: a caller can print it, or downcast it where it carries a type, but cannot match on it.",
    remedy: "Declare an error enum whose variants name each failure, and return it. Return several failures as a Vec of that error type.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Errors,
    summary: "A Result whose error type erases the failure",
};

pub const UNVETTED_CLIPPY_SETTING: IssueText = IssueText {
    definition: "A clippy configuration setting other than a required one and those that only add restrictions, which can silence or relax the lints that Heisenberg requires.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnvettedClippySetting",
    ordinal: 13,
    rationale: "A clippy setting can silence or relax the lints that Heisenberg requires, so a clippy configuration holds only the required settings and settings that only add restrictions.",
    remedy: "Remove the setting.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A clippy configuration setting that Heisenberg has not vetted",
};

pub const UNVETTED_CRITERION_METHOD: IssueText = IssueText {
    definition: "A call or reference, in the code of a workspace member, to an inherent method of criterion's Criterion, BenchmarkGroup, Bencher or AsyncBencher that Heisenberg has not vetted: one that changes how long or how many samples it measures, how it judges results or which benchmarks run, a method that criterion hides from its documentation, or a method that Heisenberg does not know. criterion is the criterion package from crates.io, and the methods of traits that these types implement are not inherent methods.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnvettedCriterionMethod",
    ordinal: 10,
    rationale: "Every benchmark is measured with criterion's defaults, the same way every time.",
    remedy: "Remove the call, and make the benchmark that needed it faster instead of shortening, weakening or skipping its measurement.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Benchmarks,
    summary: "A criterion method that Heisenberg has not vetted",
};

pub const UNVETTED_NEXTEST_SETTING: IssueText = IssueText {
    definition: "A nextest configuration setting other than the slow-timeout and those that only control concurrency, failure handling and reporting, which can retry, skip or exempt tests, or let a test run past its timeout.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnvettedNextestSetting",
    ordinal: 19,
    rationale: "Every test runs and passes on its first attempt within the limits.",
    remedy: "Remove the setting, and make the tests that needed it faster or reliable.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tests,
    summary: "A nextest configuration setting that Heisenberg has not vetted",
};

pub const UNVETTED_RUSTFMT_SETTING: IssueText = IssueText {
    definition: "A setting of a rustfmt.toml or .rustfmt.toml, in the workspace root or a directory between the workspace root and a module file of a workspace member, other than the stable settings that only choose a layout. Settings that exempt code from formatting, such as disable_all_formatting, ignore or skip_children, unstable settings and unknown settings are not vetted.",
    evaluation: RuleEvaluation::EachAnalysedConfiguration,
    exceptions: &[],
    name: "UnvettedRustfmtSetting",
    ordinal: 14,
    rationale: "A rustfmt setting can exempt code from formatting, and an unstable or unknown one is ignored by the supported toolchain.",
    remedy: "Remove the setting.",
    scope: RuleScope::Workspace,
    section: RuleSection::Tools,
    summary: "A rustfmt configuration setting that Heisenberg has not vetted",
};

pub const VACUOUS_NEGATED_CONFIGURATION: IssueText = IssueText {
    definition: "A cfg or cfg_attr predicate that negates a configuration flag or key-value pair, other than a feature, test or debug_assertions, that is set in no declared configuration that reaches the predicate, such as clippy, rustfmt, miri or doc. The negation is reported once for each such flag or pair. Evaluated only when every declared configuration is analysed.",
    evaluation: RuleEvaluation::EveryDeclaredConfiguration,
    exceptions: &[],
    name: "VacuousNegatedConfiguration",
    ordinal: 4,
    rationale: "A negation of a configuration that no declared configuration sets holds wherever Heisenberg analyses the code, so it only changes what a tool outside the declared configurations sees, and hides the code from that tool.",
    remedy: "Remove the negation.",
    scope: RuleScope::ModuleFile,
    section: RuleSection::Configurations,
    summary: "A cfg predicate that negates a configuration no declared configuration sets",
};

pub const HEISENBERG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const RULE_CATEGORY: &str = "mandatory";

pub const RULE_DECIDABILITY: &str = "decidable";

pub const DEFAULT_JOBS: NonZeroU16 = NonZeroU16::MIN.saturating_add(7);

pub const TARGET_TRIPLE_PLACEHOLDER: &str = "<TRIPLE>";
