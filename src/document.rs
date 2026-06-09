use crate::parser::{parse_all_labels, parse_all_refs, parse_code_blocks, parse_headings};

// ========== Heading ==========

#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    pub level: u8,
    pub title: String,
    pub line: u32,
    pub character: u32,
}

impl Heading {
    pub fn new(level: u8, title: String, line: u32, character: u32) -> Self {
        Self {
            level,
            title,
            line,
            character,
        }
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

#[derive(Debug, Clone)]
pub struct QmdDocument {
    pub text: String,
    pub headings: Vec<Heading>,
    pub labels: Vec<LabelDef>,
    pub refs: Vec<RefUse>,
    pub code_blocks: Vec<CodeBlock>,
}

impl QmdDocument {
    pub fn parse(text: &str) -> Self {
        Self::from_string(text.to_string())
    }

    pub fn from_string(text: String) -> Self {
        let headings = parse_headings(&text);
        let labels = parse_all_labels(&text);
        let refs = parse_all_refs(&text);
        let code_blocks = parse_code_blocks(&text);

        Self {
            text,
            headings,
            labels,
            refs,
            code_blocks,
        }
    }
}

// ========== test ==========
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_qmd_document() {
        let text = r#"
# Title

## Methods

![Model](model.png){#fig-model}
"#;

        let doc = QmdDocument::parse(text);

        assert_eq!(doc.headings.len(), 2);
        assert_eq!(doc.headings[0].title, "Title");
        assert_eq!(doc.headings[1].title, "Methods");

        assert_eq!(doc.labels.len(), 1);
        assert_eq!(doc.labels[0].label, "fig-model");

        assert_eq!(doc.text, text);
    }
}
