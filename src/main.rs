use regex::Regex;

// ========== Heading ==========

#[derive(Debug, Clone, PartialEq)]
struct Heading {
    level: u8,
    title: String,
    line: u32,
    character: u32,
}

impl Heading {
    fn new(level: u8, title: String, line: u32, character: u32) -> Self {
        Self {
            level,
            title,
            line,
            character,
        }
    }
}

fn parse_heading(line: &str, line_no: u32) -> Option<Heading> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }

    let level = trimmed.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }

    let rest = &trimmed[level..];

    if !rest.starts_with(' ') {
        return None;
    }
    // if !rest.chars().next().is_some_and(|c| c.is_whitespace()) {
    //     return None;
    // }

    let title = rest.trim();

    if title.is_empty() {
        return None;
    }
    Some(Heading::new(level as u8, title.to_string(), line_no, 0))
}

fn parse_headings(text: &str) -> Vec<Heading> {
    // let mut headings = Vec::new();
    // for (line_no, line) in text.lines().enumerate() {
    //     if let Some(heading) = parse_heading(line, line_no as u32) {
    //         headings.push(heading);
    //     }
    // }
    // headings

    text.lines()
        .enumerate()
        .filter_map(|(line_no, line)| parse_heading(line, line_no as u32))
        .collect()
}

// ========== Label ==========

#[derive(Debug, Clone, PartialEq, Eq)]
enum LabelKind {
    Figure,
    Table,
    Equation,
    Section,
    Listing,
    Theorem,
    Proof,
    Unknown,
}

impl LabelKind {
    fn from_label(label: &str) -> Self {
        if label.starts_with("fig-") {
            Self::Figure
        } else if label.starts_with("tbl-") {
            Self::Table
        } else if label.starts_with("eq-") {
            Self::Equation
        } else if label.starts_with("sec-") {
            Self::Section
        } else if label.starts_with("lst-") {
            Self::Listing
        } else if label.starts_with("thm-") {
            Self::Theorem
        } else if label.starts_with("prp-") || label.starts_with("proof-") {
            Self::Proof
        } else {
            Self::Unknown
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Figure => "Figure",
            Self::Table => "Table",
            Self::Equation => "Equation",
            Self::Section => "Section",
            Self::Listing => "Listing",
            Self::Theorem => "Theorem",
            Self::Proof => "Proof",
            Self::Unknown => "Unknown",
        }
    }

    fn display_name(&self) -> &'static str {
        self.as_str()
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Figure => "🖼️",
            Self::Table => "📊",
            Self::Equation => "∑",
            Self::Section => "§",
            Self::Listing => "💻",
            Self::Theorem => "📐",
            Self::Proof => "✅",
            Self::Unknown => "?",
        }
    }
}

#[derive(Debug, Clone)]
struct LabelDef {
    label: String,
    kind: LabelKind,
    line: u32,
    character: u32,
}

fn parse_labels(line: &str, line_no: u32) -> Vec<LabelDef> {
    let re = Regex::new(r"\{#([A-Za-z0-9_\-]+)\}").unwrap();

    let mut labels = Vec::new();
    for cap in re.captures_iter(line) {
        let m = cap.get(1).unwrap();
        let label = m.as_str();
        labels.push(LabelDef {
            kind: LabelKind::from_label(label),
            label: label.to_string(),
            line: line_no,
            character: m.start() as u32,
        })
        // let label = m.as_str().to_string();
        // labels.push(LabelDef {
        //     kind: LabelKind::from_label(&label),
        //     label,
        //     line: line_no,
        //     character: m.start() as u32,
        // })
    }

    labels
}

fn parse_all_labels(text: &str) -> Vec<LabelDef> {
    let mut labels = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let line_labels = parse_labels(line, line_no as u32);

        for label in line_labels {
            labels.push(label);
        }
    }
    labels
}

// ========== Reference ==========
#[derive(Debug, Clone)]
enum RefKind {
    Figure,
    Table,
    Equation,
    Section,
    Citation,
}

impl RefKind {
    fn from_ref(ref_str: &str) -> Self {
        match ref_str {
            k if k.starts_with("fig-") => Self::Figure,
            k if k.starts_with("tbl-") => Self::Table,
            k if k.starts_with("eq-") => Self::Equation,
            k if k.starts_with("sec-") => Self::Figure,
            _ => Self::Citation,
        }
    }
}

#[derive(Debug, Clone)]
struct RefUse {
    key: String,
    kind: RefKind,
    line: u32,
    character: u32,
}

fn parse_refs(line: &str, line_no: u32) -> Vec<RefUse> {
    let re = Regex::new(r" @([A-Za-z0-9_\-]+) ").unwrap();
    let mut refs = Vec::new();
    for cap in re.captures_iter(line) {
        let m = cap.get(1).unwrap();
        let key = m.as_str();
        refs.push(RefUse {
            kind: RefKind::from_ref(key),
            key: key.to_string(),
            line: line_no,
            character: m.start() as u32,
        })
    }
    refs
}

fn parse_all_refs(text: &str) -> Vec<RefUse> {
    let mut refs = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let line_refs = parse_refs(line, line_no as u32);
        for ref_use in line_refs {
            refs.push(ref_use);
        }
    }
    refs
}

fn main() {
    // 函数式
    let nums = vec![1, 2, 3, 4];
    let doubled: Vec<i32> = nums.iter().map(|x| *x * 2).collect();
    // let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();  //
    // 不推荐这个写法，特别是对String
    println!("{:?}", doubled);

    let evens: Vec<&i32> = nums.iter().filter(|x| **x % 2 == 0).collect();
    // let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", evens);

    let texts = vec!["Hello", "1", "abs", "3"];
    let nums: Vec<i32> = texts.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("{:?}", nums);

    return;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_headings() {
        let heading = parse_heading("# Introduction", 0).unwrap();
        assert_eq!(heading.level, 1);
        assert_eq!(heading.title, "Introduction");
        assert_eq!(heading.line, 0);
        assert_eq!(heading.character, 0);
    }

    #[test]
    fn parse_level_two_heading() {
        let heading = parse_heading("## Background", 3).unwrap();
        assert_eq!(heading.level, 2);
        assert_eq!(heading.title, "Background");
        assert_eq!(heading.line, 3);
    }

    #[test]
    fn ignore_plain_text() {
        let heading = parse_heading("This is not a heading", 0);
        assert!(heading.is_none());
    }

    #[test]
    fn ignore_heading_without_space() {
        let heading = parse_heading("#Introduction", 0);

        assert!(heading.is_none());
        // assert_eq!(heading, None);   // 也可以这样写，需要加 derive(PartialEq)
    }

    #[test]
    fn ignore_too_deep_heading() {
        let heading = parse_heading("####### Too deep", 0);

        assert!(heading.is_none());
    }

    #[test]
    fn parse_basic_label() {
        let labels = parse_labels("![caption](fig.png){#fig-overview}", 10);

        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].label, "fig-overview");
        assert_eq!(labels[0].kind, LabelKind::Figure);
        assert_eq!(labels[0].line, 10);
    }

    #[test]
    fn parse_multiple_labels_in_one_line() {
        let labels = parse_labels("{#fig-a} and {#tbl-b}", 5);

        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].label, "fig-a");
        assert_eq!(labels[1].label, "tbl-b");
        assert_eq!(labels[0].kind, LabelKind::Figure);
        assert_eq!(labels[1].kind, LabelKind::Table);
    }
}
