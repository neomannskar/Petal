use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::front::ast::Ast;
use crate::front::token::Token;

use super::nodes::function::{
    FunctionBody, FunctionDefinition, FunctionParameter, FunctionReturnType, Return,
};
use super::nodes::id::Identifier;
use super::nodes::node::Node;
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

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken {
        token: Token,
        file: String,
        position: Position,
    },
    MissingToken {
        expected: String,
        file: String,
        position: Position,
    },
    SyntaxError {
        message: String,
        file: String,
        position: Position,
    },
    InvalidParameter {
        message: String,
        file: String,
        position: Position,
    },
    GenericError(String),
}

use std::fmt;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken {
                token,
                file,
                position,
            } => {
                write!(
                    f,
                    "Unexpected token '{:?}' in file: {} on line {} at position {}",
                    token, file, position.line, position.index
                )
            }
            ParserError::MissingToken {
                expected,
                file,
                position,
            } => {
                write!(
                    f,
                    "Missing token '{}', expected in file: {} on line {} at position {}",
                    expected, file, position.line, position.index
                )
            }
            ParserError::SyntaxError {
                message,
                file,
                position,
            } => {
                write!(
                    f,
                    "Syntax error in file {} on line {} at position {}: {}",
                    file, position.line, position.line, message
                )
            }
            ParserError::InvalidParameter {
                message,
                file,
                position,
            } => {
                write!(
                    f,
                    "Invalid parameter: {} in file {} on line {} at position {}",
                    message, file, position.line, position.index
                )
            }
            ParserError::GenericError(message) => {
                write!(f, "Error: {}", message)
            }
        }
    }
}

pub struct Parser {
    file: String,
    tokens: Vec<(Token, Position)>,
    position: usize,
    ids: HashMap<Arc<Mutex<usize>>, Box<dyn Node>>,
}

impl Parser {
    pub fn new(file: String, tokens: Vec<(Token, Position)>) -> Self {
        Parser {
            file,
            tokens: tokens.to_vec(),
            position: 0,
            ids: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<Ast>, ParserError> {
        let mut ast = Box::new(Ast::new());

        while let Ok((token, pos)) = self.consume() {
            match token {
                Token::Fn => {
                    match self.parse_fn() {
                        Ok(func, ) => {
                            ast.push_child(Box::new(func));
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                    // Add the parsed function to the AST
                }
                _ => {
                    // Skip unexpected tokens or handle other cases
                    println!("Position: {} : {}", pos.line, pos.index);
                    todo!("[_] parse()")
                }
            }
        }

        Ok(ast)
    }

    pub fn parse_fn<'a>(&mut self) -> Result<FunctionDefinition, ParserError> {
        // Expect a function name
        let func_name = match self.consume() {
            Ok((Token::Identifier(name), _)) => name.clone(),
            Ok((token, pos)) => {
                return Err(ParserError::UnexpectedToken {
                    token: token,
                    file: self.file.clone(),
                    position: pos,
                })
            }
            Err(e) => return Err(e),
        };

        let mut parameters = Vec::new();

        // Remove the 'unwrap()' later in place of better error handling
        if self.consume().unwrap().0 == Token::LPar {
            if self.current().unwrap().0 != Token::RPar {
                let start = self.position;
                let mut end: usize = start;

                while self.tokens.get(start).unwrap().0 != Token::RPar {
                    end += 1;
                }
                end -= 1;

                self.parse_fn_parameters();
            } else {
                self.consume();
            }
        } else {
            dbg!(self.current());
            todo!("FUTURE SYNTAX: < for <> and [ for [] ")
        }

        // Parse parameters
        // let parameters = self.parse_fn_parameters().unwrap();

        // Parse return type
        let return_type = match self.parse_fn_return_type() {
            Ok(ret) => ret,
            Err(e) => {
                // Change later
                return Err(e);
            }
        };

        // Parse the function body
        let body = self.parse_fn_body().unwrap();

        Ok(FunctionDefinition {
            id: Identifier::from(func_name),
            parameters,
            return_type,
            body: Box::new(body),
        })
    }

    fn parse_fn_parameters(&mut self) -> Result<Vec<FunctionParameter>, ParserError> {
        let mut parameters = Vec::new();

        match self.consume()? {
            (Token::LPar, pos) => {
                loop {
                    match self.consume()? {
                        (Token::RPar, pos) => break, // Closing parenthesis marks end of parameters
                        (Token::Identifier(id), pos) => {
                            if let (Token::Colon, pos) = self.consume()? {
                                if let (Token::Identifier(type_name), pos) = self.consume()? {
                                    parameters.push(FunctionParameter {
                                        id: Identifier::from(id.clone()),
                                        r#type: type_name.clone(),
                                    });
                                } else {
                                    return Err(ParserError::MissingToken {
                                        expected: "parameter type".to_string(),
                                        file: self.file.clone(),
                                        position: pos,
                                    });
                                }
                            } else {
                                return Err(ParserError::SyntaxError {
                                    message: "Expected ':' after parameter name.".to_string(),
                                    file: self.file.clone(),
                                    position: pos,
                                });
                            }
                        }
                        (token, pos) => {
                            return Err(ParserError::UnexpectedToken {
                                token,
                                file: self.file.clone(),
                                position: pos,
                            });
                        }
                    }
                }
            }
            (_, pos) => {
                return Err(ParserError::MissingToken {
                    expected: "opening parenthesis '('".to_string(),
                    file: self.file.clone(),
                    position: pos,
                });
            }
        }

        Ok(parameters)
    }

    fn parse_fn_return_type(&mut self) -> Result<FunctionReturnType, ParserError> {
        let mut return_type = FunctionReturnType {
            r#type: "void".to_string(),
        };

        match self.consume() {
            Ok((Token::Arrow, _)) => match self.consume() {
                Ok((Token::I32, _)) => {
                    return_type.r#type = "i32".to_string();
                }
                x => {
                    dbg!(x);
                    todo!("[x] parse_fn_return_type()");
                }
            },
            Ok((Token::Semicolon, _)) => {
                return Ok(return_type);
            }
            Ok((Token::LCurl, _)) => {
                return Ok(return_type);
            }
            Ok((token, _)) => {
                dbg!(token);
                todo!("[Some(x)] parse_fn_return_type()")
            }
            Err(e) => {
                println!("{:?}", e);
                /* return Err(ParserError::MissingToken {
                    expected: String::from("'->' or '{' or ';'"),
                    file: self.file.clone(),
                    position: pos,
                }); */

                return Err(e);
            }
        }

        return Ok(return_type);
    }

    fn parse_fn_body(&mut self) -> Result<FunctionBody, ParserError> {
        let mut body = FunctionBody {
            children: Vec::new(),
        };

        if let Ok((Token::LCurl, _)) = self.current() {
            loop {
                match self.consume() {
                    Ok((Token::RCurl, _)) => break,
                    Ok(_) => {
                        let statement = self.parse_statement()?;
                        body.children.push(statement);
                    }
                    Err(e) => {
                        return Err(ParserError::GenericError(String::from(
                            "Unexpected end of input in function body.",
                        )))
                    }
                }
            }
        }

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, ParserError> {
        match self.consume()? {
            (Token::Ret, pos) => {
                if let (Token::Number(num), pos) = self.consume()? {
                    if let (Token::Semicolon, pos) = self.consume()? {
                        return Ok(Box::new(Return { value: num.clone() }));
                    } else {
                        return Err(ParserError::SyntaxError {
                            message: "Expected ';' after return value.".to_string(),
                            file: self.file.clone(),
                            position: pos,
                        });
                    }
                } else {
                    return Err(ParserError::MissingToken {
                        expected: "return value".to_string(),
                        file: self.file.clone(),
                        position: pos,
                    });
                }
            }
            (token, pos) => Err(ParserError::UnexpectedToken {
                token,
                file: self.file.clone(),
                position: pos,
            }),
        }
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Node>, ParserError> {
        match self.consume()? {
            (Token::Number(num), _) => match self.consume()? {
                (Token::Semicolon, _) => {
                    let ret = Return { value: num.clone() };
                    return Ok(Box::new(ret));
                }
                _ => {
                    todo!("parse_return_statement()")
                }
            },
            (Token::Semicolon, _) => {
                panic!("No value after 'ret' statement");
            }
            _ => {
                todo!("parse_return_statement()")
            }
        }
    }

    fn expect(&self, t: Token) -> Result<bool, ParserError> {
        if let Some(tok) = self.tokens.get(self.position + 1) {
            if tok.0 == t {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(ParserError::GenericError(
                "End of program reached (no more tokens)".to_string(),
            ))
        }
    }

    fn current(&self) -> Result<(Token, Position), ParserError> {
        if let Some((token, pos)) = self.tokens.get(self.position).cloned() {
            match token {
                Token::Eof => Err(ParserError::UnexpectedToken {
                    token: Token::Eof,
                    file: self.file.clone(),
                    position: pos.clone(),
                }),
                _ => Ok((token, pos)),
            }
        } else {
            Err(ParserError::GenericError(String::from("Reached end of Vec<(Token, Position)> for unknown reason, it should have stopped at `Token::Eof`")))
        }
    }

    // Helper method to consume the current token and advance the position
    fn consume(&mut self) -> Result<(Token, Position), ParserError> {
        if let Some((token, pos)) = self.tokens.get(self.position).cloned() {
            match token {
                Token::Eof => Err(ParserError::UnexpectedToken {
                    token: Token::Eof,
                    file: self.file.clone(),
                    position: pos.clone(),
                }),
                _ => {
                    self.position += 1;
                    Ok((token, pos))
                }
            }
        } else {
            Err(ParserError::GenericError(String::from("Reached end of Vec<(Token, Position)> for unknown reason, it should have stopped at `Token::Eof`")))
        }
    }
}
