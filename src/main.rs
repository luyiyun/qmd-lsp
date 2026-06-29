// use document::QmdDocument;
// use element::QmdElementKind;
// use range::{SourcePosition, SourceRange};
use qmd_lsp::nodes::Heading;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/Users/rong/Project/notes/src/learn/prml/10-approximate-inference.qmd";
    let content = fs::read_to_string(file_path)?;

    let qmd = r#"# Introduction

     Some text.

     ![Overview figure](overview.png){#fig-overview}

     See @fig-overview.

     | Variable | Value |
     |----------|-------|
     | age      | 60    |

     : Baseline table {#tbl-baseline}

     ## Methods {#sec-methods}

     $$
     y = ax + b
     $$ {#eq-model}

     如 @fig-overview 所示，模型结构包括三个部分。

     根据 @wang2024 的研究，结果具有稳定性。

     详见 @tbl-baseline 和 @eq-model。

     普通邮箱 test@example.com 不应该被识别。

     ```{r}
     summary(cars)

     print("hello")
     ```

     "#;

    let heading = Heading::parse(qmd, 0);

    println!("Heading: {:?}", heading);
    // println!("Content: {}", content);

    Ok(())

    // let pos = SourcePosition::new(1, 1);
    // let range = SourceRange::new(0, 0, 0, 12);
    // let kind = QmdElementKind::Heading;
    // println!("Position: {:?}", pos);
    // println!("Range: {:?}", range);
    // println!("Kind: {:?}", kind);
    //
    //     let doc = QmdDocument::parse(qmd);
    //
    //     println!("Headings:");
    //     for heading in &doc.headings {
    //         println!(
    //             "line {}: level {}, title = {}",
    //             heading.line, heading.level, heading.title
    //         );
    //     }
    //
    //     println!("\nLabels:");
    //     for label in &doc.labels {
    //         println!(
    //             "line {}: {} {} {} ({})",
    //             label.line,
    //             label.character,
    //             label.kind.icon(),
    //             label.label,
    //             label.kind.display_name()
    //         );
    //     }
    //
    //     println!("\nReferences {}:", doc.refs.len());
    //     for r in &doc.refs {
    //         println!("{:?}", r);
    //     }
    //
    //     println!("\nCode Blocks:");
    //     println!("{:?}", doc.code_blocks);
}
