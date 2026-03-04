use super::{token_matcher_context_provider::TokenMatcherContextProvider, token_type::TokenType};
use crate::{FixedToken, common::AstId};
use tl_lexer::TokenKind;

pub struct TokenMatcher<C>
where
    C: TokenMatcherContextProvider,
{
    context_provider: C,
}

impl<C> TokenMatcher<C>
where
    C: TokenMatcherContextProvider,
{
    pub fn new(context_provider: C) -> Self {
        Self { context_provider }
    }

    pub fn is(&mut self, ty: TokenType) -> bool {
        self.context_provider.add_expected_token(ty);
        self.context_provider
            .token()
            .is_some_and(|token| token.kind == ty)
    }

    pub fn id(&mut self) -> Option<AstId> {
        self.context_provider.add_expected_token(TokenType::Id);

        let token = self.context_provider.token()?;

        match &token.kind {
            TokenKind::Id(_) => {}
            _ => {
                return None;
            }
        }

        let token = self.context_provider.consume()?;

        match token.kind {
            TokenKind::Id(id) => Some(AstId::new(token.span.lo, id)),
            _ => unreachable!(),
        }
    }

    pub fn fixed<T>(&mut self) -> Option<T>
    where
        T: FixedToken,
    {
        self.context_provider.add_expected_token(T::TOKEN_TYPE);

        let token = self.context_provider.token()?;

        if token.kind != T::TOKEN_TYPE {
            return None;
        }

        let token = self.context_provider.consume()?;

        Some(T::new(token.span.lo))
    }
}
