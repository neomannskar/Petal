#[derive(Clone, Debug, PartialEq)]
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
    CharacterLiteral(char),
    StringLiteral(String),

    Plus,
    Minus,
    Asterisk,
    Fslash,
    Percent,

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

#[derive(Clone, Debug, Default)]
pub struct Position {
    pub line: usize,
    pub index: usize,
}
