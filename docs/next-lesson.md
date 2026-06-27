# Next Lesson

## Goal

If `parse_blocks` has not been implemented manually yet, finish Lesson 25
first: add the smallest parser function that turns headings and ordinary
non-empty lines into a `Vec<BlockNode>`.

If `parse_blocks` is already present and tests pass, the next small lesson
should connect the block AST to a minimal document model by adding a
`QmdDocument` that owns the source text and the parsed `Vec<BlockNode>`.

## Files To Read

- `src/parser.rs`
- `src/document.rs`
- `src/element.rs`
- `src/range.rs`

## Rust Concepts

- Returning `Vec<BlockNode>` from a parser function.
- Using `filter_map` when a parser can skip blank lines.
- Keeping public parser functions and private helper functions separate.
- Pattern matching in tests to inspect enum variant data.
- Letting a document struct own parsed blocks after the parser produces them.

## Minimal Implementation Task

If `parse_blocks` is missing:

- Replace the unused imports at the top of `src/parser.rs`.
- Add `parse_blocks(text: &str) -> Vec<BlockNode>`.
- Add a private `parse_block_line(line: &str, line_no: u32) ->
  Option<BlockNode>` helper.
- Parse a line as `Heading` first.
- Skip blank lines.
- Parse any remaining non-empty line as `Paragraph`.
- Add tests for heading/paragraph output and blank-line skipping.

If `parse_blocks` is already implemented:

- Add a minimal `QmdDocument` struct with:
  - `text: String`
  - `blocks: Vec<BlockNode>`
- Add `QmdDocument::parse(text: &str) -> Self`.
- Have `QmdDocument::parse` call `parse_blocks`.
- Add one small test showing that document parsing preserves the source text
  and stores parsed blocks.

Do not:

- merge multi-line paragraphs yet
- parse code blocks yet
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

If finishing Lesson 25:

```text
feat: parse heading and paragraph block nodes
```

If moving on to the document model:

```text
feat: store parsed block nodes in qmd document
```
