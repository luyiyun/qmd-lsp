mod document;
mod parser;

use parser::{parse_all_labels, parse_all_refs, parse_headings};

fn main() {
    // // 函数式
    // let nums = vec![1, 2, 3, 4];
    // let doubled: Vec<i32> = nums.iter().map(|x| *x * 2).collect();
    // // let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();  //
    // // 不推荐这个写法，特别是对String
    // println!("{:?}", doubled);
    //
    // let evens: Vec<&i32> = nums.iter().filter(|x| **x % 2 == 0).collect();
    // // let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    // println!("{:?}", evens);
    //
    // let texts = vec!["Hello", "1", "abs", "3"];
    // let nums: Vec<i32> = texts.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    // println!("{:?}", nums);
    //
    // return;

    let qmd = r#"
# Introduction

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
"#;

    let headings = parse_headings(qmd);
    let labels = parse_all_labels(qmd);

    println!("Headings:");
    for heading in headings {
        println!(
            "line {}: level {}, title = {}",
            heading.line, heading.level, heading.title
        );
    }

    println!("\nLabels:");

    for label in labels {
        println!(
            "line {}: {} {} {} ({})",
            label.line,
            label.character,
            label.kind.icon(),
            label.label,
            label.kind.display_name()
        );
    }

    let qmd = r#"
如 @fig-overview 所示，模型结构包括三个部分。

根据 @wang2024 的研究，结果具有稳定性。

详见 @tbl-baseline 和 @eq-model。

普通邮箱 test@example.com 不应该被识别。
"#;

    let refs = parse_all_refs(qmd);

    println!("\nReferences {}:", refs.len());
    for r in refs {
        println!("{:?}", r);
    }
}
