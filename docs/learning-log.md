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
