use crate::parser::TokenType;

use super::{super::Token, Lexer, LexerErrorType};

impl Lexer {
    pub(super) fn _next(&mut self) -> Option<Token> {
        let next = self.next.clone();

        if let None = self.reader.peek() {
            self.next = Some(Token {
                t: TokenType::EOF,
                line: self.line,
                col: self.col + 1,
            });
            return next;
        }

        while let Some(char) = self.reader.next() {
            self.col += 1;
            let start_col = self.col;
            let start_line = self.line;
            let t = match char {
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Asterisk,
                '=' => TokenType::Equal,
                ';' => TokenType::Semi,
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                ',' => TokenType::Comma,

                _ => {
                    // ident / kw
                    if char.is_alphabetic() || char == '_' {
                        self.ident(char)
                    }
                    // number
                    else if char.is_numeric() || char == '.' {
                        self.number(char)
                    }
                    // whitespace
                    else if char.is_whitespace() {
                        if char == '\n' {
                            self.line += 1;
                            self.col = 0;
                        }
                        continue;
                    }
                    // unknown char
                    else {
                        self.error(LexerErrorType::UnexpectedCharacter(char));
                        continue;
                    }
                }
            };
            self.next = Some(Token::new(t, start_line, start_col));
            break;
        }
        next
    }
    pub fn next(&mut self) -> Token {
        self._next().unwrap()
    }
    pub fn peek(&self) -> Token {
        self.next.clone().unwrap()
    }
    pub fn peek_ref(&self) -> &Token {
        self.next.as_ref().unwrap()
    }
    fn ident(&mut self, initial_char: char) -> TokenType {
        let mut ident = String::from(initial_char);
        while let Some(ident_char) = self.reader.peek() {
            if ident_char.is_alphanumeric() || ident_char == '_' {
                self.col += 1;
                ident.push(self.reader.next().unwrap());
            } else {
                break;
            }
        }
        if ident == "let" {
            TokenType::KwLet
        } else {
            TokenType::Ident(ident)
        }
    }
    fn number(&mut self, initial_char: char) -> TokenType {
        if initial_char == '.' {
            let mut float_str = String::from(initial_char);
            while let Some(float_char) = self.reader.peek() {
                if float_char.is_numeric() {
                    self.col += 1;
                    float_str.push(self.reader.next().unwrap());
                } else {
                    break;
                }
            }
            return TokenType::FloatLiteral(float_str.parse().unwrap());
        } else {
            let mut num_str = String::from(initial_char);
            while let Some(before_decimal_char) = self.reader.peek() {
                if before_decimal_char.is_numeric() {
                    self.col += 1;
                    num_str.push(self.reader.next().unwrap())
                } else {
                    break;
                }
            }

            if let Some('.') = self.reader.peek() {
                self.col += 1;
                num_str.push(self.reader.next().unwrap());
                while let Some(after_decimal_char) = self.reader.peek() {
                    if after_decimal_char.is_numeric() {
                        self.col += 1;
                        num_str.push(self.reader.next().unwrap())
                    } else {
                        break;
                    }
                }
                return TokenType::FloatLiteral(num_str.parse().unwrap());
            } else {
                return TokenType::IntLiteral(num_str.parse().unwrap());
            }
        }
    }
}
