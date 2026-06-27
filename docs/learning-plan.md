# qmd-lsp Learning Plan

This is a dynamic learning plan, not a fixed course schedule. The project should
adapt to the current code, current questions, and the smallest useful next step.

## Goals

- Learn Rust through a real Quarto `.qmd` language tooling project.
- Build a lightweight structural LSP core that can later serve Neovim.
- Keep each learning session small, testable, and easy to review.
- Prefer steady understanding over fast but opaque implementation.

## Architecture Direction

- Parse `.qmd` documents from beginning to end.
- Build a block-level AST that preserves as much source structure as possible.
- Keep inline parsing separate from block parsing.
- Use `Paragraph` for ordinary prose.
- Use `Unknown` or `Malformed` for input that should not be silently dropped.
- Use traits for shared node behavior.
- Use enums and structs for concrete AST nodes.
- Derive `QmdIndex` from the AST instead of parsing labels, crossrefs, and
  citations independently forever.
- Keep LSP-specific types outside the core parser, AST, document, and index
  layers.

## Capability Modules

- Rust basics: ownership, borrowing, structs, enums, pattern matching,
  iterators, modules, tests.
- Source model: `SourcePosition`, `SourceRange`, character offsets, later UTF-16
  conversion for LSP.
- AST model: node kinds, shared node trait, block nodes, inline nodes.
- Parser: line scanning, state machines, fenced blocks, malformed input,
  full-document coverage.
- Index: labels, references, citations, diagnostics, completion candidates,
  definition targets.
- LSP boundary: conversion from internal types to `tower-lsp` types.
- Editor integration: Neovim configuration and workflow after the server core is
  useful.

## Current Priority

1. Keep the project compiling and tests passing.
2. Add a minimal `QmdNode` trait for shared node behavior.
3. Implement that trait first for `Heading`.
4. Add `Paragraph` and `Unknown`/`Malformed` nodes after the trait shape is
   clear.
5. Rebuild parser behavior around a full-document block AST.
6. Move label, crossref, and citation lookup into `QmdIndex`.

## Dynamic Adjustment Principles

- Start each session by checking the current code and `docs/next-lesson.md`.
- Choose one small goal that improves the project and teaches one Rust concept.
- If the code is broken, repair the smallest broken part first.
- If the next planned lesson no longer matches the code, update the lesson
  instead of forcing the old plan.
- Prefer explicit, readable code while learning.
- Delay abstractions until repeated structure makes the need clear.
- Do not introduce `tower-lsp` until the internal model is stable enough.
- Record what actually happened in `docs/learning-log.md`.

