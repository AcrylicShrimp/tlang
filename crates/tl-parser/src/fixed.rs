use crate::cursor::TokenType;
use tl_span::{BytePos, Span};

pub trait FixedToken {
    const TOKEN_TYPE: TokenType;

    fn new(lo: BytePos) -> Self;
}

macro_rules! define_fixed {
    ($name:ident, $len:expr, $token_type:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            pub lo: BytePos,
        }

        impl $name {
            pub const LEN: BytePos = BytePos::new($len);
        }

        impl $name {
            pub const fn new(lo: BytePos) -> Self {
                Self { lo }
            }

            pub fn span(self) -> Span {
                Span::new(self.lo, self.lo + Self::LEN)
            }
        }

        impl FixedToken for $name {
            const TOKEN_TYPE: TokenType = $token_type;

            fn new(lo: BytePos) -> Self {
                Self { lo }
            }
        }
    };
}

pub mod keywords {
    use super::*;

    define_fixed!(KwUse, 3, TokenType::KwUse);
    define_fixed!(KwAs, 2, TokenType::KwAs);
}

pub mod punctuations {
    use super::*;

    define_fixed!(PuncDot, 1, TokenType::Dot);
    define_fixed!(PuncComma, 1, TokenType::Comma);
    define_fixed!(PuncSemicolon, 1, TokenType::Semicolon);
    define_fixed!(PuncColon, 1, TokenType::Colon);
    define_fixed!(PuncParenOpen, 1, TokenType::ParenOpen);
    define_fixed!(PuncParenClose, 1, TokenType::ParenClose);
    define_fixed!(PuncBraceOpen, 1, TokenType::BraceOpen);
    define_fixed!(PuncBraceClose, 1, TokenType::BraceClose);
    define_fixed!(PuncBracketOpen, 1, TokenType::BracketOpen);
    define_fixed!(PuncBracketClose, 1, TokenType::BracketClose);
    define_fixed!(PuncAt, 1, TokenType::At);
    define_fixed!(PuncArrow, 2, TokenType::Arrow);
}

pub mod operators {
    use crate::cursor::TokenType;

    use super::*;

    define_fixed!(OpAssign, 1, TokenType::Assign);
    define_fixed!(OpMul, 1, TokenType::Mul);
}
