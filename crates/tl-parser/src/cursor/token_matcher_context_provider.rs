use super::token_type::TokenType;
use tl_lexer::Token;

pub trait TokenMatcherContextProvider {
    fn token(&self) -> Option<&Token>;
    fn consume(&mut self) -> Option<Token>;
    fn add_expected_token(&mut self, token: TokenType);
}
