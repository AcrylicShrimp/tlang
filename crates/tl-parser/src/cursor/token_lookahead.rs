use tl_lexer::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenLookahead<I>
where
    I: Iterator<Item = Token>,
{
    first: Option<Token>,
    second: Option<Token>,
    third: Option<Token>,
    iter: I,
}

impl<I> TokenLookahead<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(mut iter: I) -> Self {
        let mut lookahead = Self {
            first: iter.next(),
            second: iter.next(),
            third: iter.next(),
            iter,
        };

        lookahead.trim();
        lookahead
    }

    pub fn first(&self) -> Option<&Token> {
        self.first.as_ref()
    }

    pub fn second(&self) -> Option<&Token> {
        self.second.as_ref()
    }

    pub fn third(&self) -> Option<&Token> {
        self.third.as_ref()
    }

    pub fn next(&mut self) -> Option<Token> {
        let token = self.first.take();

        self.first = self.second.take();
        self.second = self.third.take();
        self.third = self.iter.next();
        self.trim();

        token
    }

    fn trim(&mut self) {
        while matches!(
            self.first.as_ref().map(|token| &token.kind),
            Some(TokenKind::Whitespace | TokenKind::Comment(_))
        ) {
            self.first = self.second.take();
            self.second = self.third.take();
            self.third = self.iter.next();
        }

        while matches!(
            self.second.as_ref().map(|token| &token.kind),
            Some(TokenKind::Whitespace | TokenKind::Comment(_))
        ) {
            self.second = self.third.take();
            self.third = self.iter.next();
        }

        while matches!(
            self.third.as_ref().map(|token| &token.kind),
            Some(TokenKind::Whitespace | TokenKind::Comment(_))
        ) {
            self.third = self.iter.next();
        }
    }
}
