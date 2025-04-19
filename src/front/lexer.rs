use std::iter::Peekable;
use std::str::Chars;

use crate::front::token::Token;
use super::token::Position;

macro_rules! here {
    () => {
        println!(
            "Execution passed through here:\n\tfile: {}\n\tline: {}",
            file!(),
            line!()
        )
    };
}

pub struct Lexer<'a> {
    position: Position,
    input: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Position);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token_internal()
    }
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
                self.position.index = 1; // Reset index on a new line
            }
            _ => {
                self.position.index += 1;
            }
        }
    }

    /// Skip a nested multiline comment.
    /// Returns true if the comment was terminated correctly, false otherwise.
    fn skip_multiline_comment(&mut self) -> bool {
        let mut depth = 1;
        while let Some(ch) = self.input.next() {
            self.update_position(ch);
            match ch {
                '/' => {
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '*' {
                            self.input.next(); // Consume '*'
                            self.update_position('*');
                            depth += 1;
                        }
                    }
                }
                '*' => {
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '/' {
                            self.input.next(); // Consume '/'
                            self.update_position('/');
                            depth -= 1;
                            if depth == 0 {
                                return true;
                            }
                        }
                    }
                }
                _ => {} // Continue consuming comment characters.
            }
        }
        // If we exit the loop, EOF was reached before closing all nested comments.
        false
    }

    fn next_token_internal(&mut self) -> Option<(Token, Position)> {
        while let Some(&ch) = self.input.peek() {
            // Skip whitespace
            match ch {
                ' ' | '\t' | '\r' => {
                    self.input.next();
                    self.update_position(ch);
                    continue;
                }
                '\n' => {
                    self.input.next();
                    self.update_position(ch);
                    continue;
                }
                // String literal start
                '\"' => {
                    return Some((self.string_literal(), self.position.clone()));
                }
                // Character literal start
                '\'' => {
                    return Some((self.character_literal(), self.position.clone()));
                }
                '0'..='9' => {
                    let pos = self.position.clone();
                    return Some((self.number(), pos));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let pos = self.position.clone();
                    return Some((self.identifier_or_keyword(), pos));
                }
                '+' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::Plus, self.position.clone()));
                }
                '-' => {
                    self.input.next(); // Consume '-'
                    let pos = self.position.clone();
                    self.update_position(ch);
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '>' {
                            self.input.next();
                            self.update_position(next_ch);
                            return Some((Token::Arrow, pos));
                        }
                    }
                    return Some((Token::Minus, pos));
                }
                '&' => {
                    self.input.next(); // Consume '-'
                    let pos = self.position.clone();
                    self.update_position(ch);
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '&' {
                            self.input.next();
                            self.update_position(next_ch);
                            return Some((Token::And, pos));
                        }
                    }
                    return Some((Token::Ampersand, pos));
                }
                '|' => {
                    self.input.next(); // Consume '-'
                    let pos = self.position.clone();
                    self.update_position(ch);
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '&' {
                            self.input.next();
                            self.update_position(next_ch);
                            return Some((Token::Or, pos));
                        }
                    }
                    return Some((Token::Pipe, pos));
                }
                '*' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::Asterisk, self.position.clone()));
                }
                '/' => {
                    self.input.next(); // Consume '/'
                    self.update_position(ch);
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '/' {
                            self.input.next(); // Consume the second '/'
                            self.update_position(next_ch);
                            // Skip single-line comment
                            while let Some(&comment_ch) = self.input.peek() {
                                if comment_ch == '\n' {
                                    break;
                                }
                                self.input.next();
                                self.update_position(comment_ch);
                            }
                            continue; // Restart the loop after comment.
                        } else if next_ch == '*' {
                            self.input.next(); // Consume '*' signaling multiline comment.
                            self.update_position(next_ch);
                            // Skip the entire multiline comment.
                            if !self.skip_multiline_comment() {
                                println!("Error: Unterminated multiline comment.");
                                // In a real compiler, you might return an Error token or panic.
                            }
                            continue; // Restart scanning tokens after the comment.
                        }
                    }
                    return Some((Token::Fslash, self.position.clone()));
                }
                '%' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::Percent, self.position.clone()));
                }
                '=' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::Equal, self.position.clone()));
                }
                '(' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::LPar, self.position.clone()));
                }
                ')' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::RPar, self.position.clone()));
                }
                '{' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::LCurl, self.position.clone()));
                }
                '}' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::RCurl, self.position.clone()));
                }
                ',' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::Comma, self.position.clone()));
                }
                ';' => {
                    self.input.next();
                    self.update_position(ch);
                    return Some((Token::Semicolon, self.position.clone()));
                }
                ':' => {
                    self.input.next();
                    let pos = self.position.clone();
                    self.update_position(ch);
                    if let Some(&next_ch) = self.input.peek() {
                        if next_ch == '=' {
                            self.input.next(); // Consume '='
                            self.update_position(next_ch);
                            return Some((Token::Walrus, pos));
                        }
                    }
                    return Some((Token::Colon, pos));
                }
                _ => {
                    self.input.next();
                    self.update_position(ch);
                    println!("Warning: Unknown token '{}'", ch);
                    return Some((Token::Unknown(ch), self.position.clone()));
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

        Token::NumberLiteral(num_str)
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

    fn string_literal(&mut self) -> Token {
        let mut literal = String::new();
        // Consume the opening double-quote.
        self.input.next();
        self.update_position('\"');
        
        while let Some(&ch) = self.input.peek() {
            if ch == '"' {
                self.input.next(); // Consume the closing quote.
                self.update_position(ch);
                break;
            } else if ch == '\\' {
                self.input.next();
                self.update_position('\\');
                if let Some(&esc_ch) = self.input.peek() {
                    let escaped_char = match esc_ch {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\\' => '\\',
                        '\'' => '\'',
                        other => other,
                    };
                    literal.push(escaped_char);
                    self.input.next();
                    self.update_position(esc_ch);
                }
            } else {
                literal.push(ch);
                self.input.next();
                self.update_position(ch);
            }
        }
        Token::StringLiteral(literal)
    }

    fn character_literal(&mut self) -> Token {
        // Consume the opening single-quote.
        self.input.next();
        self.update_position('\'');
        let mut char_val = None;
        if let Some(&ch) = self.input.peek() {
            if ch == '\\' {
                self.input.next();
                self.update_position('\\');
                if let Some(&esc_ch) = self.input.peek() {
                    let c = match esc_ch {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        other => other,
                    };
                    char_val = Some(c);
                    self.input.next();
                    self.update_position(esc_ch);
                }
            } else {
                char_val = Some(ch);
                self.input.next();
                self.update_position(ch);
            }
        }
        // Expect the closing single-quote.
        if let Some(&ch) = self.input.peek() {
            if ch == '\'' {
                self.input.next();
                self.update_position('\'');
            }
        }
        Token::CharacterLiteral(char_val.unwrap_or('\0').to_string())
    }

    pub fn lex(self) -> Vec<(Token, Position)> {
        let mut vec: Vec<(Token, Position)> = self.collect();
        vec.push((Token::Eof, Position { line: vec.last().unwrap().1.line + 1, index: 1 }));
        vec
    }
}
