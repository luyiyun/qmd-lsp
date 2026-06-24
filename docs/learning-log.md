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

## Lesson 09: Parse QMD labels with regex

Date: 2026-06-06

### What I learned

- Added the external `regex` crate to the Rust project.
- Used `Regex::new()` to create a regular expression.
- Learned that `Regex::new()` returns a `Result`, so it may be `Ok` or `Err`.
- Used `unwrap()` to extract the `Regex` from `Ok`, while understanding that it will panic on `Err`.
- Used `captures_iter()` to find all regex matches in a line.
- Learned that each match result is represented by `Captures`.
- Used `cap.get(0)` to get the whole matched text.
- Used `cap.get(1)` to get the first captured group.
- Used `Match::as_str()` to get the matched string slice.
- Used `Match::start()` to get the starting position of a match.
- Defined `LabelDef` to represent a QMD label definition.
- Reused `LabelKind::from_label()` to classify labels by prefix.
- Learned the basic idea of `match` and `match guard`.

### Notes

- `Regex` represents a compiled regular expression.
- `captures_iter()` returns an iterator over all matches in the input text.
- `Captures` stores one full match and its capture groups.
- `Match` represents a specific matched text range.
- `cap.get(0)` returns the whole match, such as `{#fig-overview}`.
- `cap.get(1)` returns the first captured group, such as `fig-overview`.
- `Some`, `None`, `Ok`, and `Err` are enum variants.
- `Option<T>` represents a value that may or may not exist.
- `Result<T, E>` represents an operation that may succeed or fail.
- `match` is Rust's pattern matching syntax and is especially useful for enums.
- `match guard` uses `if` after a pattern to add extra matching conditions.
- Regex is suitable for small and regular patterns, but not for parsing the full QMD or Markdown structure.

## Lesson 11: Learn ownership, move, and clone

Date: 2026-06-06

### What I learned

- Rust values have clear ownership.
- Assigning a `String` to another variable moves ownership.
- After move, the original variable can no longer be used.
- `clone()` creates a real copy of heap data.
- Small types such as `u8`, `u32`, `bool`, and `char` are usually `Copy`.
- Borrowing with `&str` allows a function to read text without taking ownership.

### Notes

- `String` is not `Copy` because it owns heap data.
- Parser functions should usually accept `&str`.
- `.to_string()` is appropriate when parsed data must be stored in a struct.
- `.clone()` should not be used just to avoid ownership errors.
- In `parse_heading`, converting the parsed title to `String` is reasonable because `Heading` owns its title.
- In `parse_labels`, converting the captured label to `String` is reasonable because `LabelDef` owns its label.

## Lesson 12: Learn references and borrowing

Date: 2026-06-06

### What I learned

- A reference lets a function use a value without taking ownership.
- `&T` is an immutable reference.
- `&mut T` is a mutable reference.
- Parser functions should usually borrow input text instead of owning it.
- `&str` is more general than `&String`.
- Multiple immutable references can exist at the same time.
- Only one mutable reference can exist at a time.

### Notes

- Use `fn parse_xxx(text: &str) -> ...` for read-only parser functions.
- Use owned `String` in parsed structs when the data needs to be stored.
- `parse_heading(line: &str, ...)` borrows the input line.
- `Heading { title: String }` owns the parsed title.
- Borrowing avoids unnecessary moves and unnecessary cloning.

## Lesson 14: Learn iterators, closures, and reference patterns

Date: 2026-06-06

### What I learned

- Used iterator chains to rewrite `parse_headings`.
- Used `.lines()` to iterate over document lines.
- Used `.enumerate()` to get both line number and line text.
- Used `.filter_map()` to keep only successfully parsed headings.
- Learned closure syntax such as `|x| ...` and `|(line_no, line)| ...`.
- Learned that `map`, `filter`, and `filter_map` have different closure input and output types.
- Learned the difference between `.iter()` and `.into_iter()`.
- Learned how to handle references inside closure parameters, such as `|&x|` and `|&&x|`.

### Notes

- `map` transforms each item into another item.
- `filter` uses a closure returning `bool` to decide whether to keep the original item.
- `filter_map` combines filtering and mapping by using a closure that returns `Option<T>`.
- If the iterator item is `T`, then:

  - `map` receives `T` and returns a new item.
  - `filter` receives `&T` and returns `bool`.
  - `filter_map` receives `T` and returns `Option<U>`.
- `.iter()` borrows elements and usually produces references such as `&T`.
- `.into_iter()` consumes the collection and produces owned values such as `T`.
- `|x| *x` means the reference is handled inside the closure body.
- `|&x| x` means the reference is destructured in the closure parameter.
- Parameter destructuring such as `|&x|` is suitable for `Copy` types like `i32`, `usize`, `bool`, `char`, and `&str`.
- For non-`Copy` types such as `String`, avoid using `|&x|` to move values out of borrowed references.
- Function parameters such as `fn parse(line: &str)` mean the function borrows a string slice.
- Closure patterns such as `|&x|` mean the closure destructures a reference.
- These two uses of `&` look similar but have different meanings.
- Rust can automatically dereference values in method calls, such as calling `.parse()` on `&&str`.
- Manual dereferencing and automatic dereferencing usually have no meaningful performance difference in these simple cases.
- For learning, writing explicit dereferencing such as `*x` or `**x` helps make the real types clearer.

### Code example

```rust
let nums = vec![1, 2, 3, 4];

let doubled: Vec<i32> = nums
    .iter()
    .map(|&x| x * 2)
    .collect();

println!("{:?}", doubled);

let evens: Vec<&i32> = nums
    .iter()
    .filter(|x| **x % 2 == 0)
    .collect();

println!("{:?}", evens);

let texts = vec!["Hello", "1", "abs", "3"];

let parsed_nums: Vec<i32> = texts
    .iter()
    .filter_map(|&s| s.parse::<i32>().ok())
    .collect();

println!("{:?}", parsed_nums);
```

### Project decision

`parse_headings` can be written in iterator style:

```rust
pub fn parse_headings(text: &str) -> Vec<Heading> {
    text.lines()
        .enumerate()
        .filter_map(|(line_no, line)| parse_heading(line, line_no as u32))
        .collect()
}
```

This is suitable because `parse_heading` returns `Option<Heading>`, so `filter_map` naturally keeps `Some(heading)` and skips `None`.

For more complex stateful parsers, such as fenced code blocks or YAML front matter, a normal `for` loop may still be clearer.

## Lesson 15: Split code into modules

Date: 2026-06-06

### What I learned

- Split the project code from `main.rs` into separate Rust modules.
- Created `document.rs` for data structures such as `Heading`, `LabelDef`, and `LabelKind`.
- Created `parser.rs` for parser functions such as `parse_heading`, `parse_headings`, `parse_labels`, and `parse_all_labels`.
- Used `mod` to declare modules in the crate root.
- Used `pub` to expose structs, enums, fields, and functions across modules.
- Used `use crate::...` to import items from another module.
- Learned that `main.rs` is the crate root in a binary crate.
- Learned why sibling modules should usually be accessed with absolute paths such as `crate::document::Heading`.
- Learned the basic meaning of the `?` operator for `Option` and `Result`.

### Notes

- `mod parser;` declares the `parser` module and tells Rust to compile `src/parser.rs`.
- `use parser::parse_headings;` only brings a name into scope; it does not declare a module.
- Rust items are private by default.
- A `pub struct` does not automatically make its fields public, so fields such as `level`, `title`, `line`, and `character` also need `pub` if other modules should access them.
- `crate::document::Heading` means starting from the crate root and then finding the `document` module.
- In `parser.rs`, `document` and `parser` are sibling modules, so using `crate::document::...` is clearer than trying to access `document` directly.
- The `?` operator means: unwrap the value if it is `Some` or `Ok`; otherwise, return `None` or `Err` early from the current function or closure.
- In `filter_map`, using `cap.get(1)?` is a concise way to skip invalid regex captures.

### Code structure

```text
src/
├── main.rs
├── document.rs
└── parser.rs
```

### Key distinction

```rust
mod parser;
```

declares the module.

```rust
use parser::parse_headings;
```

imports a function name into the current scope.

### Checks

```bash
cargo fmt
cargo clippy
cargo test
cargo run
```

## Lesson 16: Create QmdDocument abstraction

Date: 2026-06-07

### What I learned

- Created a `QmdDocument` struct to represent a parsed QMD document.
- Stored the original document text, headings, and labels together in one document model.
- Implemented `QmdDocument::parse` as an associated function.
- Changed `QmdDocument::parse` to accept `&str`, making it easier to call with string literals, `String`, or `&String`.
- Learned that `QmdDocument` still needs to own its text, so `parse` converts `&str` into `String` internally.
- Reviewed the difference between associated functions such as `QmdDocument::parse(...)` and methods such as `doc.parse()`.

### Notes

- `QmdDocument::parse(text)` creates a new parsed document from raw QMD text.
- `Self { ... }` inside `impl QmdDocument` is equivalent to `QmdDocument { ... }`.
- Accepting `&str` makes the API more flexible, while storing `text: String` keeps the parsed document self-contained.
- Parser functions can borrow the input text, and the document model can keep its own owned copy for future LSP features.

## Lesson 17: Parse fenced code blocks

Date: 2026-06-08

### What I learned

- Used a simple state machine to parse multi-line code blocks.
- Used `Option<CodeBlock>` to represent whether the parser is currently inside a code block.
- Added a `CodeBlock` struct with language, label, start line, and end line.
- Added `code_blocks` to `QmdDocument`.
- Used `Option::take()` to move a value out of an `Option` and reset it to `None`.

### Notes

- Heading and label parsing can usually be done line by line.
- Code block parsing needs parser state because the start and end are on different lines.
- `Option<String>` is suitable for fields that may not exist, such as code block language or label.
- In this lesson, code block labels are not parsed yet. That will be handled in the next lesson.

## Lesson 18: Parse code chunk labels

Date: 2026-06-09

### What I learned

- Parsed Quarto/R Markdown code chunk labels from fenced code block headers.
- Supported labels written as `{r fig-model}`.
- Supported Quarto chunk option labels written as `#| label: fig-model`.
- Used regular expressions to capture label text.
- Used `Option` chaining with `?` and `map`.
- Added code chunk labels to the global label index.

### Notes

- Code chunk labels are also label definitions.
- `CodeBlock.label` stores the label for the code block itself.
- `QmdDocument.labels` stores all labels used by later cross-reference diagnostics, completion, and go-to-definition.
- Header labels should not be overwritten by `#| label:` inside the same code block.
- Current character positions are approximate byte offsets and will need UTF-16 handling later for LSP.

## Lesson 20: Move heading parsing into Heading methods

Date: 2026-06-24

### What I learned

- Refactored `Heading` from a simple data holder into a QMD object.
- Replaced separate `line` and `character` fields with `SourceRange`.
- Moved heading parsing logic into `Heading::parse`.
- Added `Heading::display_name` as a method.
- Reviewed the difference between associated functions and methods.

### Notes

- `Heading::parse(...)` is an associated function because no `Heading` value exists before parsing.
- `heading.display_name()` is a method because it uses `&self`.
- `Self` inside `impl Heading` refers to `Heading`.
- `SourceRange` is an internal range type, not an LSP range.
- Parser functions can delegate local parsing logic to domain objects.
