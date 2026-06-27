# qmd-lsp Agent Guide

This file records how to start or continue work on this project in future
conversations.

## Project Goal

Build a lightweight structural LSP for Quarto `.qmd` files in Rust. The first
target editor is Neovim.

The long-term features are:

- document structure navigation
- label, cross-reference, and citation parsing
- diagnostics
- completion
- go to definition
- Neovim integration through LSP

This is also a Rust learning project. Code changes should be small enough that
the user can understand and reimplement them.

## Architecture Principles

- Prefer an AST-first parser over a collection of independent extraction
  functions.
- The parser should consume the document from beginning to end.
- Preserve source content where possible.
- Put ordinary prose in `Paragraph` nodes.
- Put unrecognized or invalid but meaningful input in `Unknown` or `Malformed`
  nodes.
- Keep block-level AST separate from inline-level AST.
- Use traits for shared QMD node behavior such as `kind`, `range`, and
  `display_name`.
- Use enums and structs for concrete node types such as `Heading`, `Paragraph`,
  `CodeBlock`, and `InlineNode`.
- Derive `QmdIndex` from the AST for labels, crossrefs, citations,
  diagnostics, completion, and definition lookup.
- Keep LSP types out of parser, AST, document, and index core layers.
- Add a later conversion layer for `tower-lsp` types.

## Learning Collaboration Rules

When the user says "开始下一次学习":

1. Inspect the current code and `docs/next-lesson.md`.
2. Choose the smallest useful learning goal for this session.
3. Explain the Rust concept in plain but accurate language.
4. Output code changes in the conversation instead of applying them directly.
5. Include necessary test additions or modifications.
6. Ask the user to run, or run when appropriate:
   - `cargo fmt`
   - `cargo clippy`
   - `cargo test`
7. Update `docs/learning-log.md`.
8. Update `docs/next-lesson.md`.
9. End with a suggested Git commit message.

For normal maintenance requests, implement only what the user explicitly asks
for. For learning sessions, do not edit Rust source files directly unless the
user clearly asks for direct implementation.

## Constraints

- Do not do large rewrites unless the user explicitly asks for one.
- Do not introduce `tower-lsp` until the core parser, AST, document model, and
  index are stable enough.
- Avoid advanced generics, macros, and `Box<dyn Trait>` unless they solve a
  concrete current problem.
- Prefer readable Rust over clever Rust.
- Keep every lesson focused on one clear Rust concept.
- Keep the project compiling and testable after each small step.

## Current Priority

1. Stabilize the internal source range and node kind model.
2. Introduce a minimal `QmdNode` trait for shared node behavior.
3. Grow the block-level AST with `Heading`, `Paragraph`, `CodeBlock`, and
   `Unknown`/`Malformed` nodes.
4. Add inline parsing for crossrefs and citations after block parsing is clear.
5. Derive `QmdIndex` from the AST.
6. Add diagnostics, completion, and definition lookup from `QmdIndex`.
7. Add LSP conversion and Neovim integration later.

