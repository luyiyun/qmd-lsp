use crate::range::SourceRange;

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

pub trait QmdNode {
    fn kind(&self) -> QmdElementKind;
    fn range(&self) -> SourceRange;
    fn display_name(&self) -> String;
}
