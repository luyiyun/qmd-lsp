#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: u32,
    pub character: u32,
}

impl SourcePosition {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceRange {
    pub fn new(start_line: u32, start_character: u32, end_line: u32, end_character: u32) -> Self {
        Self {
            start: SourcePosition::new(start_line, start_character),
            end: SourcePosition::new(end_line, end_character),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_source_position() {
        let pos = SourcePosition::new(3, 5);
        assert_eq!(pos.line, 3);
        assert_eq!(pos.character, 5);
    }

    #[test]
    fn create_source_range() {
        let range = SourceRange::new(3, 5, 6, 7);
        assert_eq!(range.start.line, 3);
        assert_eq!(range.start.character, 5);
        assert_eq!(range.end.line, 6);
        assert_eq!(range.end.character, 7);
    }
}
