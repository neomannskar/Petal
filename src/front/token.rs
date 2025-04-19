#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown(char),
    Eof,

    Identifier(String),

    Fn,
    Ret,
    Struct,
    Pub,
    Enum,
    Impl,

    If,
    Else,
    For,
    While,

    NumberLiteral(String), // Stores both integers and floats as strings
    CharacterLiteral(String),
    StringLiteral(String),

    Plus,
    Minus,
    Asterisk,
    Fslash,
    Percent,
    Ampersand,
    Pipe,

    And,
    Or,
    Not,
    Compare,

    Equal,
    Walrus,

    LPar,
    RPar,
    LCurl,
    RCurl,

    Arrow,

    I32,
    I64,
    U32,
    U64,
    Usize,
    F32,
    F64,
    Char,
    Str,

    Comma,
    Semicolon,
    Colon,
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    pub line: usize,
    pub index: usize,
}
