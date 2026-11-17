---
kind: rule
section: 5
enforced_by:
  - AdditionalPublicItem
  - GlobImport
  - ImplWithoutLocalType
  - ImportInsideBlock
  - ItemOutOfOrder
  - MethodOutOfOrder
  - MisplacedBlanketImpl
  - MisplacedForeignTypeImpl
  - MisplacedImpl
  - MisplacedTraitObjectImpl
  - ModuleCycle
  - ModuleNotNamedAfterItsItem
  - NonConstantInConstantsModule
  - PublicAssociatedConstant
  - PublicConstantOutsideConstantsModule
  - Reexport
---

# Section 5: Modules and items

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- Each module exposes at most one public item, and the module is named after it. `src/constants.rs` is the exception: it holds every public constant of the crate and no other public item. [AdditionalPublicItem, ModuleNotNamedAfterItsItem, NonConstantInConstantsModule, PublicConstantOutsideConstantsModule, PublicAssociatedConstant]
- Each file follows this order: module declarations; imports from outside the workspace; imports from other workspace members; imports from this crate; private functions, constants, statics and macros; private types with their impl blocks; the public item with its impl blocks, or the `main` function; tests; inline modules. [ItemOutOfOrder]
- Within an impl block: constructors first, then `pub`, `pub(crate)` and private functions, each group sorted by name. A trait impl follows the order of its trait. [MethodOutOfOrder]
- An impl block lives in the file of the type it belongs to; a blanket impl, an impl for a foreign type and an impl for a trait object live in the file of their trait; an impl of a foreign trait serves exactly one type of this crate. [MisplacedImpl, MisplacedBlanketImpl, MisplacedForeignTypeImpl, MisplacedTraitObjectImpl, ImplWithoutLocalType]
- Import every name explicitly, at the top of the module: no glob imports, no imports inside blocks, no re-exports. [GlobImport, ImportInsideBlock, Reexport]
- Modules never depend on each other in a cycle. [ModuleCycle]
