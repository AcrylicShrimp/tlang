#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LowTokenKind {
    // common
    Eof,
    Unknown,
    Whitespace,
    Comment,
    DocComment,
    UnterminatedDocComment,

    // keywords
    Id,

    // literals
    LitInteger,

    // punctuations - common
    Dot,
    Comma,
    Semicolon,
    Colon,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    At,
    Bang,

    // operators - assignments
    Assign,

    // operators - arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // operators - bitwise
    BitwiseNot,
    BitwiseXor,
    BitwiseAnd,
    BitwiseOr,

    // operators - comparison
    Lt,
    Gt,
}
