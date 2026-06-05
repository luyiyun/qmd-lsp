fn parse_heading(line: &str) -> Option<(u8, String)> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }

    let level = trimmed.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }

    let rest = &trimmed[level..];

    if !rest.chars().next().is_some_and(|c| c.is_whitespace()) {
        return None;
    }

    let title = rest.trim();

    if title.is_empty() {
        return None;
    }
    Some((level as u8, title.to_string()))
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
    for line in lines {
        match parse_heading(line) {
            Some((level, title)) => {
                println!("Heading level {level}: {title}");
            }
            None => {
                println!("Not a heading: {line}");
            }
        }
    }
}
