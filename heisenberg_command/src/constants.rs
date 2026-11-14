use std::num::NonZeroU16;

use crate::issue_text::IssueText;

macro_rules! resolution_purpose {
    ($instruction:literal) => {
        concat!(
            "The code compiles, so a name that rust-analyzer cannot resolve leaves a gap in the analysis. ",
            $instruction
        )
    };
}

pub const ADDITIONAL_PUBLIC_ITEM: IssueText = IssueText {
    definition: "A public item of a module beyond the one that the module's name spells, or beyond its first public item when the name spells none, among the items of module files, including those that a macro_rules! macro of the crate writes, constants, modules and the functions that Rust requires for procedural macros at the crate root aside. An item is public when its visibility reaches beyond its own module, so a `pub(self)` item is not public while a `pub` item of a private module is, and a `macro_rules!` macro is public when exported.",
    name: "AdditionalPublicItem",
    purpose: "Each module exposes a single public item, so every file has one responsibility. Move each additional public item into its own module.",
    summary: "A module with more than one public item",
};

pub const BENCHMARKS_ONLY_PACKAGE_WITHOUT_LIBRARIES: IssueText = IssueText {
    definition: "A benchmarks package, whose targets are benchmark targets beside an absent or empty library, a library being empty when it defines nothing in any declared configuration, and whose benchmark targets hold nothing but their entry points, constants, imports, modules and file-local private items, so that it holds no library of its own. Benchmarks belong to the package whose code they measure, in its benches directory, and may use any dev-dependency, test crates included; a helper used beyond its own benchmark file belongs in a library crate; a separate package exists only to hold a library of its own, such as helpers that need the measured package, which that package cannot dev-depend on without a package cycle. The issue lists every workspace member the package depends on, either as a package that could take its benchmarks or with the dependency cycle that moving them there would close, as guidance, not as the owner.",
    name: "BenchmarksOnlyPackageWithoutLibraries",
    purpose: "Benchmarks belong to the package whose code they measure, in its benches directory, and a benchmark that needs a library which depends on that package, such as a test harness, belongs beside that library in its package. Heisenberg cannot tell from the code which package the benchmarks measure, so it lists every workspace member they use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they measure.",
    summary: "A package of benchmarks that holds no library of its own",
};

pub const BENCHMARK_TARGET_EXCLUDED_FROM_BENCHMARKS: IssueText = IssueText {
    definition: "A benchmark target whose manifest sets bench = false, so cargo bench never runs it.",
    name: "BenchmarkTargetExcludedFromBenchmarks",
    purpose: "Every benchmark runs under cargo bench. Remove bench = false, or remove the benchmark target.",
    summary: "A benchmark target that cargo bench never runs",
};

pub const BENCHMARK_TARGET_RUN_BY_TESTS: IssueText = IssueText {
    definition: "A benchmark target whose manifest sets test = true, so cargo test and nextest build and run it with the tests.",
    name: "BenchmarkTargetRunByTests",
    purpose: "Benchmarks measure, and the test run never runs them. Remove test = true, and verify the behaviour that a benchmark relies on in a test.",
    summary: "A benchmark target that the test run runs",
};

pub const BENCHMARK_TARGET_WITHOUT_CRITERION_BENCHMARK: IssueText = IssueText {
    definition: "A benchmark target with harness = false whose module files call none of criterion's benchmark registrations, Criterion::bench_function, Criterion::bench_with_input, BenchmarkGroup::bench_function and BenchmarkGroup::bench_with_input, criterion being the criterion package from crates.io, so that a path, git or alternative registry package named criterion is not criterion. It is reported at the start of the target's crate root, and not when the target's code holds an unresolved path, method call, macro call or macro expansion, whose issue is reported instead.",
    name: "BenchmarkTargetWithoutCriterionBenchmark",
    purpose: "Heisenberg analyses benchmarks through criterion, the benchmark harness whose measurement it understands, so a benchmark measured any other way escapes its rules. Register every benchmark of the target with criterion in the target's own module files.",
    summary: "A benchmark target that registers no criterion benchmark",
};

pub const BENCHMARK_TARGET_WITH_LIBTEST_HARNESS: IssueText = IssueText {
    definition: "A benchmark target built with libtest's harness, harness = true, including a file of the benches directory that cargo discovers without a [[bench]] table. libtest's #[bench] is available only on the nightly channel, so on stable Rust such a target holds no benchmark, only tests that the test run never builds.",
    name: "BenchmarkTargetWithLibtestHarness",
    purpose: "A benchmark target measures through criterion from its own entry point. Declare the target in a [[bench]] table with harness = false and register its benchmarks with criterion, and move its tests into a tests directory.",
    summary: "A benchmark target built with libtest's harness",
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

pub const CODE_ACTIVE_IN_NO_CONFIGURATION: IssueText = IssueText {
    definition: "Code active in no declared configuration, a configuration being a supported target with a selection of features. A module file that its own attributes configure out is inactive code at its declaration. Code that a negated predicate leaves active in no declared configuration is listed within the predicate's NegatedConfiguration issue instead, and a compile_error! call, which exists only to fail the builds no configuration declares, is not inactive code.",
    name: "CodeActiveInNoConfiguration",
    purpose: "Code active in no declared configuration is never built or analysed. Remove it, declare a configuration it is active in, or make it active in a declared configuration.",
    summary: "Code that no declared configuration builds",
};

pub const CODE_WITHOUT_COVERAGE_DATA: IssueText = IssueText {
    definition: "A measured function, one with a body whose opening brace is production code as UncoveredCode defines it, written directly or by a macro, that no region of the coverage export falls into, because no instrumented build of the measured run compiled it.",
    name: "CodeWithoutCoverageData",
    purpose: "Every production function is measured. Build and run the target that holds the function under the instrumented tests, and keep the function instrumented: rustc records no coverage for #[automatically_derived] code, and cfg(coverage) leaves code out of the instrumented build.",
    summary: "A production function without coverage data",
};

pub const CONFIGURATION_SELECTION_MACRO: IssueText = IssueText {
    definition: "A cfg_select! call, whose unselected arms escape the analysis.",
    name: "ConfigurationSelectionMacro",
    purpose: "cfg_select! discards the arms it does not select before they can be analysed. Write each arm as an item under its own #[cfg] attribute.",
    summary: "A cfg_select! call",
};

pub const CONFINED_PACKAGE_REACHED_OUTSIDE_ITS_USERS: IssueText = IssueText {
    definition: "A workspace member that confined_packages does not list as a user of a package, while cargo builds the package for the member in an analysed configuration: through the normal and build dependencies of every package on the way, other workspace members and third-party packages alike, and through the member's own dev-dependencies, starting from the side its own targets build on, the host for a procedural macro crate and the target otherwise, each dependency on the target or the host where cargo builds it; a member never reaches its own package. The issue is reported once per member and package at the manifest of the member, with a shortest dependency path to the package, and names the configurations it holds in when it does not hold in every analysed one.",
    name: "ConfinedPackageReachedOutsideItsUsers",
    purpose: "A confined package is built and used only by its declared users, so nothing else reaches what it provides, even through another member. Remove the dependency that the path names; code that needs the package belongs in one of its users.",
    summary: "A member that reaches a confined package without being one of its users",
};

pub const CONFINED_PACKAGE_USER_LISTED_TWICE: IssueText = IssueText {
    definition: "A name that confined_packages lists more than once among the users of one package.",
    name: "ConfinedPackageUserListedTwice",
    purpose: "Each user of a confined package is listed once. Remove the repeated entry from confined_packages.",
    summary: "A confined package user listed more than once",
};

pub const CONFINED_PACKAGE_USER_NOT_A_MEMBER: IssueText = IssueText {
    definition: "A user of a confined package, in confined_packages, that names no workspace member.",
    name: "ConfinedPackageUserNotAMember",
    purpose: "Only a workspace member can use a confined package. Remove the entry from confined_packages, or add the package to the workspace.",
    summary: "A confined package user that is not a workspace member",
};

pub const CONFINED_PACKAGE_USER_NOT_REACHING_IT: IssueText = IssueText {
    definition: "A workspace member that confined_packages lists as a user of a package that cargo builds for it in no declared configuration, reported at the workspace manifest. A partial run does not evaluate it.",
    name: "ConfinedPackageUserNotReachingIt",
    purpose: "The users of a confined package are exactly the members that reach it, so the list states where the package is used. Remove the member from the users of the package.",
    summary: "A confined package user that never reaches the package",
};

pub const DISCARDED_ERROR: IssueText = IssueText {
    definition: "Code that drops an owned error, one of a Result whose error is not a reference, (), Infallible or !, or a value of an error type, without binding and reading it: a _ pattern or a _ = assignment that matches it, a .. rest pattern that leaves it out, an if let, while let or let else that matches only the Ok of a Result, a method of Result that takes it by value and whose signature names neither the error type nor Self other than to let the error be dropped, such as ok, unwrap_or or map_or, iterating a Result with into_iter or a for loop, flatten or flat_map over Results, drop of it, a method of Result whose signature never names the error type called on a Result that is not stored in a place, such as is_ok, is_err, iter or iter_mut, directly or through the methods that lend the error by reference, such as as_ref, and a binding of it that is never read.",
    name: "DiscardedError",
    purpose: "A dropped error hides the failure that caused it, so the program continues on a state it never checked and nothing explains what went wrong. Handle the error: return it, wrap it as the source of a more specific error, or match its variants.",
    summary: "An error that is dropped without being handled",
};

pub const DUPLICATED_FUNCTION_BODY: IssueText = IssueText {
    definition: "Functions written in the module files of workspace members whose signatures and bodies are identical once every name is replaced by the definition it resolves to, with names a function declares itself compared by declaration order and names that resolve to nothing never equal; a format or assembly template compares by its written text and by the definitions its placeholders capture, and the name of a named format argument by its spelling. Functions with an empty body are not compared, and neither are the entry points that Rust requires one of per target or per macro: the main function of a binary, a build script, an example, or a test or benchmark target without libtest's harness, and the functions of procedural macros.",
    name: "DuplicatedFunctionBody",
    purpose: "Duplicated code drifts apart when one copy changes. Keep one function and call it from every place that needs it.",
    summary: "Functions whose signatures and bodies are identical",
};

pub const ERROR_SOURCE_OUTSIDE_CHAIN: IssueText = IssueText {
    definition: "A field of a variant of an enum that derives thiserror::Error whose type implements std::error::Error and that is not the source of its variant, as thiserror decides it: the field marked #[source] or #[from], otherwise the field named source, or the single field of an #[error(transparent)] variant.",
    name: "ErrorSourceOutsideChain",
    purpose: "An error kept outside the source never reaches the error chain, so whoever reports the error loses its cause. Make the field the source of its variant, with #[source] or #[from], and give the variant no other error.",
    summary: "An error held outside the source chain",
};

pub const FAILED_MACRO_EXPANSION: IssueText = IssueText {
    definition: "A macro call whose expansion fails. Every failing expansion is listed within the macro call written in the source.",
    name: "FailedMacroExpansion",
    purpose: "A macro call that fails to expand produces no code to analyse. Fix the call so it expands.",
    summary: "A macro call that fails to expand",
};

pub const FUNCTION_USED_ONLY_BY_BENCHMARKS: IssueText = IssueText {
    definition: "A free or inherent function outside test and benchmark code whose references all lie in test or benchmark code, at least one of them in benchmark code, benchmark code being every benchmark target. Every function of a package with publish = false is checked; in a package that can be published, only those its crate does not expose to other crates.",
    name: "FunctionUsedOnlyByBenchmarks",
    purpose: "Production code holds only what production uses. Move a function that only benchmarks use, or benchmarks and tests, into a test crate, a package that test_crates in [workspace.metadata.heisenberg] lists.",
    summary: "A production function that only benchmarks use",
};

pub const FUNCTION_USED_ONLY_BY_TESTS: IssueText = IssueText {
    definition: "A free or inherent function outside test and benchmark code whose references all lie in test code. Every function of a package with publish = false is checked; in a package that can be published, only those its crate does not expose to other crates.",
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

pub const HANDWRITTEN_ERROR_TYPE: IssueText = IssueText {
    definition: "An implementation of std::error::Error that the Error derive of thiserror does not generate for an enum: one written by hand, one that another macro generates, and one that thiserror derives for a struct. thiserror is the thiserror-impl package from crates.io, whose Error derive generates the implementation.",
    name: "HandwrittenErrorType",
    purpose: "An error type written by hand repeats what thiserror generates, its messages, source chain and conversions, and drifts from it, and a struct cannot name which failure happened. Declare the error type as an enum with a variant for each failure, and derive thiserror::Error on it.",
    summary: "An error type that is not an enum deriving thiserror::Error",
};

pub const IGNORED_TEST: IssueText = IssueText {
    definition: "An ignore attribute, also one that a cfg_attr applies, written in the workspace or generated by one of its macros, which makes the test run skip its test.",
    name: "IgnoredTest",
    purpose: "A skipped test looks like part of the suite while it verifies nothing. Make the test pass, or remove it together with what it was meant to verify.",
    summary: "A test that the test run skips",
};

pub const IMPL_WITHOUT_LOCAL_TYPE: IssueText = IssueText {
    definition: "An impl of another crate's trait for a self type this crate does not define, with no single struct, enum or union of this crate among the trait's type arguments.",
    name: "ImplWithoutLocalType",
    purpose: "An impl of another crate's trait belongs beside the one type of this crate it serves. Implement the trait for a struct, enum or union of this crate, or for a foreign type with exactly one of them among the trait's type arguments.",
    summary: "An impl of another crate's trait with no home in this crate",
};

pub const INCLUDED_SOURCE_FILE: IssueText = IssueText {
    definition: "An include! of a file that no build script provides: one outside the OUT_DIR of its crate, which a build script writes or an out_dirs declaration stands in for. Included files are trusted as generated text and not analysed.",
    name: "IncludedSourceFile",
    purpose: "An included file is trusted as generated text, so code written by hand escapes every rule once it is included. Make the file a module, or have a build script generate it into OUT_DIR.",
    summary: "An include! of a file that is not a build script output",
};

pub const INVALID_MACRO_DEFINITION: IssueText = IssueText {
    definition: "A macro definition that rust-analyzer reports as invalid.",
    name: "InvalidMacroDefinition",
    purpose: "An invalid macro definition produces no code to analyse. Fix the definition.",
    summary: "A macro definition that is not valid",
};

pub const ITEM_OUT_OF_ORDER: IssueText = IssueText {
    definition: "An item of a module file or inline module, among the items written in module files, that follows an item of a later group. The groups, in order: module declarations; imports from vendor crates, which are std, core, alloc and every crate outside the workspace; imports from other workspace members; imports from this crate; private functions, constants, statics, macro_rules! macros and macro calls; private structs, enums, unions, type aliases and traits with their impl blocks; public items with their impl blocks; tests, the functions under the built-in test, bench or test_case attribute; inline modules. An import takes the group of the crate that the first segment of each of its paths resolves to, except that an import of a macro_rules! macro of its own module, which Rust allows only after the macro, takes the group of the macro; an impl block takes the group of the item of its file it belongs to, and an item is public when its visibility reaches beyond its own module. The item is reported at its name, with the first earlier item of a later group.",
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
    definition: "An allow or expect attribute of any lint, or a warn attribute that lowers a required lint or lint group, that the workspace writes, in module files, through a cfg_attr, through a macro of a workspace member or as output of a procedural macro of a workspace member, over code that holds text written in the module files of workspace members once its macros are expanded. A suppression that a vendored macro writes, or one over code that only a generator wrote, such as bindings that a build script writes and include! brings in, is not one.",
    name: "LintSuppression",
    purpose: "Suppressing lints is a slippery slope. Remove the attribute and fix the code the lint reports.",
    summary: "An attribute that suppresses or lowers lints over code written in the workspace",
};

pub const METHOD_OUT_OF_ORDER: IssueText = IssueText {
    definition: "A function of an impl block or of a trait definition, among the items written in module files, that follows a function it belongs before. The functions of an inherent impl and of a trait definition form groups, in order: constructors, which return Self, Option<Self> or Result<Self, _>, an async function by the output of its future, without a receiver or with a self receiver taken by value, by visibility, pub, pub(crate), pub(super) or pub(in path), and private; then the other functions, by the same visibilities. The functions of a trait definition are public. Within a group, names compare as Rust strings. The functions of a trait impl follow the order in which their trait declares them. The function is reported at its name, with the first earlier function it belongs before.",
    name: "MethodOutOfOrder",
    purpose: "Constructors come first and wider visibility before narrower, so the interface of a type reads from the top and each function is found by its name, and a trait impl reads in the order of its trait. Move the function before the function the message names.",
    summary: "A function of an impl block or trait that follows one it belongs before",
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
    definition: "A module other than a crate root whose file, or whose name for an inline module or the directory of a mod.rs, spells the words of none of its public items, reported at its first public item. A module spells the words of an item in order and in lowercase, each joined to the next by an underscore or by nothing. The words of a name are split at its underscores, before an uppercase letter that follows a lowercase letter or a digit, and before the last letter of an uppercase run that a lowercase letter follows, with digits kept in the word before them, so that pub struct OpenAIApiPath lives in open_ai_api_path.rs, openai_api_path.rs or openaiapipath.rs, while Ministral3ClusterParams does not live in ministral_3_cluster_params.rs.",
    name: "ModuleNotNamedAfterItsItem",
    purpose: "A module is named after the one item it exposes, so the file that defines an item is found from the item's name alone. Rename the module to the words of its public item, keeping each word whole and joining it to the next by an underscore or by nothing, or rename the item.",
    summary: "A module that is not named after its public item",
};

pub const MULTIPLE_ERROR_TYPES: IssueText = IssueText {
    definition: "A crate target that implements std::error::Error more than once, by hand or through derives and other macros, each implementation reported where it is written.",
    name: "MultipleErrorTypes",
    purpose: "A crate whose failures one error enum cannot describe does more than one thing, and its callers cannot tell from its signatures which concern failed. Split the crate along its error boundaries: move each error enum, with the code that fails with it, into its own crate, and wrap it in the crates that depend on it as a #[source] or #[from] variant of their own error enum.",
    summary: "A crate with more than one error type",
};

pub const MUST_USE_BENCHMARK_OUTPUT: IssueText = IssueText {
    definition: "A call or reference, in the code of a workspace member, to one of criterion's measurement loops that black-box and drop what the measured routine returns, Bencher::iter, Bencher::iter_batched, Bencher::iter_batched_ref, Bencher::iter_with_large_drop and their AsyncBencher counterparts, whose routine returns a must-use value: a value of a type whose definition carries #[must_use], such as Result, of an impl Trait or dyn Trait type whose trait or one of its supertraits carries #[must_use], such as Future or Iterator, a closure, or a Box, Pin, tuple or non-empty array holding one. The returned type is the loop's output type parameter as rust-analyzer infers it at the call, and it is reported at the name of the loop.",
    name: "MustUseBenchmarkOutput",
    purpose: "A must-use value that criterion drops hides an error or work that never ran, so the benchmark can measure a failure path or the construction of lazy work instead of the code. Handle the outcome inside the routine, for example by recording errors and failing the benchmark after measuring, or validate the input before measuring.",
    summary: "A measured routine whose must-use result criterion drops",
};

pub const NEGATED_CONFIGURATION: IssueText = IssueText {
    definition: "A cfg or cfg_attr predicate that negates a feature, test or debug_assertions. Code that the predicate leaves active in no declared configuration is listed within its issue, and the predicate of a module file that its own attributes configure out is reported at the declaration unless an analysis of that same declaration reports it where it is written.",
    name: "NegatedConfiguration",
    purpose: "Code that exists only without a feature, outside tests or outside debug builds escapes testing and makes features non-additive. Express the condition positively.",
    summary: "A cfg predicate that negates a feature, test or debug_assertions",
};

pub const NEXTEST_CONFIGURATION_MISSING: IssueText = IssueText {
    definition: "A workspace whose root holds no .config/nextest.toml, the only repository configuration that nextest reads, while its analysed code has an active #[test] that libtest's harness runs, so that its tests run under nextest's defaults, which never terminate a test. A partial run in which no analysed configuration runs such a test does not evaluate it.",
    name: "NextestConfigurationMissing",
    purpose: "Every test finishes within the limits, so that a slow test shows slow code instead of stalling the run. Add a .config/nextest.toml whose default profile sets a slow-timeout within the limits, and make the tests that exceed them faster.",
    summary: "A workspace without a nextest configuration",
};

pub const NEXTEST_OVERRIDE: IssueText = IssueText {
    definition: "A per-test override in a nextest profile, which changes the timeouts, retries or threads of the tests it selects ahead of every profile setting.",
    name: "NextestOverride",
    purpose: "Every test meets the same limits. Remove the override and make the tests it singles out faster.",
    summary: "A per-test override in the nextest configuration",
};

pub const NEXTEST_TIMEOUT_NOT_ENFORCED: IssueText = IssueText {
    definition: "A nextest profile whose slow-timeout flags a test as slow later than the slow limit, terminates it later than the termination limit or never, or counts a timed out test as passing. The default profile sets a slow-timeout, since the one nextest ships never terminates a test, and a profile that sets none inherits the slow-timeout of its parent.",
    name: "NextestTimeoutNotEnforced",
    purpose: "A test that runs longer than the limits shows slow code or a slow test. Make the code and the test faster instead of relaxing the timeout.",
    summary: "A nextest profile whose slow-timeout exceeds the limits",
};

pub const NON_BENCHMARK_ITEM_IN_BENCHMARK_TARGET: IssueText = IssueText {
    definition: "An item of a benchmark target other than its entry point, a test, a constant, an import, a module or a private item that only the file defining it uses, among the items of module files, including those that a macro_rules! macro of the crate writes: a macro call is judged by the items it expands to, and the items that procedural macros and the macros of other crates generate are trusted. A target with harness = false enters through the main function of its crate root. The impl blocks of a flagged type are listed within the type's issue.",
    name: "NonBenchmarkItemInBenchmarkTarget",
    purpose: "A helper used beyond its own benchmark file is a library, and libraries never live in a benches directory. Move it into a library crate, a test crate when only tests and benchmarks use it; when that crate depends on the package the benchmarks measure, move the benchmarks that use it there too, so package dependencies never form a cycle.",
    summary: "A benchmark target item that is neither its entry point nor a file-local helper",
};

pub const NON_CONSTANT_IN_CONSTANTS_MODULE: IssueText = IssueText {
    definition: "A public item other than a constant or a module inside the crate's constants module, crate::constants.",
    name: "NonConstantInConstantsModule",
    purpose: "The constants module is the single home of public constants. Move every other public item out of it.",
    summary: "A public item other than a constant in crate::constants",
};

pub const NON_DOC_COMMENT: IssueText = IssueText {
    definition: "A comment other than a doc comment, in a module file.",
    name: "NonDocComment",
    purpose: "Names and structure document the code; only doc comments, which documentation tooling consumes, are allowed, and only on the items they document. Rename or restructure until the comment is unnecessary, or make it a doc comment if it documents an item; a statement, an expression or an unsafe block has no comment form, and unsafe code exists only where a crate integrates foreign code.",
    summary: "A comment that is not a doc comment",
};

pub const NON_TEST_ITEM_IN_TEST_TARGET: IssueText = IssueText {
    definition: "An item of an integration test target other than a test, a constant, an import, a module or a private item that only the file defining it uses, among the items of module files, including those that a macro_rules! macro of the crate writes: a macro call is judged by the items it expands to, and the items that procedural macros and the macros of other crates generate are trusted. A test is a function under the built-in test, bench or test_case attribute, and a target with harness = false enters through the main function of its crate root. The impl blocks of a flagged type are listed within the type's issue.",
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
    definition: "A function that takes a struct, enum or union of its own crate, by value, by reference or behind a type that dereferences to it, such as Box, Rc or Arc, without belonging to it: it has no self receiver that its body reads, is neither declared by a trait nor part of a trait impl, does not construct the self type of its impl, directly, as the success of a Result or as the value of an Option, an async function by the output of its future, and is not a private helper used only in the file that defines it.",
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

pub const REFERENCED_TEST: IssueText = IssueText {
    definition: "A function under the built-in test, bench or test_case attribute that other code references.",
    name: "ReferencedTest",
    purpose: "The test harness calls every test, and a test that other code calls ties tests to each other. Move the code they share into a helper that each test calls.",
    summary: "A test that other code references",
};

pub const REEXPORT: IssueText = IssueText {
    definition: "A re-export, apart from the import that gives a macro_rules! macro of its own module that is not exported its only path.",
    name: "Reexport",
    purpose: "Every item is reachable through exactly one path. Import the item from where it is defined instead of re-exporting it.",
    summary: "A re-export",
};

pub const REQUIRED_CLIPPY_SETTING_MISSING: IssueText = IssueText {
    definition: "A clippy configuration that does not give each required setting the value it requires: true for the test allowances, which are the only way tests may use expect and unwrap, and false for avoid-breaking-exported-api, which clippy otherwise sets to true and so leaves the lints that would change a public signature silent on public items.",
    name: "RequiredClippySettingMissing",
    purpose: "Tests may fail fast with expect and unwrap, and the lints that clippy relaxes for public items by default must hold for them too. Set each required setting to the value it names.",
    summary: "A clippy configuration without a required setting",
};

pub const REQUIRED_LINT_NOT_DENIED: IssueText = IssueText {
    definition: "A workspace member whose lints, after workspace inheritance and in cargo's order, leave a required lint or lint group below deny, under either spelling of its name or a name clippy renamed to it, a warn level counting as deny while the warnings group is denied. Every member, a test crate included, denies the warnings group, the clippy all, nursery and pedantic groups and the panicking lints, the clippy lints that report code that can panic. A finding for a lints table is reported at the manifest that declares the table, once at the workspace manifest for every member that inherits [workspace.lints].",
    name: "RequiredLintNotDenied",
    purpose: "Every warning and every lint of the clippy groups fails the build, and code reports failures as typed errors instead of panicking, wherever it is written; tests, in every crate alike, fail fast only with expect and unwrap, which the required clippy settings allow in #[test] functions and #[cfg(test)] items. Deny the lint in the [lints] or [workspace.lints] table the package uses.",
    summary: "A workspace member that leaves a required lint or lint group below deny",
};

pub const SERIALIZATION_ROUND_TRIP_TEST: IssueText = IssueText {
    definition: "A test whose every assert_eq! or debug_assert_eq! compares a local value with what a serialization round trip of it decodes: data that a serde serializer, a function with a parameter bounded by Serialize, or ToString::to_string encodes from the value, and that a serde deserializer, a function returning a type bounded by Deserialize or DeserializeOwned, FromStr::from_str or str::parse decodes. The value is followed through single let initializers and unwrap, expect, ?, as_str, as_bytes, as_slice, references and parentheses. serde is the serde and serde_core packages from crates.io.",
    name: "SerializationRoundTripTest",
    purpose: "A round trip only shows that the serialization library is consistent with itself, which its own tests already cover, and it still passes when the encoded form is wrong. Test the behaviour that depends on the encoded form, such as the exact data it produces or the value a fixed input decodes to.",
    summary: "A test that only checks a serialization round trip",
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

pub const SLEEP_IN_TEST: IssueText = IssueText {
    definition: "A path in test code that resolves to std::thread::sleep or std::thread::sleep_ms, test code being an integration test target, a target of a test crate or a module that cfg(test) gates.",
    name: "SleepInTest",
    purpose: "A test that sleeps waits a guessed time instead of the event it needs, so it is slow when the guess is long and flaky when it is short. Wait for the event itself, such as by joining a thread, receiving from a channel or waiting for a child process.",
    summary: "Test code that sleeps",
};

pub const SOURCE_IN_ERROR_MESSAGE: IssueText = IssueText {
    definition: "An #[error(...)] message of an enum that derives thiserror::Error that formats the source of its variant itself, rather than a value that the source holds.",
    name: "SourceInErrorMessage",
    purpose: "The error chain reports the source after the message, so a message that formats the source as well prints the cause twice. Leave the source out of the message and let the error chain report it.",
    summary: "An error message that formats its own source",
};

pub const SPLIT_INTEGRATION_TESTS: IssueText = IssueText {
    definition: "A workspace member with integration test targets that libtest's harness runs, whose [package] table does not set autotests = false, or that has more than one such target. Targets with harness = false, which bring their own harness, are exempt.",
    name: "SplitIntegrationTests",
    purpose: "Every integration test target is its own binary, linked and started separately, which slows every test run and multiplies the long paths of build artifacts, and automatic discovery builds every file in tests/ as a target, including files that are meant to be modules. Set autotests = false, declare one [[test]] target, and include the other test files as its modules.",
    summary: "Integration tests spread over several targets or discovered automatically",
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
    definition: "A tests package, whose targets are integration tests, possibly with benchmark targets, beside an absent or empty library, a library being empty when it defines nothing in any declared configuration, and whose integration test and benchmark targets hold nothing but tests, entry points, constants, imports, modules and file-local private items, so that it holds no test library of its own. Integration tests belong to the package whose behaviour they verify, in its tests directory, and may use any dev-dependency; a helper used beyond its own test file is a test library, which belongs in a library crate; a separate tests package exists only to hold test libraries of its own, such as helpers that need the package under test, which that package cannot dev-depend on without a package cycle. The issue lists every workspace member the package depends on, either as a package that could take its tests or with the dependency cycle that moving them there would close, as guidance, not as the owner.",
    name: "TestsOnlyPackageWithoutTestLibraries",
    purpose: "Tests belong to the package whose behaviour they verify, in its tests directory. A separate tests package is justified only by test libraries of its own: shared helpers, which never live in a tests directory, and helpers that need the package under test, which that package cannot dev-depend on without a dependency cycle. Heisenberg cannot tell from the code which package the tests verify, so it lists every workspace member the tests use, either as a package that could take them or with the dependency cycle that moving them there would close; the right owner is the package they verify. If a test library they use depends on that package, the test library is entangled with the code it helps to test: move the helpers that need the package into this package's library, or make the test library independent of it.",
    summary: "A package of tests that holds no test library of its own",
};

pub const TEST_WITHOUT_ASSERTION: IssueText = IssueText {
    definition: "A test function that libtest runs, whose body, including the expansions of the macros it calls, calls none of the assertion macros of the standard library, assert!, assert_eq!, assert_ne!, debug_assert!, debug_assert_eq! and debug_assert_ne!, and that is not marked should_panic.",
    name: "TestWithoutAssertion",
    purpose: "A test without an assertion passes whenever its code does not panic, so it checks no result. Assert the values the tested code produces.",
    summary: "A test that asserts nothing",
};

pub const TUPLE_IN_SIGNATURE: IssueText = IssueText {
    definition: "A tuple type of two or more elements written in an item signature or type definition: the type of a function parameter, a return type, a field, a type alias, the type of a constant or static, an associated type, an impl header or a trait definition, nested types included. The functions and constants of a trait impl, whose types the trait dictates, and types written inside function bodies are not checked.",
    name: "TupleInSignature",
    purpose: "A tuple names none of its values, so its readers must remember what each position holds. Declare a struct with named fields instead.",
    summary: "A tuple type in an item signature",
};

pub const UNCOVERED_CODE: IssueText = IssueText {
    definition: "A region of production code that the coverage export records and that no test executed. A function that no test executes is reported at its name, a function that tests execute in part once at its name with every unexecuted region, and a region outside every function, such as one in a macro_rules! definition that only test code calls, at the region itself. A region counts as executed when any instrumented build executed it, whichever instantiation, crate type or function written from the same text it belongs to. Production code is the text written in the module files of every crate target that is neither an integration test target, a benchmark target nor a target of a test crate, active in the measured configuration and outside the modules that cfg(test) gates.",
    name: "UncoveredCode",
    purpose: "Every line of production code runs under the tests. Add tests that execute the code, or remove the code that nothing needs.",
    summary: "Production code that no test executes",
};

pub const UNDESTRUCTURED_PARAMETER: IssueText = IssueText {
    definition: "A function or closure parameter bound by an identifier, other than self, whose every use, at least one, accesses a field of the struct or tuple the parameter holds, directly or behind references, with the field name written in a module file. Fields reached through Box or another Deref implementation, and fields of unions, do not count, and neither does a parameter that holds by value a type implementing Drop and reads a field that is not Copy, which a pattern cannot move out of.",
    name: "UndestructuredParameter",
    purpose: "A parameter used only through its fields reads better as a pattern that names those fields. Destructure the parameter in its pattern.",
    summary: "A parameter used only through its fields",
};

pub const UNDETERMINED_BENCHMARK_OUTPUT: IssueText = IssueText {
    definition: "A call or reference to one of the measurement loops that MustUseBenchmarkOutput checks, whose routine returns a type that leaves undetermined whether it is a must-use value: a type parameter, or a type that rust-analyzer cannot infer, alone or where the must-use rule looks into it, such as inside a Box or a tuple.",
    name: "UndeterminedBenchmarkOutput",
    purpose: "Whether criterion drops a must-use value is decided where the routine is measured. Write the measured routine where it is measured, so that the type it returns is concrete.",
    summary: "A measured routine whose return type the analysis cannot determine",
};

pub const UNIMPLEMENTED_BUILTIN_MACRO: IssueText = IssueText {
    definition: "A call of a built-in macro that rust-analyzer does not implement.",
    name: "UnimplementedBuiltinMacro",
    purpose: "rust-analyzer cannot expand this built-in macro, so its output cannot be analysed. Replace the call with code that can be analysed.",
    summary: "A built-in macro that rust-analyzer cannot expand",
};

pub const UNKNOWN_CONFINED_PACKAGE: IssueText = IssueText {
    definition: "A confined_packages entry that names no package that the workspace resolves, member or dependency.",
    name: "UnknownConfinedPackage",
    purpose: "A confinement holds only for a package that the workspace resolves. Remove the entry from confined_packages, or correct the package name.",
    summary: "A confined package that the workspace does not resolve",
};

pub const UNMAPPED_ITEM: IssueText = IssueText {
    definition: "An item whose syntax rust-analyzer cannot map to its definition, such as an item of a block inside a type.",
    name: "UnmappedItem",
    purpose: "An item that cannot be mapped to its definition leaves the analysis incomplete. Write it so it maps to a definition.",
    summary: "An item that cannot be mapped to its definition",
};

pub const UNNEEDED_NEXTEST_CONFIGURATION: IssueText = IssueText {
    definition: "A .config/nextest.toml in a workspace whose analysed code has no active #[test] that libtest's harness runs, in any declared configuration: every test target brings its own harness. A partial run in which no analysed configuration runs such a test does not evaluate it.",
    name: "UnneededNextestConfiguration",
    purpose: "Nextest runs only the tests that libtest's harness defines, so a workspace whose tests all bring their own harness never uses its nextest configuration, and a configuration that nothing reads only suggests limits that nothing enforces. Remove the configuration.",
    summary: "A nextest configuration in a workspace without libtest tests",
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

pub const UNREACHABLE_TEST_FILE: IssueText = IssueText {
    definition: "A file in the tests directory of a workspace member that rust-analyzer loads and that is a module file of no crate target in any declared configuration. Evaluated only when every declared configuration is analysed.",
    name: "UnreachableTestFile",
    purpose: "A test file that no target includes is never compiled, so its tests never run and nothing reports it, which is the risk of declaring integration test targets by hand. Include the file as a module of the integration test target, or remove it.",
    summary: "A test file that no crate target includes",
};

pub const UNRESOLVED_EXTERN_CRATE: IssueText = IssueText {
    definition: "An extern crate declaration that rust-analyzer cannot resolve.",
    name: "UnresolvedExternCrate",
    purpose: resolution_purpose!("Declare the crate as a dependency or remove the declaration."),
    summary: "An extern crate that rust-analyzer cannot resolve",
};

pub const UNRESOLVED_MACRO_CALL: IssueText = IssueText {
    definition: "A macro call, attribute or derive whose macro rust-analyzer cannot resolve. A macro call whose name an unresolved import binds belongs to that import's issue.",
    name: "UnresolvedMacroCall",
    purpose: resolution_purpose!("Define or import the macro so that rust-analyzer resolves it."),
    summary: "A macro call, attribute or derive whose macro rust-analyzer cannot resolve",
};

pub const UNRESOLVED_METHOD_CALL: IssueText = IssueText {
    definition: "A method call whose method rust-analyzer cannot resolve, unless its receiver holds a name that rust-analyzer cannot resolve, which is that name's issue.",
    name: "UnresolvedMethodCall",
    purpose: resolution_purpose!(
        "Write the call so that rust-analyzer resolves its method, such as by naming the type of its receiver."
    ),
    summary: "A method call that rust-analyzer cannot resolve",
};

pub const UNRESOLVED_MODULE: IssueText = IssueText {
    definition: "A module declaration whose file rust-analyzer cannot resolve.",
    name: "UnresolvedModule",
    purpose: resolution_purpose!("Create the module file or remove the declaration."),
    summary: "A module declaration whose file rust-analyzer cannot resolve",
};

pub const UNRESOLVED_PATH: IssueText = IssueText {
    definition: "A path that rust-analyzer cannot resolve, imports included. A name bound by an unresolved import, whether a path or a macro call, belongs to that import's issue, and paths into a module of another crate that do not resolve are one issue per module, listed under a failed macro expansion that a workspace member reports in that module's file when there is one.",
    name: "UnresolvedPath",
    purpose: resolution_purpose!(
        "Write the path so that rust-analyzer resolves it, such as by importing the item it names."
    ),
    summary: "A path that rust-analyzer cannot resolve",
};

pub const UNSAFE_CODE: IssueText = IssueText {
    definition: "An unsafe block, function, trait, impl, extern block or attribute, written in the module files of a crate whose own code neither declares foreign functions or statics in an extern block, nor uses one, nor defines a function with a foreign ABI.",
    name: "UnsafeCode",
    purpose: "Unsafe code escapes the guarantees that make a project statically analyzable, and only code that meets a foreign library cannot avoid it. Write the code without unsafe; only a crate that integrates foreign code, by declaring or using foreign functions or statics, or by defining functions with a foreign ABI, may use it.",
    summary: "Unsafe code in a crate that integrates no foreign code",
};

pub const UNTYPED_RESULT_ERROR: IssueText = IssueText {
    definition: "A Result written in an item signature or type definition, such as the type of a function parameter, a return type, a field or a type alias, whose error type does not implement std::error::Error, such as String, &str, (), an integer, a collection, Box<dyn Error> or anyhow::Error, and an associated type of a trait impl that the trait names as the error of a Result, such as the Err of FromStr, set to such a type. A type parameter is checked where it is given a type, and a Result whose type the trait of an impl dictates is exempt.",
    name: "UntypedResultError",
    purpose: "An error that is not an error type carries no source chain and no variant that a caller can match, so callers can only print it or compare its text. Declare an error enum whose variants name each failure, and return it.",
    summary: "A Result whose error type is not an error",
};

pub const UNVETTED_CLIPPY_SETTING: IssueText = IssueText {
    definition: "A clippy configuration setting other than a required one and those that only add restrictions, which can silence or relax the lints that Heisenberg requires.",
    name: "UnvettedClippySetting",
    purpose: "A clippy setting can silence or relax the lints that Heisenberg requires, so a clippy configuration holds only the required settings and settings that only add restrictions. Remove the setting.",
    summary: "A clippy configuration setting that Heisenberg has not vetted",
};

pub const UNVETTED_CRITERION_METHOD: IssueText = IssueText {
    definition: "A call or reference, in the code of a workspace member, to an inherent method of criterion's Criterion, BenchmarkGroup, Bencher or AsyncBencher that Heisenberg has not vetted: one that changes how long or how many samples it measures, how it judges results or which benchmarks run, a method that criterion hides from its documentation, or a method that Heisenberg does not know. criterion is the criterion package from crates.io, and the methods of traits that these types implement are not inherent methods.",
    name: "UnvettedCriterionMethod",
    purpose: "Every benchmark is measured with criterion's defaults, the same way every time. Remove the call, and make the benchmark that needed it faster instead of shortening, weakening or skipping its measurement.",
    summary: "A criterion method that Heisenberg has not vetted",
};

pub const UNVETTED_NEXTEST_SETTING: IssueText = IssueText {
    definition: "A nextest configuration setting other than the slow-timeout and those that only control concurrency, failure handling and reporting, which can retry, skip or exempt tests, or let a test run past its timeout.",
    name: "UnvettedNextestSetting",
    purpose: "Every test runs and passes on its first attempt within the limits. Remove the setting, and make the tests that needed it faster or reliable.",
    summary: "A nextest configuration setting that Heisenberg has not vetted",
};

pub const DEFAULT_JOBS: NonZeroU16 = NonZeroU16::MIN.saturating_add(7);

pub const SUPPORTED_RUST_VERSION: &str = "1.99.1";

pub const TARGET_TRIPLE_PLACEHOLDER: &str = "<TRIPLE>";
