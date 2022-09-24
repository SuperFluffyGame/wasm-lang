use std::{iter::Peekable, str::Chars};

use super::{super::Token, super::TokenType, Lexer, LexerErrorType};

impl<'a> Lexer<'a> {
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(char) = chars.next() {
            self.col += 1;
            let start_col = self.col;
            let start_line = self.line;
            let t = match char {
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Asterisk,
                '=' => TokenType::Equal,
                ';' => TokenType::Semi,

                _ => {
                    // ident / kw
                    if char.is_alphabetic() || char == '_' {
                        self.ident(char, &mut chars)
                    }
                    // number
                    else if char.is_numeric() || char == '.' {
                        self.number(char, &mut chars)
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
            tokens.push(Token::new(t, start_line, start_col))
        }

        tokens
    }

    fn ident(&mut self, initial_char: char, chars: &mut Peekable<Chars>) -> TokenType {
        let mut ident = String::from(initial_char);
        while let Some(ident_char) = chars.peek() {
            if ident_char.is_alphanumeric() || ident_char == &'_' {
                self.col += 1;
                ident.push(chars.next().unwrap());
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
    fn number(&mut self, initial_char: char, chars: &mut Peekable<Chars>) -> TokenType {
        if initial_char == '.' {
            let mut float_str = String::from(initial_char);
            while let Some(float_char) = chars.peek() {
                if float_char.is_numeric() {
                    self.col += 1;
                    float_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            return TokenType::FloatLiteral(float_str.parse().unwrap());
        } else {
            let mut num_str = String::from(initial_char);
            while let Some(before_decimal_char) = chars.peek() {
                if before_decimal_char.is_numeric() {
                    self.col += 1;
                    num_str.push(chars.next().unwrap())
                } else {
                    break;
                }
            }

            if let Some('.') = chars.peek() {
                self.col += 1;
                num_str.push(chars.next().unwrap());
                while let Some(after_decimal_char) = chars.peek() {
                    if after_decimal_char.is_numeric() {
                        self.col += 1;
                        num_str.push(chars.next().unwrap())
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
