mod expected_token_set;
mod token_lookahead;
mod token_matcher;
mod token_matcher_context_provider;
mod token_type;

pub use token_matcher::*;
pub use token_type::*;

use self::{
    expected_token_set::ExpectedTokenSet, token_lookahead::TokenLookahead,
    token_matcher_context_provider::TokenMatcherContextProvider,
};
use tl_diagnostic::{DiagnosticItem, DiagnosticItemBuilder};
use tl_lexer::Token;
use tl_span::{BytePos, Span};

pub struct Cursor<I>
where
    I: Iterator<Item = Token>,
{
    lookahead: TokenLookahead<I>,
    last_span: Span,
    source_hi: BytePos,
    expected_token_set: ExpectedTokenSet,
}

impl<I> Cursor<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(source_hi: BytePos, iter: I) -> Self {
        let lookahead = TokenLookahead::new(iter);
        let last_span = lookahead
            .first()
            .map(|token| token.span)
            .unwrap_or(Span::new(source_hi, source_hi));

        Self {
            lookahead,
            last_span,
            source_hi,
            expected_token_set: ExpectedTokenSet::new(),
        }
    }

    pub fn discard(&mut self) -> Option<Token> {
        self.lookahead.next()
    }

    pub fn span(&self) -> Span {
        self.lookahead
            .first()
            .map(|token| token.span)
            .unwrap_or(Span::new(self.source_hi, self.source_hi))
    }

    pub fn last_span(&self) -> Span {
        self.last_span
    }

    pub fn is_eof(&self) -> bool {
        self.lookahead
            .first()
            .map(|token| TokenType::from_token_kind(&token.kind))
            .unwrap_or(TokenType::Eof)
            == TokenType::Eof
    }

    pub fn first_token(&self) -> Option<&Token> {
        self.lookahead.first()
    }

    pub fn second_token(&self) -> Option<&Token> {
        self.lookahead.second()
    }

    pub fn first(&mut self) -> TokenMatcher<impl TokenMatcherContextProvider> {
        struct First<'a, I>
        where
            I: Iterator<Item = Token>,
        {
            lookahead: &'a mut TokenLookahead<I>,
            last_span: &'a mut Span,
            expected_token_set: &'a mut ExpectedTokenSet,
        }

        impl<'a, I> TokenMatcherContextProvider for First<'a, I>
        where
            I: Iterator<Item = Token>,
        {
            fn token(&self) -> Option<&Token> {
                self.lookahead.first()
            }

            fn consume(&mut self) -> Option<Token> {
                let token = self.lookahead.next()?;
                *self.last_span = token.span;
                Some(token)
            }

            fn add_expected_token(&mut self, token: TokenType) {
                self.expected_token_set.add(token);
            }
        }

        TokenMatcher::new(First {
            lookahead: &mut self.lookahead,
            last_span: &mut self.last_span,
            expected_token_set: &mut self.expected_token_set,
        })
    }

    pub fn second(&mut self) -> TokenMatcher<impl TokenMatcherContextProvider> {
        struct Second<'a, I>
        where
            I: Iterator<Item = Token>,
        {
            lookahead: &'a mut TokenLookahead<I>,
        }

        impl<'a, I> TokenMatcherContextProvider for Second<'a, I>
        where
            I: Iterator<Item = Token>,
        {
            fn token(&self) -> Option<&Token> {
                self.lookahead.second()
            }

            fn consume(&mut self) -> Option<Token> {
                panic!("cannot consume second token");
            }

            fn add_expected_token(&mut self, _token: TokenType) {
                // no-op
            }
        }

        TokenMatcher::new(Second {
            lookahead: &mut self.lookahead,
        })
    }

    pub fn make_unexpected_token_err(&mut self) -> DiagnosticItem {
        let mut expected_token_set = ExpectedTokenSet::new();
        std::mem::swap(&mut self.expected_token_set, &mut expected_token_set);

        let token_ty = self
            .lookahead
            .first()
            .map(|token| TokenType::from_token_kind(&token.kind))
            .unwrap_or(TokenType::Eof);

        let expected = expected_token_set.into_expected_list_str();
        let got = token_ty.as_str();
        let err_msg = format!("expected {expected}, got {got}");

        DiagnosticItemBuilder::error(self.span(), err_msg).build()
    }
}
