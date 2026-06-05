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
}
