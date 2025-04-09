#[derive(Debug, PartialEq)]
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

    Number(String), // Stores both integers and floats as strings

    Plus,
    Minus,
    Asterisk,
    FSlash,

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

    Comma,
    Semicolon,
    Colon,
}
