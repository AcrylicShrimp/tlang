use self::common::AstId;
use crate::{fixed::keywords::*, fixed::operators::*, fixed::punctuations::*};
use tl_span::Span;

pub mod common {
    use tl_span::{BytePos, Span};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AstId {
        pub lo: BytePos,
        pub name: String,
    }

    impl AstId {
        pub fn new(lo: BytePos, name: String) -> Self {
            Self { lo, name }
        }

        pub fn span(&self) -> Span {
            Span::new(self.lo, self.lo + BytePos::new(self.name.len() as u32))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstModule {
    pub span: Span,
    pub items: Vec<AstTopLevelItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstTopLevelItem {
    pub span: Span,
    pub kind: AstTopLevelItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstTopLevelItemKind {
    Use(AstUse),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUse {
    pub span: Span,
    pub kw_use: KwUse,
    pub path: AstPath,
    pub tail: Option<AstUseTail>,
    pub semicolon: PuncSemicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstPath {
    pub span: Span,
    pub segment: AstId,
    pub extends: Vec<AstPathExtend>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstPathExtend {
    pub span: Span,
    pub dot: PuncDot,
    pub name: AstId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUseTail {
    pub span: Span,
    pub kind: AstUseTailKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstUseTailKind {
    As(AstUseTailAs),
    All(AstUseTailAll),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUseTailAs {
    pub span: Span,
    pub kw_as: KwAs,
    pub name: AstId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstUseTailAll {
    pub span: Span,
    pub dot: PuncDot,
    pub star: OpMul,
}
