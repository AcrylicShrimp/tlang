use crate::TokenKind;
use tl_span::Span;

#[derive(Debug, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
