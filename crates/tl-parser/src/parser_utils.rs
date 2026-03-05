use crate::{FixedToken, Parser, common::AstId};
use tl_lexer::Token;

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_id(&mut self) -> Option<AstId> {
        let id = self.cursor.first().id();

        match id {
            Some(id) => Some(id),
            None => {
                let err = self.cursor.make_unexpected_token_err();
                self.diagnostics.push(err);
                self.cursor.discard();
                None
            }
        }
    }

    pub fn parse_fixed<T>(&mut self) -> Option<T>
    where
        T: FixedToken,
    {
        let fixed = self.cursor.first().fixed();

        match fixed {
            Some(fixed) => Some(fixed),
            None => {
                let err = self.cursor.make_unexpected_token_err();
                self.diagnostics.push(err);
                self.cursor.discard();
                None
            }
        }
    }
}
