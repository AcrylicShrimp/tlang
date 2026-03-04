use crate::DiagnosticLevel;
use tl_span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticItem {
    pub level: DiagnosticLevel,
    pub span: Span,
    pub message: String,
    pub sub_items: Vec<DiagnosticSubItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticSubItem {
    pub level: DiagnosticLevel,
    pub span: Span,
    pub message: String,
}
