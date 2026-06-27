# Next Lesson

## Goal

Introduce a minimal `QmdNode` trait and implement it for the existing
`Heading` type.

This should be a small trait-focused lesson, not a parser rewrite.

## Files To Read

- `src/element.rs`
- `src/document.rs`
- `src/range.rs`

## Rust Concepts

- `trait` as a shared behavior contract.
- Method signatures inside a trait.
- `impl TraitName for TypeName`.
- Returning a `Copy` value such as `QmdElementKind`.
- Returning an owned `String` from `display_name`.
- Using tests to confirm trait behavior.

## Minimal Implementation Task

- Define a trait, likely named `QmdNode`, near `QmdElementKind`.
- Give it methods similar to:
  - `kind(&self) -> QmdElementKind`
  - `range(&self) -> SourceRange`
  - `display_name(&self) -> String`
- Implement `QmdNode` for `Heading`.
- Add or adjust tests to call the trait methods on a `Heading`.

Do not:

- rewrite the parser
- introduce `Box<dyn Trait>`
- introduce advanced generics
- add `tower-lsp`
- add a full AST enum yet

## Check Commands

```bash
cargo fmt
cargo clippy
cargo test
```

## Suggested Commit Message

```text
feat: add minimal qmd node trait for headings
```
