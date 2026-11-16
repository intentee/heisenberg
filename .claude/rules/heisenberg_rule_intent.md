# Rules Follow Their Intent

- A rule enforces what it is for, not only its wording. Its check, its message and its explanation must agree with that intent, and a construct that satisfies the intent is compliant.
- Heisenberg never fights Rust. Every rule must have a compliant form that Rust accepts; when the language leaves no other form, the rule does not apply to that construct.
- The fix a rule names must compile. When the obvious fix does not, the rule names the form that does.
- An issue is one defect: it is reported once, however many configurations or crate targets show it and however its details differ between them.
- Messages name things as the source writes them, and where nothing is written, by their full path.
