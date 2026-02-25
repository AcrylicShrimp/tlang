#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // common
    Eof,
    Unknown,
    Whitespace,
    Comment,
    DocComment,

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
