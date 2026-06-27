# Next Lesson

## Goal

After the `QmdNode` trait has been implemented for `Heading`, add the next
smallest block-level AST node: `Paragraph`.

Start by confirming that Lesson 22's `QmdNode` changes are present and that
tests pass. If they are not present yet, finish Lesson 22 first.

## Files To Read

- `src/element.rs`
- `src/document.rs`
- `src/range.rs`

## Rust Concepts

- Reusing a trait across more than one concrete type.
- Designing a small struct with owned text.
- Choosing a source range for a multi-line or single-line text node.
- Avoiding premature parser rewrites while growing the AST model.

## Minimal Implementation Task

- Add a `Paragraph` struct with text and `SourceRange`.
- Implement `QmdNode` for `Paragraph`.
- Add tests for `Paragraph::display_name`, `kind`, and `range`.

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
feat: add paragraph node to qmd ast model
```
