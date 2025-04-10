use std::iter::Peekable;
use std::str::Chars;

use crate::front::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn next_token_internal(&mut self) -> Option<Token> {
        while let Some(&ch) = self.input.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.input.next(); // Consume whitespace
                    continue;
                }
                '0'..='9' => return Some(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.identifier_or_keyword()),
                '+' => {
                    self.input.next();
                    return Some(Token::Plus);
                }
                '-' => {
                    self.input.next();
                    if self.input.peek() == Some(&'>') {
                        self.input.next();
                        return Some(Token::Arrow);
                    }
                    return Some(Token::Minus);
                }
                '*' => {
                    self.input.next();
                    return Some(Token::Asterisk);
                }
                '/' => {
                    self.input.next();
                    return Some(Token::FSlash);
                }
                '(' => {
                    self.input.next();
                    return Some(Token::LPar);
                }
                ')' => {
                    self.input.next();
                    return Some(Token::RPar);
                }
                '{' => {
                    self.input.next();
                    return Some(Token::LCurl);
                }
                '}' => {
                    self.input.next();
                    return Some(Token::RCurl);
                }
                ',' => {
                    self.input.next();
                    return Some(Token::Comma);
                }
                ';' => {
                    self.input.next();
                    return Some(Token::Semicolon);
                }
                ':' => {
                    self.input.next();
                    return Some(Token::Colon);
                }
                _ => {
                    let unknown = self.input.next().unwrap();
                    println!("Warning: Unknown token '{}'", unknown); // Debugging info
                    return Some(Token::Unknown(unknown));
                }
            }
        }
        None
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut has_decimal = false;

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.input.next();
            } else if ch == '.' {
                if has_decimal || self.input.clone().nth(1) == Some('.') {
                    break; // Stop if we already have a decimal or if next char is also a dot
                }
                has_decimal = true;
                num_str.push(ch);
                self.input.next();
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
                self.input.next();
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
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token_internal()
    }
}
