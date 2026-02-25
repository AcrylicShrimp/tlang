use crate::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LowToken {
    pub kind: TokenKind,
    pub len: u32,
}
