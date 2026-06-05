#[derive(Debug, Clone)]
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
    let mut headings = Vec::new();

    for (line_no, line) in text.lines().enumerate() {
        if let Some(heading) = parse_heading(line, line_no as u32) {
            headings.push(heading);
        }
    }
    headings
}

fn main() {
    let qmd = r#"# Title

This is plain text.

## Background

Some more text.

### Methods

#### Results

####### Too deep

#Invalid heading
"#;

    let headings = parse_headings(qmd);

    for heading in headings {
        println!("{:?}", heading);
    }

    let labels = [
        "fig-model",
        "tbl-baseline",
        "eq-loss",
        "sec-methods",
        "lst-code",
        "thm-main",
        "proof-main",
        "something-else",
    ];

    println!();
    println!("Labels:");

    for label in labels {
        let kind = LabelKind::from_label(label);

        if kind == LabelKind::Figure {
            println!("{label} is a figure label");
        } else {
            println!("{label} is {}", kind.as_str());
        }

        println!("{} {}: {}", kind.icon(), kind.display_name(), label);
    }
}
