# Learning Log

This file records my Rust learning progress while building qmd-lsp.

## Lesson 01: Create qmd-lsp project

Date: 2026-06-04

### What I learned

- Installed Rust with rustup.
- Created a Rust project with cargo.
- Ran the first Rust program with cargo run.

### Notes

- `rustc` is the Rust compiler.
- `cargo` is the Rust package and build tool.
- `rustup` manages Rust toolchains.

## Lesson 02: Add qmd file detection

Date: 2026-06-04

### What I learned

- Defined a function with `fn`.
- Used `&str` as a string parameter.
- Used `bool` as the return type.
- Used `if` and `for`.

### Notes

- Variables are immutable by default.
- The last expression in a function can be returned without `return`.
- `path.ends_with(".qmd")` returns a boolean.

## Lesson 03: Add README and learning log

Date: 2026-06-05

### What I learned

- Added project documentation.
- Added a learning log.
- Used basic Cargo development commands.

### Notes

- `cargo check` checks whether the project can compile.
- `cargo fmt` formats Rust code.
- `cargo clippy` gives code quality suggestions.

## Lesson 04: Learn String and &str

Date: 2026-06-05

### What I learned

- `String` owns string data.
- `&str` borrows string data.
- Parser functions usually accept `&str`.
- `text.lines()` can iterate over lines in a text document.
-rRaw string literals are useful for writing multi-line QMD examples.

### Notes

- Use `print_lines(&qmd)` when `qmd` is a `String`.
- Use `print_lines(qmd)` when `qmd` is already a string literal `&str`.
- `lines()` does not include the trailing newline character.

## Lesson 05: Parse Markdown headings

Date: 2026-06-05

### What I learned

- Used `Option` to represent a value that may or may not exist.
- Used `Some` and `None`.
- Used `starts_with()` to check the beginning of a string.
- Used `trim_start()` and `trim()` to remove whitespace.
- Used `chars()`, `take_while()`, and `count()` to count heading markers.
- Used `match` to handle `Option`.

### Notes

- `Option<T>` is useful when parsing may fail.
- `Some(value)` means parsing succeeded.
- `None` means parsing failed or the line is not a heading.
- Markdown headings usually have levels from 1 to 6.
