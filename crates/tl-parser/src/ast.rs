use crate::{
    fixed::{keywords::*, operators::*, punctuations::*},
    primitive::*,
    utils::*,
};
use tl_span::Span;

pub mod primitive {
    use super::utils::PunctuatedNoTrailing;
    use crate::punctuations::*;
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

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AstTypeName {
        pub span: Span,
        pub id: AstId,
    }

    pub type AstPath = PunctuatedNoTrailing<AstId, PuncDot>;
}

pub mod utils {
    use tl_span::Span;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Punctuated<T, P> {
        pub span: Span,
        pub first: T,
        pub rest: Vec<(P, T)>,
        pub trailing: Option<P>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PunctuatedNoTrailing<T, P> {
        pub span: Span,
        pub first: T,
        pub rest: Vec<(P, T)>,
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
    ExposeRefFn(AstExposeRefFn),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstExposeRefFn {
    pub span: Span,
    pub kw_expose: KwExpose,
    pub kw_ref: KwRef,
    pub kw_fn: KwFn,
    pub name: AstId,
    pub paren_open: PuncParenOpen,
    pub params: Option<Punctuated<AstFnParam, PuncComma>>,
    pub paren_close: PuncParenClose,
    pub arrow: PuncArrow,
    pub return_ty: AstTypeName,
    pub semicolon: PuncSemicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstFnParam {
    pub span: Span,
    pub id: AstId,
    pub colon: PuncColon,
    pub ty: AstTypeName,
}
