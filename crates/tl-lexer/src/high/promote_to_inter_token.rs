use crate::{
    high::{
        inter_token::InterToken,
        inter_token_kind::{InterTokenComment, InterTokenKind},
    },
    low::{LowToken, LowTokenKind},
};
use tl_span::{BytePos, Source, Span};

pub fn promote_to_inter_token(source: &Source, offset: BytePos, token: LowToken) -> InterToken {
    let lo = source.span.lo + offset;
    let hi = lo + BytePos::new(token.len);
    let span = Span::new(lo, hi);

    let kind = match token.kind {
        LowTokenKind::Eof => InterTokenKind::Eof,
        LowTokenKind::Unknown => InterTokenKind::Unknown,
        LowTokenKind::Whitespace => InterTokenKind::Whitespace,
        LowTokenKind::Comment => InterTokenKind::Comment(InterTokenComment::Line {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
        }),
        LowTokenKind::DocComment => InterTokenKind::Comment(InterTokenComment::Doc {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
            is_terminated: true,
        }),
        LowTokenKind::DocCommentUnterminated => InterTokenKind::Comment(InterTokenComment::Doc {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
            is_terminated: false,
        }),
        LowTokenKind::Id => {
            let content = source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned();

            match content.as_str() {
                "true" | "false" => InterTokenKind::LitBool { content },
                "use" => InterTokenKind::KwUse,
                "as" => InterTokenKind::KwAs,
                "not" => InterTokenKind::LogicalNot,
                "and" => InterTokenKind::LogicalAnd,
                "or" => InterTokenKind::LogicalOr,
                _ => InterTokenKind::Id(content),
            }
        }
        LowTokenKind::LitInteger => InterTokenKind::LitInteger {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
            suffix: None,
        },
        LowTokenKind::LitFloat => InterTokenKind::LitFloat {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
            suffix: None,
        },
        LowTokenKind::LitString => InterTokenKind::LitString {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
            is_terminated: true,
        },
        LowTokenKind::LitStringUnterminated => InterTokenKind::LitString {
            content: source
                .slice_content(span)
                .expect("offset is out of range")
                .to_owned(),
            is_terminated: false,
        },
        LowTokenKind::Dot => InterTokenKind::Dot,
        LowTokenKind::Comma => InterTokenKind::Comma,
        LowTokenKind::Semicolon => InterTokenKind::Semicolon,
        LowTokenKind::Colon => InterTokenKind::Colon,
        LowTokenKind::ParenOpen => InterTokenKind::ParenOpen,
        LowTokenKind::ParenClose => InterTokenKind::ParenClose,
        LowTokenKind::BraceOpen => InterTokenKind::BraceOpen,
        LowTokenKind::BraceClose => InterTokenKind::BraceClose,
        LowTokenKind::BracketOpen => InterTokenKind::BracketOpen,
        LowTokenKind::BracketClose => InterTokenKind::BracketClose,
        LowTokenKind::At => InterTokenKind::At,
        LowTokenKind::Bang => InterTokenKind::Bang,
        LowTokenKind::Assign => InterTokenKind::Assign,
        LowTokenKind::Add => InterTokenKind::Add,
        LowTokenKind::Sub => InterTokenKind::Sub,
        LowTokenKind::Mul => InterTokenKind::Mul,
        LowTokenKind::Div => InterTokenKind::Div,
        LowTokenKind::Mod => InterTokenKind::Mod,
        LowTokenKind::BitwiseNot => InterTokenKind::BitwiseNot,
        LowTokenKind::BitwiseXor => InterTokenKind::BitwiseXor,
        LowTokenKind::BitwiseAnd => InterTokenKind::BitwiseAnd,
        LowTokenKind::BitwiseOr => InterTokenKind::BitwiseOr,
        LowTokenKind::Lt => InterTokenKind::Lt,
        LowTokenKind::Gt => InterTokenKind::Gt,
    };

    InterToken { kind, span }
}
