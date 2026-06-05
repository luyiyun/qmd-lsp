fn print_lines(text: &str) {
    for line in text.lines() {
        println!("{}", line);
    }
}

fn main() {
    let qmd = r#"
---
title: "My QMD Note"
format: html
---

# Introduction

This is a Quarto document.

## Methods

```{r}
summary(cars)
```

如 @fig-model 所示。
"#;
    print_lines(qmd);
    // let qmd = String::from("# Title\n\n## Background\n\nSome text");
    // print_lines(&qmd);
}
