use crate::{
    TokenKind,
    low::{Cursor, LowToken},
};
use unicode_xid::UnicodeXID;

pub fn low_token_iter(mut s: &str) -> impl Iterator<Item = LowToken> {
    let mut eof_reached = false;

    std::iter::from_fn(move || {
        if eof_reached {
            return None;
        }

        let token = next(s);
        s = &s[token.len as usize..];
        eof_reached = token.kind == TokenKind::Eof;

        Some(token)
    })
}

fn next(s: impl AsRef<str>) -> LowToken {
    let s = s.as_ref();
    let mut cursor = Cursor::new(s);

    let c = match cursor.consume() {
        Some(c) => c,
        None => {
            return LowToken {
                kind: TokenKind::Eof,
                len: 0,
            };
        }
    };

    let kind = match c {
        c if is_whitespace_start(c) => {
            cursor.consume_while(is_whitespace_continue);
            TokenKind::Whitespace
        }

        c if is_doc_comment_start(c, &cursor) => {
            cursor.consume_while(is_doc_comment_continue);
            // consume trailing `/#`
            cursor.consume();
            cursor.consume();
            TokenKind::DocComment
        }

        c if is_comment_start(c) => {
            cursor.consume_while(is_comment_continue);
            TokenKind::Comment
        }

        c if is_id_start(c) => {
            cursor.consume_while(is_id_continue);
            TokenKind::Id
        }

        c if is_integer_start(c) => {
            cursor.consume_while(is_integer_continue);
            TokenKind::LitInteger
        }

        '.' => TokenKind::Dot,
        ',' => TokenKind::Comma,
        ';' => TokenKind::Semicolon,
        ':' => TokenKind::Colon,
        '(' => TokenKind::ParenOpen,
        ')' => TokenKind::ParenClose,
        '{' => TokenKind::BraceOpen,
        '}' => TokenKind::BraceClose,
        '[' => TokenKind::BracketOpen,
        ']' => TokenKind::BracketClose,
        '@' => TokenKind::At,
        '!' => TokenKind::Bang,

        '=' => TokenKind::Assign,

        '+' => TokenKind::Add,
        '-' => TokenKind::Sub,
        '*' => TokenKind::Mul,
        '/' => TokenKind::Div,
        '%' => TokenKind::Mod,

        '~' => TokenKind::BitwiseNot,
        '^' => TokenKind::BitwiseXor,
        '&' => TokenKind::BitwiseAnd,
        '|' => TokenKind::BitwiseOr,

        '<' => TokenKind::Lt,
        '>' => TokenKind::Gt,

        _ => TokenKind::Unknown,
    };

    let consumed = cursor.len_consumed();

    #[cfg(debug_assertions)]
    if (u32::MAX as usize) < consumed {
        panic!("token length is too long");
    }

    LowToken {
        kind,
        len: consumed as u32,
    }
}

fn is_whitespace_start(c: char) -> bool {
    c.is_whitespace()
}

fn is_whitespace_continue(cursor: &Cursor) -> bool {
    cursor.at(0).is_whitespace()
}

fn is_id_start(c: char) -> bool {
    c == '_' || UnicodeXID::is_xid_start(c)
}

fn is_id_continue(cursor: &Cursor) -> bool {
    UnicodeXID::is_xid_continue(cursor.at(0))
}

fn is_doc_comment_start(c: char, cursor: &Cursor) -> bool {
    c == '#' && cursor.at(0) == '/'
}

fn is_doc_comment_continue(cursor: &Cursor) -> bool {
    !cursor.is_empty() && (cursor.at(0) != '/' || cursor.at(1) != '#')
}

fn is_comment_start(c: char) -> bool {
    c == '#'
}

fn is_comment_continue(cursor: &Cursor) -> bool {
    !cursor.is_empty() && cursor.at(0) != '\n'
}

fn is_integer_start(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_integer_continue(cursor: &Cursor) -> bool {
    cursor.at(0).is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_tokens(s: &str, expected: &[LowToken]) {
        let tokens = low_token_iter(s).collect::<Vec<_>>();

        assert_eq!(tokens.len(), expected.len());

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token, &expected[i]);
        }
    }

    fn match_kinds(s: &str, expected: &[TokenKind]) {
        let kinds = low_token_iter(s)
            .map(|token| token.kind)
            .collect::<Vec<_>>();

        assert_eq!(kinds.len(), expected.len());

        for (i, kind) in kinds.iter().enumerate() {
            assert_eq!(kind, &expected[i]);
        }
    }

    #[test]
    fn test_empty_input() {
        match_tokens(
            "",
            &[LowToken {
                kind: TokenKind::Eof,
                len: 0,
            }],
        );
    }

    #[test]
    fn test_len_sum_is_equal_to_input_len() {
        let s = "Hello, world! This is a test. 1 + 1 = 2!";
        let tokens = low_token_iter(s).collect::<Vec<_>>();
        let len_sum = tokens.iter().map(|token| token.len).sum::<u32>();
        assert_eq!(len_sum, s.len() as u32);
    }
}
