use crate::{
    ast::*,
    cursor::{Cursor, TokenType},
    primitive::*,
};
use tl_diagnostic::DiagnosticItem;
use tl_lexer::Token;
use tl_span::Span;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub(super) cursor: Cursor<I>,
    pub(super) diagnostics: Vec<DiagnosticItem>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(cursor: Cursor<I>) -> Self {
        Self {
            cursor,
            diagnostics: vec![],
        }
    }

    pub fn span_range(&self, from_span: Span) -> Span {
        Span::union(from_span, self.cursor.last_span())
    }

    pub fn into_diagnostics(self) -> Vec<DiagnosticItem> {
        self.diagnostics
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_type_name(&mut self) -> Option<AstTypeName> {
        let span = self.cursor.span();

        let path = self.parse_path()?;

        Some(AstTypeName {
            span: self.span_range(span),
            path,
        })
    }

    pub fn parse_path(&mut self) -> Option<AstPath> {
        let span = self.cursor.span();

        let first = self.parse_id()?;
        let mut rest = Vec::new();

        while self.cursor.first().is(TokenType::PathSep) && self.cursor.second().is(TokenType::Id) {
            let path_sep = self.parse_fixed()?;
            let id = self.parse_id()?;
            rest.push((path_sep, id));
        }

        Some(AstPath {
            span: self.span_range(span),
            first,
            rest,
        })
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_module(&mut self) -> AstModule {
        let span = self.cursor.span();

        let mut items = vec![];

        while !self.cursor.is_eof() {
            if let Some(item) = self.parse_top_level_item() {
                items.push(item);
            }
        }

        AstModule {
            span: self.span_range(span),
            items,
        }
    }

    pub fn parse_top_level_item(&mut self) -> Option<AstTopLevelItem> {
        let span = self.cursor.span();

        let kind = if self.cursor.first().is(TokenType::KwUse) {
            AstTopLevelItemKind::Use(self.parse_use()?)
        } else if self.cursor.first().is(TokenType::KwExpose)
            && self.cursor.second().is(TokenType::KwRef)
            && self.cursor.third().is(TokenType::KwFn)
        {
            AstTopLevelItemKind::ExposeRefFn(self.parse_expose_ref_fn()?)
        } else {
            self.diagnostics
                .push(self.cursor.make_unexpected_token_err());
            self.cursor.discard();
            return None;
        };

        Some(AstTopLevelItem {
            span: self.span_range(span),
            kind,
        })
    }

    pub fn parse_use(&mut self) -> Option<AstUse> {
        let span = self.cursor.span();

        let kw_use = self.parse_fixed()?;
        let path = self.parse_path()?;
        let tail = if self.cursor.first().is(TokenType::KwAs)
            || self.cursor.first().is(TokenType::PathSep)
        {
            Some(self.parse_use_tail()?)
        } else {
            None
        };
        let semicolon = self.parse_fixed()?;

        Some(AstUse {
            span: self.span_range(span),
            kw_use,
            path,
            tail,
            semicolon,
        })
    }

    pub fn parse_use_tail(&mut self) -> Option<AstUseTail> {
        let span = self.cursor.span();

        let kind = if self.cursor.first().is(TokenType::KwAs) {
            AstUseTailKind::As(self.parse_use_tail_as()?)
        } else if self.cursor.first().is(TokenType::PathSep) {
            AstUseTailKind::All(self.parse_use_tail_all()?)
        } else {
            self.diagnostics
                .push(self.cursor.make_unexpected_token_err());
            self.cursor.discard();
            return None;
        };

        Some(AstUseTail {
            span: self.span_range(span),
            kind,
        })
    }

    pub fn parse_use_tail_as(&mut self) -> Option<AstUseTailAs> {
        let span = self.cursor.span();

        let kw_as = self.parse_fixed()?;
        let name = self.parse_id()?;

        Some(AstUseTailAs {
            span: self.span_range(span),
            kw_as,
            name,
        })
    }

    pub fn parse_use_tail_all(&mut self) -> Option<AstUseTailAll> {
        let span = self.cursor.span();

        let path_sep = self.parse_fixed()?;
        let star = self.parse_fixed()?;

        Some(AstUseTailAll {
            span: self.span_range(span),
            path_sep,
            star,
        })
    }

    pub fn parse_expose_ref_fn(&mut self) -> Option<AstExposeRefFn> {
        let span = self.cursor.span();

        let kw_expose = self.parse_fixed()?;
        let kw_ref = self.parse_fixed()?;
        let kw_fn = self.parse_fixed()?;
        let name = self.parse_id()?;
        let paren_open = self.parse_fixed()?;
        let params = if self.cursor.first().is(TokenType::ParenClose) {
            None
        } else {
            Some(self.parse_params()?)
        };
        let paren_close = self.parse_fixed()?;
        let arrow = self.parse_fixed()?;
        let return_ty = self.parse_type_name()?;
        let semicolon = self.parse_fixed()?;

        Some(AstExposeRefFn {
            span: self.span_range(span),
            kw_expose,
            kw_ref,
            kw_fn,
            name,
            paren_open,
            params,
            paren_close,
            arrow,
            return_ty,
            semicolon,
        })
    }

    pub fn parse_params(&mut self) -> Option<AstFnParams> {
        let span = self.cursor.span();

        let first = self.parse_fn_param()?;
        let mut rest = Vec::new();

        while self.cursor.first().is(TokenType::Comma) {
            let comma = self.parse_fixed()?;
            let param = self.parse_fn_param()?;
            rest.push((comma, param));
        }

        Some(AstFnParams {
            span: self.span_range(span),
            first,
            rest,
        })
    }

    pub fn parse_fn_param(&mut self) -> Option<AstFnParam> {
        let span = self.cursor.span();

        let id = self.parse_id()?;
        let colon = self.parse_fixed()?;
        let ty = self.parse_type_name()?;

        Some(AstFnParam {
            span: self.span_range(span),
            id,
            colon,
            ty,
        })
    }
}
