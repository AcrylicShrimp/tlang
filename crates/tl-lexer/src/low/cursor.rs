use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Cursor<'s> {
    chars: Chars<'s>,
    initial_len: usize,
}

impl<'s> Cursor<'s> {
    pub fn new(s: &'s str) -> Self {
        Self {
            chars: s.chars(),
            initial_len: s.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn len(&self) -> usize {
        self.chars.as_str().len()
    }

    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.len()
    }

    pub fn at(&self, index: usize) -> char {
        self.chars.clone().nth(index).unwrap_or_default()
    }

    pub fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn consume_while(&mut self, mut predicate: impl FnMut(&Self) -> bool) {
        while predicate(self) {
            self.consume();
        }
    }
}
