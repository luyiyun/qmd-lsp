#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QmdElementKind {
    Heading,
    CodeBlock,
    LabelDef,
    CrossRefUse,
    CitationUse,
    Paragraph,
    Unknown,
}
