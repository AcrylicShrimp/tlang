#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // common
    Eof,
    Unknown,
    Whitespace,
    Comment(TokenComment),

    // keywords
    Id(String),
    KwUse,
    KwAs,

    // literals
    LitBool {
        content: String,
    },
    LitInteger {
        content: String,
        suffix: Option<String>,
    },
    LitFloat {
        content: String,
        suffix: Option<String>,
    },
    LitString {
        content: String,
        is_terminated: bool,
    },

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

    // punc - compound
    Arrow,
    PathSep,

    // operators - assignments
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    BitwiseNotAssign,
    BitwiseXorAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseShiftLeftAssign,
    BitwiseShiftRightAssign,

    // operators - arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // operators - bitwise
    BitwiseNot,
    BitwiseXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    // operators - comparison
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,

    // operators - logical
    LogicalNot,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenComment {
    Line {
        content: String,
    },
    Doc {
        content: String,
        is_terminated: bool,
    },
}
