#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // common
    Eof,
    Unknown,
    Whitespace,
    Comment(TokenComment),

    // keywords
    Id(String),

    // literals
    LitBool {
        content: String,
    },
    LitInteger {
        prefix: Option<String>,
        content: String,
        suffix: Option<String>,
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

    // operators - assignments
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
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
    LogicalXor,
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
