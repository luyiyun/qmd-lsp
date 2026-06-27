# qmd-lsp

A lightweight structural language server project for Quarto `.qmd` files.

This repository is also a Rust learning project. The goal is to grow the code
in small, understandable steps while building a useful tool for Neovim.

## Goals

- Parse Quarto `.qmd` documents into a structural AST.
- Support document structure navigation.
- Parse labels, cross-references, and citations.
- Provide diagnostics, completion, and go to definition.
- Integrate with Neovim through LSP once the core model is stable.

## Architecture Direction

- The parser should consume the `.qmd` text from beginning to end.
- The parser should build an AST that preserves as much of the source document
  as possible.
- Normal prose should become `Paragraph` nodes.
- Unrecognized but important source fragments should become `Unknown` or
  `Malformed` nodes instead of being silently dropped.
- Block-level AST and inline-level AST should stay separate.
- Shared QMD node behavior should be expressed with traits such as `kind`,
  `range`, and `display_name`.
- Concrete node variants should be represented with enums and structs such as
  `Heading`, `Paragraph`, `CodeBlock`, and `InlineNode`.
- `QmdIndex` should be derived from the AST and used for labels, crossrefs,
  citations, diagnostics, completion, and definition lookup.
- LSP types should not leak into `parser`, `ast`, or `document` core layers.
  Later, a conversion layer can translate internal types into `tower-lsp` types.

## Learning Workflow

This project is intentionally developed as a sequence of small lessons. Each
lesson should focus on one Rust concept and one minimal project improvement.

When continuing the project, read:

- `agent.md`
- `docs/learning-plan.md`
- `docs/learning-log.md`
- `docs/next-lesson.md`

## Development

Run the project with:

```bash
cargo run
```

Check the project with:

```bash
cargo fmt --check
cargo clippy
cargo test
```
