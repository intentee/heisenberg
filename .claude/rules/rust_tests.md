---
kind: rule
section: 8
enforced_by:
  - IgnoredTest
  - ItemUsedOnlyByTests
  - NextestConfigurationMissing
  - NextestOverride
  - NextestTimeoutNotEnforced
  - NonTestItemInTestTarget
  - PublishableTestCrate
  - ReferencedTest
  - SplitIntegrationTests
  - TestCrateDeclaredTwice
  - TestCrateNotAMember
  - TestCrateUsedByProduction
  - TestModuleInItsOwnFile
  - TestModuleNotNamedTests
  - TestOnlyCodeInProduction
  - TestOutsideTestModule
  - TestsOnlyPackageWithoutTestLibraries
  - UnneededNextestConfiguration
  - UnvettedNextestSetting
---

# Section 8: Tests

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- In production code, `cfg(test)` appears only as `#[cfg(test)] mod tests { ... }`, inline in the file it tests: no test module in a file of its own, no other name, and no `cfg(test)` on a function, method, field, variant, import, statement or expression. [TestOnlyCodeInProduction, TestModuleInItsOwnFile, TestModuleNotNamedTests]
- A test in production code lives inside that test module. [TestOutsideTestModule]
- Production code holds no item that only tests use; move it into the test module that uses it or into a test crate. [ItemUsedOnlyByTests]
- No ignored tests, and no test that other code calls. [IgnoredTest, ReferencedTest]
- A package declares one integration test target, with `autotests = false`; the other files of `tests/` are its modules. [SplitIntegrationTests]
- A `tests/` directory holds tests and file-local helpers only; a helper shared between test files is a test library and lives in a library crate. [NonTestItemInTestTarget]
- A package that holds only tests exists only for test libraries of its own; otherwise the tests move into the package they verify. [TestsOnlyPackageWithoutTestLibraries]
- A test crate is a workspace member listed once in `test_crates`, with `publish = false`, that production code never depends on. [TestCrateDeclaredTwice, TestCrateNotAMember, PublishableTestCrate, TestCrateUsedByProduction]
- A workspace with libtest tests has a `.config/nextest.toml` whose default profile enforces the time limits, with vetted settings only and no per-test overrides; a workspace without them has none. [NextestConfigurationMissing, NextestTimeoutNotEnforced, NextestOverride, UnvettedNextestSetting, UnneededNextestConfiguration]
