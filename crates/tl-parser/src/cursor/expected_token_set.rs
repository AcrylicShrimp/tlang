use super::TokenType;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExpectedTokenSet {
    tokens: BTreeSet<TokenType>,
}

impl ExpectedTokenSet {
    pub fn new() -> Self {
        Self {
            tokens: BTreeSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn add(&mut self, token: TokenType) {
        self.tokens.insert(token);
    }

    pub fn into_expected_list_str(self) -> String {
        let mut expected_list = self
            .tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>();

        if expected_list.is_empty() {
            expected_list.push(TokenType::Eof.as_str());
        }

        match expected_list.split_last() {
            Some((last, rest)) => {
                let rest_part = rest
                    .iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("{rest_part} or {last}")
            }
            None => expected_list[0].to_owned(),
        }
    }
}
