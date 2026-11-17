---
kind: rule
section: 1
enforced_by:
  - ConfigurationSelectionMacro
  - FailedMacroExpansion
  - IncludedSourceFile
  - InvalidMacroDefinition
  - ModuleFileInsideBlock
  - SharedModuleFile
  - UnimplementedBuiltinMacro
  - UnmappedItem
  - UnparsableMacroExpansion
  - UnreachableSourceFile
  - UnresolvedExternCrate
  - UnresolvedMacroCall
  - UnresolvedMethodCall
  - UnresolvedModule
  - UnresolvedPath
  - UnsafeCode
---

# Section 1: Analysable code

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Every name resolves: extern crates, modules, paths, method calls, macro calls, attributes and derives. [UnresolvedExternCrate, UnresolvedModule, UnresolvedPath, UnresolvedMethodCall, UnresolvedMacroCall]
- Every macro definition is valid, and every macro call expands to code that parses. [InvalidMacroDefinition, FailedMacroExpansion, UnparsableMacroExpansion, UnimplementedBuiltinMacro]
- Every item maps to its definition. [UnmappedItem]
- Never use `cfg_select!`; write each arm as an item under its own `#[cfg]` attribute. [ConfigurationSelectionMacro]
- `include!` brings in only build script output from `OUT_DIR`; code written by hand is a module. [IncludedSourceFile]
- A module file is declared at module level, never inside a block, and belongs to exactly one module. [ModuleFileInsideBlock, SharedModuleFile]
- Every `.rs` file of a workspace member is a module file of a crate target. [UnreachableSourceFile]
- No unsafe code, except in a crate that integrates foreign code. [UnsafeCode]
