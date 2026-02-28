use crate::low::{Cursor, LowToken, LowTokenKind};
use unicode_xid::UnicodeXID;

pub fn low_token_iter(mut s: &str) -> impl Iterator<Item = LowToken> {
    let mut eof_reached = false;

    std::iter::from_fn(move || {
        if eof_reached {
            return None;
        }

        let token = next(s);
        s = &s[token.len as usize..];
        eof_reached = token.kind == LowTokenKind::Eof;

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
                kind: LowTokenKind::Eof,
                len: 0,
            };
        }
    };

    let kind = match c {
        c if is_whitespace_start(c) => {
            cursor.consume_while(is_whitespace_continue);
            LowTokenKind::Whitespace
        }

        c if is_doc_comment_start(c, &cursor) => {
            cursor.consume_while(is_doc_comment_continue);

            if cursor.at(0) == '/' && cursor.at(1) == '#' {
                // consume trailing `/#`
                cursor.consume();
                cursor.consume();
                LowTokenKind::DocComment
            } else {
                LowTokenKind::DocCommentUnterminated
            }
        }

        c if is_comment_start(c) => {
            cursor.consume_while(is_comment_continue);
            LowTokenKind::Comment
        }

        c if is_id_start(c) => {
            cursor.consume_while(is_id_continue);
            LowTokenKind::Id
        }

        c if is_integer_start(c) => read_numeric(&mut cursor),

        '"' => read_string(&mut cursor),

        '.' => LowTokenKind::Dot,
        ',' => LowTokenKind::Comma,
        ';' => LowTokenKind::Semicolon,
        ':' => LowTokenKind::Colon,
        '(' => LowTokenKind::ParenOpen,
        ')' => LowTokenKind::ParenClose,
        '{' => LowTokenKind::BraceOpen,
        '}' => LowTokenKind::BraceClose,
        '[' => LowTokenKind::BracketOpen,
        ']' => LowTokenKind::BracketClose,
        '@' => LowTokenKind::At,
        '!' => LowTokenKind::Bang,

        '=' => LowTokenKind::Assign,

        '+' => LowTokenKind::Add,
        '-' => LowTokenKind::Sub,
        '*' => LowTokenKind::Mul,
        '/' => LowTokenKind::Div,
        '%' => LowTokenKind::Mod,

        '~' => LowTokenKind::BitwiseNot,
        '^' => LowTokenKind::BitwiseXor,
        '&' => LowTokenKind::BitwiseAnd,
        '|' => LowTokenKind::BitwiseOr,

        '<' => LowTokenKind::Lt,
        '>' => LowTokenKind::Gt,

        _ => LowTokenKind::Unknown,
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

fn read_numeric(cursor: &mut Cursor) -> LowTokenKind {
    cursor.consume_while(|cursor| cursor.at(0).is_ascii_digit());

    let mut has_dot = false;
    let mut has_exponent = false;

    if cursor.at(0) == '.' {
        has_dot = true;
        cursor.consume();
        cursor.consume_while(|cursor| cursor.at(0).is_ascii_digit());
    }

    if (cursor.at(0) == 'e' || cursor.at(0) == 'E')
        && (cursor.at(1) == '+' || cursor.at(1) == '-' || cursor.at(1).is_ascii_digit())
    {
        has_exponent = true;
        cursor.consume();
        cursor.consume();
        cursor.consume_while(|cursor| cursor.at(0).is_ascii_digit());
    }

    if has_dot || has_exponent {
        LowTokenKind::LitFloat
    } else {
        LowTokenKind::LitInteger
    }
}

fn read_string(cursor: &mut Cursor) -> LowTokenKind {
    while !cursor.is_empty() {
        match cursor.at(0) {
            '"' => {
                cursor.consume();
                return LowTokenKind::LitString;
            }
            '\\' => {
                cursor.consume();
                cursor.consume();
            }
            _ => {
                cursor.consume();
            }
        }
    }

    LowTokenKind::LitStringUnterminated
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(input: &str, expected: &[(LowTokenKind, u32)]) {
        let mut expected_tokens = expected
            .iter()
            .copied()
            .map(|(kind, len)| LowToken { kind, len })
            .collect::<Vec<_>>();
        expected_tokens.push(LowToken {
            kind: LowTokenKind::Eof,
            len: 0,
        });

        let tokens = low_token_iter(input).collect::<Vec<_>>();
        assert_eq!(tokens, expected_tokens, "input: {input:?}");
    }

    fn assert_kinds(input: &str, expected: &[LowTokenKind]) {
        let mut expected_kinds = expected.to_vec();
        expected_kinds.push(LowTokenKind::Eof);

        let kinds = low_token_iter(input)
            .map(|token| token.kind)
            .collect::<Vec<_>>();
        assert_eq!(kinds, expected_kinds, "input: {input:?}");
    }

    fn assert_len_sum_matches_input(input: &str) {
        let tokens = low_token_iter(input).collect::<Vec<_>>();
        let len_sum = tokens.iter().map(|token| token.len).sum::<u32>();
        assert_eq!(len_sum, input.len() as u32, "input: {input:?}");
        assert_eq!(
            tokens.last().map(|token| token.kind),
            Some(LowTokenKind::Eof)
        );
        assert_eq!(tokens.last().map(|token| token.len), Some(0));

        let mut index = 0usize;
        for token in tokens {
            let next = index + token.len as usize;
            assert!(next <= input.len(), "input: {input:?}");
            assert!(input.is_char_boundary(index), "input: {input:?}");
            assert!(input.is_char_boundary(next), "input: {input:?}");

            if token.kind != LowTokenKind::Eof {
                let _ = &input[index..next];
            }

            index = next;
        }

        assert_eq!(index, input.len(), "input: {input:?}");
    }

    #[test]
    fn test_empty_input() {
        assert_tokens("", &[]);
    }

    #[test]
    fn test_whitespace_tokens() {
        assert_tokens(" ", &[(LowTokenKind::Whitespace, 1)]);
        assert_tokens("\t\n", &[(LowTokenKind::Whitespace, 2)]);
        assert_tokens("\r\n \t", &[(LowTokenKind::Whitespace, 4)]);
        assert_tokens(
            "  abc",
            &[(LowTokenKind::Whitespace, 2), (LowTokenKind::Id, 3)],
        );
    }

    #[test]
    fn test_comment_tokens() {
        assert_tokens("#", &[(LowTokenKind::Comment, 1)]);
        assert_tokens(
            "# hello",
            &[(LowTokenKind::Comment, "# hello".len() as u32)],
        );
        assert_tokens(
            "# hello\n",
            &[
                (LowTokenKind::Comment, "# hello".len() as u32),
                (LowTokenKind::Whitespace, "\n".len() as u32),
            ],
        );
        assert_tokens(
            "# hello\r\nx",
            &[
                (LowTokenKind::Comment, "# hello\r".len() as u32),
                (LowTokenKind::Whitespace, "\n".len() as u32),
                (LowTokenKind::Id, 1),
            ],
        );
    }

    #[test]
    fn test_doc_comment_tokens() {
        assert_tokens("#//#", &[(LowTokenKind::DocComment, "#//#".len() as u32)]);
        assert_tokens(
            "#/hello/#",
            &[(LowTokenKind::DocComment, "#/hello/#".len() as u32)],
        );
        assert_tokens(
            "#/hello\nworld/#",
            &[(LowTokenKind::DocComment, "#/hello\nworld/#".len() as u32)],
        );
        assert_tokens(
            "#/a/#b",
            &[
                (LowTokenKind::DocComment, "#/a/#".len() as u32),
                (LowTokenKind::Id, 1),
            ],
        );
    }

    #[test]
    fn test_unterminated_doc_comment_tokens() {
        assert_tokens(
            "#/",
            &[(LowTokenKind::DocCommentUnterminated, "#/".len() as u32)],
        );
        assert_tokens(
            "#/abc",
            &[(LowTokenKind::DocCommentUnterminated, "#/abc".len() as u32)],
        );
        assert_tokens(
            "#/abc\n",
            &[(LowTokenKind::DocCommentUnterminated, "#/abc\n".len() as u32)],
        );
        assert_tokens(
            "#//",
            &[(LowTokenKind::DocCommentUnterminated, "#//".len() as u32)],
        );
    }

    #[test]
    fn test_id_tokens_ascii_and_unicode() {
        assert_tokens("abc", &[(LowTokenKind::Id, 3)]);
        assert_tokens("_abc123", &[(LowTokenKind::Id, 7)]);
        assert_tokens(
            "abc-1",
            &[
                (LowTokenKind::Id, 3),
                (LowTokenKind::Sub, 1),
                (LowTokenKind::LitInteger, 1),
            ],
        );

        assert!(UnicodeXID::is_xid_start('Δ'));
        assert_tokens("Δx", &[(LowTokenKind::Id, "Δx".len() as u32)]);

        assert_tokens(
            "x🙂",
            &[
                (LowTokenKind::Id, 1),
                (LowTokenKind::Unknown, "🙂".len() as u32),
            ],
        );
    }

    #[test]
    fn test_integer_tokens() {
        assert_tokens("0", &[(LowTokenKind::LitInteger, 1)]);
        assert_tokens("1234567890", &[(LowTokenKind::LitInteger, 10)]);
        assert_tokens(
            "42abc",
            &[(LowTokenKind::LitInteger, 2), (LowTokenKind::Id, 3)],
        );
        assert_tokens(
            "1_2",
            &[(LowTokenKind::LitInteger, 1), (LowTokenKind::Id, 2)],
        );
    }

    #[test]
    fn test_float_tokens() {
        assert_tokens("1.0", &[(LowTokenKind::LitFloat, "1.0".len() as u32)]);
        assert_tokens("1.", &[(LowTokenKind::LitFloat, "1.".len() as u32)]);
        assert_tokens("1e2", &[(LowTokenKind::LitFloat, "1e2".len() as u32)]);
        assert_tokens("42e-10", &[(LowTokenKind::LitFloat, "42e-10".len() as u32)]);
        assert_tokens(
            "6.02e23",
            &[(LowTokenKind::LitFloat, "6.02e23".len() as u32)],
        );
        assert_tokens("7.e-1", &[(LowTokenKind::LitFloat, "7.e-1".len() as u32)]);
        assert_tokens(
            "1e",
            &[(LowTokenKind::LitInteger, 1), (LowTokenKind::Id, 1)],
        );
        assert_tokens(
            "1..2",
            &[
                (LowTokenKind::LitFloat, "1.".len() as u32),
                (LowTokenKind::Dot, 1),
                (LowTokenKind::LitInteger, 1),
            ],
        );
    }

    #[test]
    fn test_string_tokens() {
        assert_tokens(
            "\"hello\"",
            &[(LowTokenKind::LitString, "\"hello\"".len() as u32)],
        );
        assert_tokens(
            "\"a\\\"b\"",
            &[(LowTokenKind::LitString, "\"a\\\"b\"".len() as u32)],
        );
        assert_tokens(
            "\"unterminated",
            &[(
                LowTokenKind::LitStringUnterminated,
                "\"unterminated".len() as u32,
            )],
        );
        assert_tokens(
            "\"hi\"id",
            &[
                (LowTokenKind::LitString, "\"hi\"".len() as u32),
                (LowTokenKind::Id, 2),
            ],
        );
    }

    #[test]
    fn test_punctuation_and_operator_tokens() {
        let cases = [
            (".", LowTokenKind::Dot),
            (",", LowTokenKind::Comma),
            (";", LowTokenKind::Semicolon),
            (":", LowTokenKind::Colon),
            ("(", LowTokenKind::ParenOpen),
            (")", LowTokenKind::ParenClose),
            ("{", LowTokenKind::BraceOpen),
            ("}", LowTokenKind::BraceClose),
            ("[", LowTokenKind::BracketOpen),
            ("]", LowTokenKind::BracketClose),
            ("@", LowTokenKind::At),
            ("!", LowTokenKind::Bang),
            ("=", LowTokenKind::Assign),
            ("+", LowTokenKind::Add),
            ("-", LowTokenKind::Sub),
            ("*", LowTokenKind::Mul),
            ("/", LowTokenKind::Div),
            ("%", LowTokenKind::Mod),
            ("~", LowTokenKind::BitwiseNot),
            ("^", LowTokenKind::BitwiseXor),
            ("&", LowTokenKind::BitwiseAnd),
            ("|", LowTokenKind::BitwiseOr),
            ("<", LowTokenKind::Lt),
            (">", LowTokenKind::Gt),
        ];

        for (input, kind) in cases {
            assert_tokens(input, &[(kind, 1)]);
        }
    }

    #[test]
    fn test_unknown_tokens_ascii_and_unicode() {
        assert_tokens("$", &[(LowTokenKind::Unknown, 1)]);
        assert_tokens("`", &[(LowTokenKind::Unknown, 1)]);

        for c in ['€', '🙂'] {
            let expected_kind = if c == '_' || UnicodeXID::is_xid_start(c) {
                LowTokenKind::Id
            } else {
                LowTokenKind::Unknown
            };

            let input = c.to_string();
            assert_kinds(&input, &[expected_kind]);
        }
    }

    #[test]
    fn test_compound_operators_are_split_currently() {
        assert_kinds(
            "-> == += <<= >>= && || ^=",
            &[
                LowTokenKind::Sub,
                LowTokenKind::Gt,
                LowTokenKind::Whitespace,
                LowTokenKind::Assign,
                LowTokenKind::Assign,
                LowTokenKind::Whitespace,
                LowTokenKind::Add,
                LowTokenKind::Assign,
                LowTokenKind::Whitespace,
                LowTokenKind::Lt,
                LowTokenKind::Lt,
                LowTokenKind::Assign,
                LowTokenKind::Whitespace,
                LowTokenKind::Gt,
                LowTokenKind::Gt,
                LowTokenKind::Assign,
                LowTokenKind::Whitespace,
                LowTokenKind::BitwiseAnd,
                LowTokenKind::BitwiseAnd,
                LowTokenKind::Whitespace,
                LowTokenKind::BitwiseOr,
                LowTokenKind::BitwiseOr,
                LowTokenKind::Whitespace,
                LowTokenKind::BitwiseXor,
                LowTokenKind::Assign,
            ],
        );
    }

    #[test]
    fn test_length_and_eof_invariants() {
        let cases = [
            "",
            "abc",
            " \t\r\n",
            "# hello\nworld",
            "#/doc/#",
            "#/unterminated",
            "a+b*(c-2)",
            "Δx = 42",
            "🙂 + €",
            "#/line 1\nline 2/#tail",
        ];

        for input in cases {
            assert_len_sum_matches_input(input);
        }
    }
}
