use crate::high::inter_token_kind::InterTokenKind;
use tl_span::Span;

pub struct InterToken {
    pub kind: InterTokenKind,
    pub span: Span,
}
