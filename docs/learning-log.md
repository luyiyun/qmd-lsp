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

## Lesson 06: Represent headings with struct

Date: 2026-06-05

### What I learned

- Used `struct` to represent a Markdown heading.
- Added fields to `Heading`: `level`, `title`, `line`, and `character`.
- Used `#[derive(Debug, Clone)]` to automatically implement useful traits.
- Used `impl` to define functions related to a struct.
- Used `Self` inside an `impl` block to refer to the current type.
- Changed `parse_heading` from returning `Option<(u8, String)>` to returning `Option<Heading>`.
- Learned that Rust does not allow direct string indexing like `rest[0]`.
- Used `starts_with(' ')` to check whether a heading marker is followed by a space.
- Learned why derived `Debug` printing does not count as reading struct fields in dead code analysis.

### Notes

- `struct` makes parsed data clearer than a tuple.
- `Heading` owns its title by using `String`.
- `line` and `character` are useful for future LSP features.
- `#[derive(Debug)]` allows printing a struct with `{:?}`.
- `#[derive(Clone)]` allows explicitly cloning a struct.
- `impl Heading` is a good place to define a constructor such as `Heading::new`.
- `Self` means the current type inside an `impl` block.
- Rust strings are UTF-8, so they cannot be indexed directly by integer.
- `println!("{:?}", heading)` is useful for debugging, but it does not prevent dead code warnings for unused fields.
- To avoid the warning, read the fields directly, such as `heading.level`, `heading.title`, `heading.line`, and `heading.character`.

## Lesson 08: Classify label kinds with enum

Date: 2026-06-05

### What I learned

- Used `enum` to represent different QMD label kinds.
- Defined `LabelKind` variants such as `Figure`, `Table`, `Equation`, `Section`, `Listing`, `Theorem`, `Proof`, and `Unknown`.
- Used `#[derive(Debug, Clone, PartialEq, Eq)]` to automatically implement useful traits.
- Used `PartialEq` and `Eq` to compare enum values with `==`.
- Used `impl LabelKind` to define functions related to the enum.
- Used `Self::Figure`, `Self::Table`, and other variants inside an `impl` block.
- Learned that `Self` means the current type, while `self` means the current value.
- Used `match self` to return different values for different enum variants.
- Learned that `&'static str` is a reference to string data that is valid for the whole program.
- Added methods such as `as_str()`, `display_name()`, and `icon()` for label display.

### Notes

- `enum` is useful when a value can only be one of several known variants.
- `LabelKind::Figure` represents the figure label kind.
- `Self::Figure` is equivalent to `LabelKind::Figure` inside `impl LabelKind`.
- `self` refers to the current enum value when calling a method.
- `match` must cover all enum variants.
- `PartialEq` allows comparing values such as `kind == LabelKind::Figure`.
- `Eq` means equality comparison is complete and has no special exception.
- `&'static str` is suitable for returning fixed string literals such as `"Figure"` or `"unknown"`.
- `as_str()` can be used for machine-readable names such as `"figure"`.
- `display_name()` can be used for user-facing names such as `"Figure"`.
- `icon()` can return a fixed icon for each label kind.
