use crate::{
    Token, TokenComment, TokenKind,
    high::{
        inter_token::InterToken,
        inter_token_kind::{InterTokenComment, InterTokenKind},
    },
};

pub fn promote_to_token(token: InterToken) -> Token {
    use InterTokenKind as From;
    use TokenKind as To;

    let kind = match token.kind {
        From::Eof => To::Eof,
        From::Unknown => To::Unknown,
        From::Whitespace => To::Whitespace,
        From::Comment(comment) => match comment {
            InterTokenComment::Line { content } => To::Comment(TokenComment::Line { content }),
            InterTokenComment::Doc {
                content,
                is_terminated,
            } => To::Comment(TokenComment::Doc {
                content,
                is_terminated,
            }),
        },
        From::Id(id) => To::Id(id),
        From::KwUse => To::KwUse,
        From::KwAs => To::KwAs,
        From::LitBool { content } => To::LitBool { content },
        From::LitInteger { content, suffix } => To::LitInteger { content, suffix },
        From::LitFloat { content, suffix } => To::LitFloat { content, suffix },
        From::LitString {
            content,
            is_terminated,
        } => To::LitString {
            content,
            is_terminated,
        },
        From::Dot => To::Dot,
        From::Comma => To::Comma,
        From::Semicolon => To::Semicolon,
        From::Colon => To::Colon,
        From::ParenOpen => To::ParenOpen,
        From::ParenClose => To::ParenClose,
        From::BraceOpen => To::BraceOpen,
        From::BraceClose => To::BraceClose,
        From::BracketOpen => To::BracketOpen,
        From::BracketClose => To::BracketClose,
        From::At => To::At,
        From::Bang => To::Unknown,
        From::Arrow => To::Arrow,
        From::PathSep => To::PathSep,
        From::Assign => To::Assign,
        From::AddAssign => To::AddAssign,
        From::SubAssign => To::SubAssign,
        From::MulAssign => To::MulAssign,
        From::DivAssign => To::DivAssign,
        From::ModAssign => To::ModAssign,
        From::PowAssign => To::PowAssign,
        From::BitwiseNotAssign => To::BitwiseNotAssign,
        From::BitwiseXorAssign => To::BitwiseXorAssign,
        From::BitwiseAndAssign => To::BitwiseAndAssign,
        From::BitwiseOrAssign => To::BitwiseOrAssign,
        From::BitwiseShiftLeftAssign => To::BitwiseShiftLeftAssign,
        From::BitwiseShiftRightAssign => To::BitwiseShiftRightAssign,
        From::Add => To::Add,
        From::Sub => To::Sub,
        From::Mul => To::Mul,
        From::Div => To::Div,
        From::Mod => To::Mod,
        From::Pow => To::Pow,
        From::BitwiseNot => To::BitwiseNot,
        From::BitwiseXor => To::BitwiseXor,
        From::BitwiseAnd => To::BitwiseAnd,
        From::BitwiseOr => To::BitwiseOr,
        From::BitwiseShiftLeft => To::BitwiseShiftLeft,
        From::BitwiseShiftRight => To::BitwiseShiftRight,
        From::Eq => To::Eq,
        From::Neq => To::Neq,
        From::Lt => To::Lt,
        From::Le => To::Le,
        From::Gt => To::Gt,
        From::Ge => To::Ge,
        From::LogicalNot => To::LogicalNot,
        From::LogicalAnd => To::LogicalAnd,
        From::LogicalOr => To::LogicalOr,
    };

    Token {
        kind,
        span: token.span,
    }
}
