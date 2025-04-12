use std::iter::Peekable;
use std::str::Chars;

use crate::front::token::Token;

use super::token::Position;

pub struct Lexer<'a> {
    position: Position,
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            position: Position { line: 1, index: 1 },
            input: input.chars().peekable(),
        }
    }

    fn update_position(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.position.line += 1;
                self.position.index = 1; // Reset index at the start of a new line
            }
            _ => {
                self.position.index += 1; // Increment index for other characters
            }
        }
    }

    fn next_token_internal(&mut self) -> Option<(Token, Position)> {
        if let Some(&ch) = self.input.peek() {
            while let Some(&ch) = self.input.peek() {
                match ch {
                    ' ' | '\t' | '\r' => {
                        self.input.next(); // Consume whitespace
                        self.update_position(ch);
                        continue;
                    }
                    '\n' => {
                        self.input.next(); // Consume newline
                        self.update_position(ch);
                        continue;
                    }
                    '0'..='9' => {
                        return Some((self.number(), self.position.clone())); // Token positions updated inside `number()`
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        return Some((self.identifier_or_keyword(), self.position.clone()));
                        // Similar here
                    }
                    '+' => {
                        self.input.next(); // Consume '+'
                        self.update_position(ch);
                        return Some((Token::Plus, self.position.clone()));
                    }
                    '-' => {
                        self.input.next(); // Consume '-'
                        self.update_position(ch);
                        if let Some(&next_ch) = self.input.peek() {
                            if next_ch == '>' {
                                self.input.next(); // Consume '>'
                                self.update_position(next_ch);
                                return Some((Token::Arrow, self.position.clone()));
                            }
                        }
                        return Some((Token::Minus, self.position.clone()));
                    }
                    '*' => {
                        self.input.next(); // Consume '*'
                        self.update_position(ch);
                        return Some((Token::Asterisk, self.position.clone()));
                    }
                    '/' => {
                        self.input.next(); // Consume '/'
                        self.update_position(ch);
                        return Some((Token::Fslash, self.position.clone()));
                    }
                    '%' => {
                        self.input.next(); // Consume '%'
                        self.update_position(ch);
                        return Some((Token::Percent, self.position.clone()));
                    }
                    '(' => {
                        self.input.next(); // Consume '('
                        self.update_position(ch);
                        return Some((Token::LPar, self.position.clone()));
                    }
                    ')' => {
                        self.input.next(); // Consume ')'
                        self.update_position(ch);
                        return Some((Token::RPar, self.position.clone()));
                    }
                    '{' => {
                        self.input.next(); // Consume '{'
                        self.update_position(ch);
                        return Some((Token::LCurl, self.position.clone()));
                    }
                    '}' => {
                        self.input.next(); // Consume '}'
                        self.update_position(ch);
                        return Some((Token::RCurl, self.position.clone()));
                    }
                    ',' => {
                        self.input.next(); // Consume ','
                        self.update_position(ch);
                        return Some((Token::Comma, self.position.clone()));
                    }
                    ';' => {
                        self.input.next(); // Consume ';'
                        self.update_position(ch);
                        return Some((Token::Semicolon, self.position.clone()));
                    }
                    ':' => {
                        self.input.next(); // Consume ':'
                        self.update_position(ch);
                        return Some((Token::Colon, self.position.clone()));
                    }
                    _ => {
                        self.input.next(); // Consume unknown character
                        self.update_position(ch);
                        println!("Warning: Unknown token '{}'", ch);
                        return Some((Token::Unknown(ch), self.position.clone()));
                    }
                }
            }

            Some((Token::Eof, self.position.clone()))
        } else {
            None
        }
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut has_decimal = false;

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.input.next(); // Consume digit
                self.update_position(ch);
            } else if ch == '.' {
                if has_decimal || self.input.clone().nth(1) == Some('.') {
                    break;
                }
                has_decimal = true;
                num_str.push(ch);
                self.input.next(); // Consume '.'
                self.update_position(ch);
            } else {
                break;
            }
        }

        Token::Number(num_str)
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.input.next(); // Consume character
                self.update_position(ch);
            } else {
                break;
            }
        }

        match ident.as_str() {
            "fn" => Token::Fn,
            "ret" => Token::Ret,
            "struct" => Token::Struct,
            "pub" => Token::Pub,
            "enum" => Token::Enum,
            "impl" => Token::Impl,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "i32" => Token::I32,
            "i64" => Token::I64,
            "u32" => Token::U32,
            "u64" => Token::U64,
            "usize" => Token::Usize,
            "f32" => Token::F32,
            "f64" => Token::F64,
            "char" => Token::Char,
            _ => Token::Identifier(ident),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Position);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token_internal()
    }
}
