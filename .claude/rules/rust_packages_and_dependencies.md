---
kind: rule
section: 4
enforced_by:
  - ConfinedPackageReachedOutsideItsUsers
  - ConfinedPackageUserListedTwice
  - ConfinedPackageUserNotAMember
  - ConfinedPackageUserNotReachingIt
  - PackageDependencyCycle
  - PackageDependsOnItself
  - TestDependencyInProduction
  - TestLibraryInProductionPackage
  - UnknownConfinedPackage
  - UnpinnedDependencyRequirement
  - UnusedDependency
---

# Section 4: Packages and dependencies

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Package dependencies form a tree, dev-dependencies included; a package never depends on itself. [PackageDependencyCycle, PackageDependsOnItself]
- Write every dependency requirement as `MAJOR.MINOR`, or as `MAJOR.MINOR.PATCH` for a `0.0.z` or pre-release version; `Cargo.lock` fixes the exact version. A git dependency names its `rev`. Path dependencies without a version are exempt. [UnpinnedDependencyRequirement]
- A confined package is reached only by its declared users, each a workspace member that reaches it and is listed once. [ConfinedPackageReachedOutsideItsUsers, ConfinedPackageUserListedTwice, ConfinedPackageUserNotAMember, ConfinedPackageUserNotReachingIt, UnknownConfinedPackage]
- A dependency is declared in the role its code uses it in: every dependency is referred to by code its role serves, and a dependency that only tests, benchmarks or examples use is a dev-dependency. [UnusedDependency, TestDependencyInProduction]
- A production package declares no test library from outside the workspace: tests that need one live, with it, in a declared test crate. [TestLibraryInProductionPackage]
