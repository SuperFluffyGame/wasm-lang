use std::str::Chars;

pub struct CharReader<'a> {
    input: &'a str,
    index: usize,
    _chars: Vec<char>,
}

impl<'a> CharReader<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            input: s,
            index: 0,
            _chars: s.chars().collect(),
        }
    }
    pub fn peek(&self) -> Option<&char> {
        self._chars.get(self.index)
    }
    pub fn next(&mut self) -> Option<&char> {
        let c = self._chars.get(self.index);
        self.index += 1;
        c
    }
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.index += 1;
            }
        }
    }
}
