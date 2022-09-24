// LEXER
mod lex;
pub enum LexerError {}
pub struct Lexer<'a> {
    index: i32,
    input: &'a str,
    errors: Vec<LexerError>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            index: 0,
            input,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, e: LexerError) {
        self.errors.push(e);
    }
}

// TOKENS
pub enum TokenType {}
pub struct Token {
    t: TokenType,
    line: i32,
    col: i32,
}

// PARSER
mod parse;
mod tree;
pub enum ParserError {}
pub struct Parser<'a> {
    index: i32,
    tokens: &'a Vec<Token>,
    errors: Vec<ParserError>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, e: ParserError) {
        self.errors.push(e);
    }
}
