---
kind: rule
section: 9
enforced_by:
  - BenchmarkTargetExcludedFromBenchmarks
  - BenchmarkTargetRunByTests
  - BenchmarkTargetWithLibtestHarness
  - BenchmarkTargetWithoutCriterionBenchmark
  - BenchmarksOnlyPackageWithoutLibraries
  - ItemUsedOnlyByBenchmarks
  - MustUseBenchmarkOutput
  - NonBenchmarkItemInBenchmarkTarget
  - UndeterminedBenchmarkOutput
  - UnvettedCriterionMethod
---

# Section 9: Benchmarks

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- A benchmark target is declared with `harness = false`, runs under `cargo bench` and never under the test run, and registers its benchmarks with criterion. [BenchmarkTargetWithLibtestHarness, BenchmarkTargetExcludedFromBenchmarks, BenchmarkTargetRunByTests, BenchmarkTargetWithoutCriterionBenchmark]
- Benchmarks are measured with criterion's defaults; only vetted criterion methods are called. [UnvettedCriterionMethod]
- A measured routine returns a concrete type, and never a must-use value that criterion drops. [MustUseBenchmarkOutput, UndeterminedBenchmarkOutput]
- A `benches/` directory holds entry points and file-local helpers only. [NonBenchmarkItemInBenchmarkTarget]
- A package that holds only benchmarks exists only for a library of its own. [BenchmarksOnlyPackageWithoutLibraries]
- Production code holds no item that only benchmarks use. [ItemUsedOnlyByBenchmarks]
