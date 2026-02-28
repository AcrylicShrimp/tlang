use crate::{
    Token,
    high::{
        glue::glue, inter_token::InterToken, promote_to_inter_token::promote_to_inter_token,
        promote_to_token::promote_to_token,
    },
    low::low_token_iter,
};
use tl_span::{BytePos, Source, Span};

pub fn token_iter(source: &Source) -> impl Iterator<Item = Token> {
    let mut iter = unglued_token_iter(source).peekable();

    std::iter::from_fn(move || {
        let mut token = iter.next()?;

        while let Some(next) = iter.peek() {
            if let Some(glue) = glue(&token.kind, &next.kind) {
                token = InterToken {
                    kind: glue,
                    span: Span::union(token.span, next.span),
                };
                iter.next();
            } else {
                break;
            }
        }

        Some(promote_to_token(token))
    })
}

fn unglued_token_iter(source: &Source) -> impl Iterator<Item = InterToken> {
    let mut offset = 0;
    let mut iter = low_token_iter(&source.content);

    std::iter::from_fn(move || {
        let token = iter.next()?;
        let len = token.len;
        let token = promote_to_inter_token(source, BytePos::new(offset), token);

        offset += len;

        Some(token)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TokenComment, TokenKind};

    fn test_tokens(input: &str, expected: &[TokenKind]) {
        let source = Source::new("test".to_owned(), BytePos::new(0), input.to_owned());
        let tokens = token_iter(&source)
            .map(|token| token.kind)
            .collect::<Vec<_>>();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokens_empty() {
        test_tokens("", &[TokenKind::Eof]);
    }

    #[test]
    fn test_tokens_whitespace() {
        test_tokens(" ", &[TokenKind::Whitespace, TokenKind::Eof]);
    }

    #[test]
    fn test_tokens_glue() {
        test_tokens(
            "<< >> == != <= >= <<= >>=",
            &[
                TokenKind::BitwiseShiftLeft,
                TokenKind::Whitespace,
                TokenKind::BitwiseShiftRight,
                TokenKind::Whitespace,
                TokenKind::Eq,
                TokenKind::Whitespace,
                TokenKind::Neq,
                TokenKind::Whitespace,
                TokenKind::Le,
                TokenKind::Whitespace,
                TokenKind::Ge,
                TokenKind::Whitespace,
                TokenKind::BitwiseShiftLeftAssign,
                TokenKind::Whitespace,
                TokenKind::BitwiseShiftRightAssign,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_tokens_bang() {
        test_tokens(
            "! !=",
            &[
                TokenKind::Unknown,
                TokenKind::Whitespace,
                TokenKind::Neq,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_tokens_doc_comment() {
        test_tokens(
            "#/ hello, world /# #/ hi",
            &[
                TokenKind::Comment(TokenComment::Doc {
                    content: "#/ hello, world /#".to_owned(),
                    is_terminated: true,
                }),
                TokenKind::Whitespace,
                TokenKind::Comment(TokenComment::Doc {
                    content: "#/ hi".to_owned(),
                    is_terminated: false,
                }),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_tokens_float_literal() {
        test_tokens(
            "1.0 2e+3 4.f32 5e-2f64 6.02e23 7.e-1f32",
            &[
                TokenKind::LitFloat {
                    content: "1.0".to_owned(),
                    suffix: None,
                },
                TokenKind::Whitespace,
                TokenKind::LitFloat {
                    content: "2e+3".to_owned(),
                    suffix: None,
                },
                TokenKind::Whitespace,
                TokenKind::LitFloat {
                    content: "4.".to_owned(),
                    suffix: Some("f32".to_owned()),
                },
                TokenKind::Whitespace,
                TokenKind::LitFloat {
                    content: "5e-2".to_owned(),
                    suffix: Some("f64".to_owned()),
                },
                TokenKind::Whitespace,
                TokenKind::LitFloat {
                    content: "6.02e23".to_owned(),
                    suffix: None,
                },
                TokenKind::Whitespace,
                TokenKind::LitFloat {
                    content: "7.e-1".to_owned(),
                    suffix: Some("f32".to_owned()),
                },
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_tokens_string_literal() {
        test_tokens(
            "\"hi\" \"a\\\"b\" \"unterminated",
            &[
                TokenKind::LitString {
                    content: "\"hi\"".to_owned(),
                    is_terminated: true,
                },
                TokenKind::Whitespace,
                TokenKind::LitString {
                    content: "\"a\\\"b\"".to_owned(),
                    is_terminated: true,
                },
                TokenKind::Whitespace,
                TokenKind::LitString {
                    content: "\"unterminated".to_owned(),
                    is_terminated: false,
                },
                TokenKind::Eof,
            ],
        );
    }
}
