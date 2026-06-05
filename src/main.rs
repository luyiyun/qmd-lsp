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

fn main() {
    let lines = [
        "# Title",
        "## Background",
        "### Methods",
        "#### Results",
        "##### Discussion",
        "###### Conclusion",
        "####### Too deep",
        "Plain text",
        "",
        "####",
        "# ",
        "#Title",
        "#\tTabbed title",
    ];
    for (line_no, line) in lines.iter().enumerate() {
        match parse_heading(line, line_no as u32) {
            Some(heading) => {
                println!("{:?}", heading);
            }
            None => {
                println!("Not a heading: {:?}", line);
            }
        }
    }
}
