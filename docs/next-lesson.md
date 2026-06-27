# Next Lesson

## Goal

After `Paragraph` has been implemented, add the smallest possible block AST enum
that can hold either a `Heading` or a `Paragraph`.

Start by confirming that Lesson 23's `Paragraph` changes are present and that
tests pass. If they are not present yet, finish Lesson 23 first.

## Files To Read

- `src/element.rs`
- `src/document.rs`
- `src/range.rs`

## Rust Concepts

- `enum` variants that hold data.
- Pattern matching with `match`.
- Delegating trait methods from an enum to its inner values.
- Keeping an AST enum small before adding parser behavior.

## Minimal Implementation Task

- Add an enum such as `BlockNode` with variants for `Heading` and `Paragraph`.
- Implement `QmdNode` for `BlockNode` by matching on each variant.
- Add tests showing that `BlockNode::Heading` and `BlockNode::Paragraph`
  delegate `kind`, `range`, and `display_name` correctly.

Do not:

- rewrite the full parser yet
- introduce `Box<dyn Trait>`
- introduce advanced generics
- add `tower-lsp`
- build `QmdIndex` yet

## Check Commands

```bash
cargo fmt
cargo clippy
cargo test
```

## Suggested Commit Message

```text
feat: add block node enum for heading and paragraph
```
