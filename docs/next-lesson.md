# Next Lesson

## Goal

If `QmdDocument` has not been implemented manually yet, finish Lesson 26 first:
add the smallest document model that owns the source text and stores the parsed
`Vec<BlockNode>` from `parse_blocks`.

If `QmdDocument` is already present and tests pass, the next small lesson should
add `CodeBlock` to the block-level AST shape without parsing fenced code blocks
yet.

## Files To Read

- `src/document.rs`
- `src/parser.rs`
- `src/element.rs`
- `src/range.rs`

## Rust Concepts

If finishing Lesson 26:

- Owning source text with `String`.
- Borrowing parser input as `&str`.
- Storing parsed enum nodes in `Vec<BlockNode>`.
- Calling one module's function from another module with `crate::parser::...`.

If moving on to the next AST node:

- Extending an enum with another data-carrying variant.
- Implementing a trait for a new concrete struct.
- Delegating trait methods from `BlockNode` with `match`.
- Using `Option<String>` for optional code block language.

## Minimal Implementation Task

If `QmdDocument` is missing:

- Import `parse_blocks` in `src/document.rs`.
- Add a minimal `QmdDocument` struct with:
  - `text: String`
  - `blocks: Vec<BlockNode>`
- Add `QmdDocument::parse(text: &str) -> Self`.
- Have `QmdDocument::parse` call `parse_blocks`.
- Add one small test showing that document parsing preserves the source text
  and stores parsed blocks.

If `QmdDocument` is already implemented:

- Reshape the existing `CodeBlock` into a block-level AST node with a
  `SourceRange`.
- Implement `QmdNode` for `CodeBlock`.
- Add `BlockNode::CodeBlock(CodeBlock)`.
- Update the `BlockNode` trait delegation match arms.
- Add one test showing that a `BlockNode::CodeBlock` delegates `kind`, `range`,
  and `display_name` to the inner `CodeBlock`.

Do not:

- parse fenced code blocks yet
- merge multi-line paragraphs yet
- add labels, citations, or crossrefs yet
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

If finishing Lesson 26:

```text
feat: store parsed block nodes in qmd document
```

If adding the `CodeBlock` AST shape:

```text
feat: add code block node to qmd ast model
```
