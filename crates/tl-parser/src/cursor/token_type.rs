use tl_lexer::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    // common
    Eof,
    Unknown,
    Whitespace,
    Comment,

    // keywords
    Id,
    KwUse,
    KwAs,

    // literals
    LitBool,
    LitInteger,
    LitFloat,
    LitString,

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

impl TokenType {
    pub fn from_token_kind(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Eof => Self::Eof,
            TokenKind::Unknown => Self::Unknown,
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Comment(_) => Self::Comment,
            TokenKind::Id(_) => Self::Id,
            TokenKind::KwUse => Self::KwUse,
            TokenKind::KwAs => Self::KwAs,
            TokenKind::LitBool { .. } => Self::LitBool,
            TokenKind::LitInteger { .. } => Self::LitInteger,
            TokenKind::LitFloat { .. } => Self::LitFloat,
            TokenKind::LitString { .. } => Self::LitString,
            TokenKind::Dot => Self::Dot,
            TokenKind::Comma => Self::Comma,
            TokenKind::Semicolon => Self::Semicolon,
            TokenKind::Colon => Self::Colon,
            TokenKind::ParenOpen => Self::ParenOpen,
            TokenKind::ParenClose => Self::ParenClose,
            TokenKind::BraceOpen => Self::BraceOpen,
            TokenKind::BraceClose => Self::BraceClose,
            TokenKind::BracketOpen => Self::BracketOpen,
            TokenKind::BracketClose => Self::BracketClose,
            TokenKind::At => Self::At,
            TokenKind::Arrow => Self::Arrow,
            TokenKind::Assign => Self::Assign,
            TokenKind::AddAssign => Self::AddAssign,
            TokenKind::SubAssign => Self::SubAssign,
            TokenKind::MulAssign => Self::MulAssign,
            TokenKind::DivAssign => Self::DivAssign,
            TokenKind::ModAssign => Self::ModAssign,
            TokenKind::PowAssign => Self::PowAssign,
            TokenKind::BitwiseNotAssign => Self::BitwiseNotAssign,
            TokenKind::BitwiseXorAssign => Self::BitwiseXorAssign,
            TokenKind::BitwiseAndAssign => Self::BitwiseAndAssign,
            TokenKind::BitwiseOrAssign => Self::BitwiseOrAssign,
            TokenKind::BitwiseShiftLeftAssign => Self::BitwiseShiftLeftAssign,
            TokenKind::BitwiseShiftRightAssign => Self::BitwiseShiftRightAssign,
            TokenKind::Add => Self::Add,
            TokenKind::Sub => Self::Sub,
            TokenKind::Mul => Self::Mul,
            TokenKind::Div => Self::Div,
            TokenKind::Mod => Self::Mod,
            TokenKind::Pow => Self::Pow,
            TokenKind::BitwiseNot => Self::BitwiseNot,
            TokenKind::BitwiseXor => Self::BitwiseXor,
            TokenKind::BitwiseAnd => Self::BitwiseAnd,
            TokenKind::BitwiseOr => Self::BitwiseOr,
            TokenKind::BitwiseShiftLeft => Self::BitwiseShiftLeft,
            TokenKind::BitwiseShiftRight => Self::BitwiseShiftRight,
            TokenKind::Eq => Self::Eq,
            TokenKind::Neq => Self::Neq,
            TokenKind::Lt => Self::Lt,
            TokenKind::Le => Self::Le,
            TokenKind::Gt => Self::Gt,
            TokenKind::Ge => Self::Ge,
            TokenKind::LogicalNot => Self::LogicalNot,
            TokenKind::LogicalAnd => Self::LogicalAnd,
            TokenKind::LogicalOr => Self::LogicalOr,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            TokenType::Eof => "<eof>",
            TokenType::Unknown => "an unknown token",
            TokenType::Whitespace => "whitespaces",
            TokenType::Comment => "a comment",
            TokenType::Id => "an identifier",
            TokenType::KwUse => "`use`",
            TokenType::KwAs => "`as`",
            TokenType::LitBool => "a boolean literal",
            TokenType::LitInteger => "an integer literal",
            TokenType::LitFloat => "a float literal",
            TokenType::LitString => "a string literal",
            TokenType::Dot => "`.`",
            TokenType::Comma => "`,`",
            TokenType::Semicolon => "`;`",
            TokenType::Colon => "`:`",
            TokenType::ParenOpen => "`(`",
            TokenType::ParenClose => "`)`",
            TokenType::BraceOpen => "`{`",
            TokenType::BraceClose => "`}`",
            TokenType::BracketOpen => "`[`",
            TokenType::BracketClose => "`]`",
            TokenType::At => "`@`",
            TokenType::Arrow => "`->`",
            TokenType::Assign => "`=`",
            TokenType::AddAssign => "`+=`",
            TokenType::SubAssign => "`-=`",
            TokenType::MulAssign => "`*=`",
            TokenType::DivAssign => "`/=`",
            TokenType::ModAssign => "`%=`",
            TokenType::PowAssign => "`**=`",
            TokenType::BitwiseNotAssign => "`~=`",
            TokenType::BitwiseXorAssign => "`^=`",
            TokenType::BitwiseAndAssign => "`&=`",
            TokenType::BitwiseOrAssign => "`|=`",
            TokenType::BitwiseShiftLeftAssign => "`<<=`",
            TokenType::BitwiseShiftRightAssign => "`>>=`",
            TokenType::Add => "`+`",
            TokenType::Sub => "`-`",
            TokenType::Mul => "`*`",
            TokenType::Div => "`/`",
            TokenType::Mod => "`%`",
            TokenType::Pow => "`**`",
            TokenType::BitwiseNot => "`~`",
            TokenType::BitwiseXor => "`^`",
            TokenType::BitwiseAnd => "`&`",
            TokenType::BitwiseOr => "`|`",
            TokenType::BitwiseShiftLeft => "`<<`",
            TokenType::BitwiseShiftRight => "`>>`",
            TokenType::Eq => "`==`",
            TokenType::Neq => "`!=`",
            TokenType::Lt => "`<`",
            TokenType::Le => "`<=`",
            TokenType::Gt => "`>`",
            TokenType::Ge => "`>=`",
            TokenType::LogicalNot => "`not`",
            TokenType::LogicalAnd => "`and`",
            TokenType::LogicalOr => "`or`",
        }
    }
}

impl PartialEq<TokenKind> for TokenType {
    fn eq(&self, other: &TokenKind) -> bool {
        Self::from_token_kind(other) == *self
    }
}

impl PartialEq<TokenType> for TokenKind {
    fn eq(&self, other: &TokenType) -> bool {
        *other == TokenType::from_token_kind(self)
    }
}
