# Next Lesson

## Goal

If `BlockNode` has not been implemented manually yet, finish Lesson 24 first:
add the smallest possible block AST enum that can hold either a `Heading` or a
`Paragraph`.

If `BlockNode` is already present and tests pass, the next small lesson should
connect that enum to parser behavior by returning a `Vec<BlockNode>` for a very
small input that contains only headings and paragraphs.

## Files To Read

- `src/element.rs`
- `src/document.rs`
- `src/range.rs`
- `src/parser.rs`

## Rust Concepts

- `enum` variants that hold data.
- Pattern matching with `match`.
- Delegating trait methods from an enum to its inner values.
- Keeping an AST enum small before adding parser behavior.
- Returning a vector of enum values such as `Vec<BlockNode>`.

## Minimal Implementation Task

If `BlockNode` is missing:

- Add an enum such as `BlockNode` with variants for `Heading` and `Paragraph`.
- Implement `QmdNode` for `BlockNode` by matching on each variant.
- Add tests showing that `BlockNode::Heading` and `BlockNode::Paragraph`
  delegate `kind`, `range`, and `display_name` correctly.

If `BlockNode` is already implemented:

- Add the smallest parser-facing function that can produce `Vec<BlockNode>`
  from headings and ordinary paragraph lines.
- Keep code blocks, labels, citations, and malformed input out of scope for the
  first version.

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
