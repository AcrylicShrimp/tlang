use crate::{
    ast::*,
    cursor::{Cursor, TokenType},
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
        let tail =
            if self.cursor.first().is(TokenType::Dot) || self.cursor.first().is(TokenType::KwAs) {
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

    pub fn parse_path(&mut self) -> Option<AstPath> {
        let span = self.cursor.span();

        let segment = self.parse_id()?;
        let mut extends = Vec::new();

        while self.cursor.first().is(TokenType::Dot) && self.cursor.second().is(TokenType::Id) {
            extends.push(self.parse_path_extend()?);
        }

        Some(AstPath {
            span: self.span_range(span),
            segment,
            extends,
        })
    }

    pub fn parse_path_extend(&mut self) -> Option<AstPathExtend> {
        let span = self.cursor.span();

        let dot = self.parse_fixed()?;
        let name = self.parse_id()?;

        Some(AstPathExtend {
            span: self.span_range(span),
            dot,
            name,
        })
    }

    pub fn parse_use_tail(&mut self) -> Option<AstUseTail> {
        let span = self.cursor.span();

        let kind = if self.cursor.first().is(TokenType::KwAs) {
            AstUseTailKind::As(self.parse_use_tail_as()?)
        } else if self.cursor.first().is(TokenType::Dot) {
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

        let dot = self.parse_fixed()?;
        let star = self.parse_fixed()?;

        Some(AstUseTailAll {
            span: self.span_range(span),
            dot,
            star,
        })
    }
}
