---
kind: rule
section: 6
enforced_by:
  - DuplicatedFunctionBody
  - NonDocComment
  - PositionalFields
  - ProceduralFunction
  - SingleLetterName
  - TupleInSignature
  - UndestructuredParameter
  - UnprefixedTypeParameter
---

# Section 6: Names, data and functions

Every statement is mandatory and is decided by the Heisenberg check named after it; `heisenberg explain <check>` gives its exact definition.

- No single-character names for parameters, lifetimes or type parameters; a lifetime is named after what it borrows, such as `'message`. [SingleLetterName]
- A type parameter is named `T` followed by a capitalized word, such as `TNode`. [UnprefixedTypeParameter]
- No tuple types in item signatures, and no struct or enum variant with two or more positional fields: return and store structs with named fields. A single-field newtype is fine. [TupleInSignature, PositionalFields]
- Destructure a parameter that is used only through its fields. [UndestructuredParameter]
- No comments other than doc comments; names and structure document the code. [NonDocComment]
- A function that takes a type of its crate is a method of that type, a constructor of its own type, or a private helper of a single file; otherwise the code is restructured so that the function no longer needs a value of that type. [ProceduralFunction]
- No two functions with identical signatures and bodies; keep one and call it. [DuplicatedFunctionBody]
