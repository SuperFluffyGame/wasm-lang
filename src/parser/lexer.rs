mod lex;
pub mod tokens;

#[derive(Debug)]
pub enum LexerErrorType {
    UnexpectedCharacter(char),
}
#[derive(Debug)]
pub struct LexerError {
    t: LexerErrorType,
    line: i32,
    col: i32,
}
pub struct Lexer<'a> {
    pub index: i32,
    pub col: i32,
    pub line: i32,
    pub input: &'a str,
    pub errors: Vec<LexerError>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            index: 0,
            col: 0,
            line: 0,
            input,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, t: LexerErrorType) {
        self.errors.push(LexerError {
            t,
            line: self.line,
            col: self.col,
        });
    }
}
