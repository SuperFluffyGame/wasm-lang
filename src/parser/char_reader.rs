pub struct CharReader {
    index: usize,
    _chars: Vec<char>,
}

impl CharReader {
    pub fn new(s: &str) -> Self {
        Self {
            index: 0,
            _chars: s.chars().collect(),
        }
    }
    pub fn peek(&self) -> Option<char> {
        self._chars.get(self.index).copied()
    }
    pub fn next(&mut self) -> Option<char> {
        let c = self._chars.get(self.index).copied();
        self.index += 1;
        c
    }
}
