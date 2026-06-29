use crate::element::{QmdElement, QmdElementKind};
use crate::range::SourceRange;

const MAX_HEADING_LEVEL: usize = 6;
const MIN_HEADING_LEVEL: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub range: SourceRange,
    pub level: u8,
    pub title: String,
}

impl QmdElement for Heading {
    fn kind(&self) -> QmdElementKind {
        QmdElementKind::Heading
    }

    fn range(&self) -> SourceRange {
        self.range
    }

    fn display_name(&self) -> String {
        format!("Heading({})", self.level)
    }
}

impl Heading {
    pub fn parse(text: &str, start_line: u32) -> Option<Self> {
        let first_line = text.lines().next()?.trim_start();
        if !first_line.starts_with('#') {
            return None;
        }
        let level = first_line.chars().filter(|c| *c == '#').count();
        if level == MIN_HEADING_LEVEL || level > MAX_HEADING_LEVEL {
            return None;
        }

        let rest = &first_line[level..];
        if !rest.starts_with(' ') {
            return None;
        }

        let title = rest.trim().to_string();
        // if title.is_empty() {
        //     return None;
        // }

        Some(Self {
            range: SourceRange::new(start_line, 0, start_line, first_line.len() as u32),
            level: level as u8,
            title,
        })
    }
}
