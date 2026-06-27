use crate::{
    element::{QmdElementKind, QmdNode},
    // parser::{parse_all_labels, parse_all_refs, parse_code_blocks, parse_headings},
    range::SourceRange,
};

// ========== Heading ==========

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub title: String,
    pub range: SourceRange,
}

impl Heading {
    pub fn parse_line(line: &str, line_no: u32) -> Option<Self> {
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

        let title = rest.trim().to_string();

        if title.is_empty() {
            return None;
        }

        let start_character = line.chars().take_while(|c| c.is_whitespace()).count() as u32;
        let end_character = line.chars().count() as u32;
        Some(Self {
            level: level as u8,
            title,
            range: SourceRange::new(line_no, start_character, line_no, end_character),
        })
    }

    pub fn parse_text(text: &str) -> Option<Vec<Self>> {
        let headings: Vec<Self> = text
            .lines()
            .enumerate()
            .filter_map(|(line_no, line)| Self::parse_line(line, line_no as u32))
            .collect();
        if headings.is_empty() {
            return None;
        }
        Some(headings)
    }
}

impl QmdNode for Heading {
    fn kind(&self) -> QmdElementKind {
        QmdElementKind::Heading
    }
    fn range(&self) -> SourceRange {
        self.range
    }
    fn display_name(&self) -> String {
        format!("{} {}", "#".repeat(self.level as usize), self.title)
    }
}

// ========== Label ==========

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelKind {
    Figure,
    Table,
    Equation,
    Section,
    Listing,
    Theorem,
    Proof,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LabelDef {
    pub label: String,
    pub kind: LabelKind,
    pub line: u32,
    pub character: u32,
}

impl LabelKind {
    pub fn from_label(label: &str) -> Self {
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

    pub fn as_str(&self) -> &'static str {
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

    pub fn display_name(&self) -> &'static str {
        self.as_str()
    }

    pub fn icon(&self) -> &'static str {
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

// ========== Reference ==========
#[derive(Debug, Clone)]
pub enum RefKind {
    Figure,
    Table,
    Equation,
    Section,
    Citation,
}

impl RefKind {
    pub fn from_ref(ref_str: &str) -> Self {
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
pub struct RefUse {
    pub key: String,
    pub kind: RefKind,
    pub line: u32,
    pub character: u32,
}

// ========== Code Block ==========

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub label: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
}

// ========== Document ==========

// #[derive(Debug, Clone)]
// pub struct QmdDocument {
//     pub text: String,
//     pub headings: Vec<Heading>,
//     pub labels: Vec<LabelDef>,
//     pub refs: Vec<RefUse>,
//     pub code_blocks: Vec<CodeBlock>,
// }
//
// impl QmdDocument {
//     pub fn parse(text: &str) -> Self {
//         Self::from_string(text.to_string())
//     }
//
//     pub fn from_string(text: String) -> Self {
//         let headings = parse_headings(&text);
//         let labels = parse_all_labels(&text);
//         let refs = parse_all_refs(&text);
//         let code_blocks = parse_code_blocks(&text);
//
//         Self {
//             text,
//             headings,
//             labels,
//             refs,
//             code_blocks,
//         }
//     }
// }

// ========== test ==========
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_heading() {
        let heading = Heading::parse_line("# Introduction", 0).unwrap();

        assert_eq!(heading.level, 1);
        assert_eq!(heading.title, "Introduction");
        assert_eq!(heading.range.start.line, 0);
        assert_eq!(heading.range.start.character, 0);
        assert_eq!(heading.display_name(), "# Introduction");
    }

    #[test]
    fn parse_indented_heading() {
        let heading = Heading::parse_line("  ## Background", 3).unwrap();

        assert_eq!(heading.level, 2);
        assert_eq!(heading.title, "Background");
        assert_eq!(heading.range.start.line, 3);
        assert_eq!(heading.range.start.character, 2);
        assert_eq!(heading.display_name(), "## Background");
    }

    #[test]
    fn ignore_plain_text() {
        let heading = Heading::parse_line("This is not a heading", 0);

        assert!(heading.is_none());
    }

    #[test]
    fn ignore_heading_without_space() {
        let heading = Heading::parse_line("#Invalid", 0);

        assert!(heading.is_none());
    }

    #[test]
    fn ignore_too_deep_heading() {
        let heading = Heading::parse_line("####### Too deep", 0);

        assert!(heading.is_none());
    }

    #[test]
    fn parse_multiple_headings() {
        let text = r#"
        # Title
        ## Methods
        "#;
        let headings = Heading::parse_text(text).unwrap();
        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].title, "Title");
        assert_eq!(headings[1].title, "Methods");
    }

    #[test]
    fn parse_zero_headings() {
        let text = r#"
        sfsfsf
        "#;
        let headings = Heading::parse_text(text);
        assert!(headings.is_none());
    }

    //     #[test]
    //     fn parse_qmd_document() {
    //         let text = r#"
    // # Title
    //
    // ## Methods
    //
    // ![Model](model.png){#fig-model}
    // "#;
    //
    //         let doc = QmdDocument::parse(text);
    //
    //         assert_eq!(doc.headings.len(), 2);
    //         assert_eq!(doc.headings[0].title, "Title");
    //         assert_eq!(doc.headings[1].title, "Methods");
    //
    //         assert_eq!(doc.labels.len(), 1);
    //         assert_eq!(doc.labels[0].label, "fig-model");
    //
    //         assert_eq!(doc.text, text);
    //     }
    //

    #[test]
    fn heading_implements_qmd_node() {
        let heading = Heading::parse_line("## Methods", 7).unwrap();
        assert_eq!(heading.kind(), QmdElementKind::Heading);
        assert_eq!(heading.range(), SourceRange::new(7, 0, 7, 10));
        assert_eq!(heading.display_name(), "## Methods");
    }
}
