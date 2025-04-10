use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::front::ast::Ast;
use crate::front::token::Token;

use super::nodes::function::{FunctionDefinition, FunctionBody, FunctionParam, FunctionReturnType, Return};
use super::nodes::id::Identifier;
use super::nodes::node::Node;

macro_rules! here {
    () => {
        println!("Execution passed through here:\n\tfile: {}\n\tline: {}", file!(), line!())
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    ids: HashMap<Arc<Mutex<usize>>, Box<dyn Node>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            position: 0,
            ids: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<Ast>, String> {
        let mut ast = Box::new(Ast::new());

        while let Some(token) = self.consume() {
            match token {
                Token::Fn => {
                    let func = self.parse_fn()?;
                    ast.push_child(Box::new(func)); // Add the parsed function to the AST
                }
                _ => {
                    // Skip unexpected tokens or handle other cases if needed
                }
            }
        }

        Ok(ast)
    }

    pub fn parse_fn(&mut self) -> Result<FunctionDefinition, String> {
        // Expect a function name
        let func_name = match self.consume() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err("Expected function name after 'fn'.".to_string()),
        };

        // Parse parameters
        let parameters = self.parse_fn_parameters()?;

        // Parse return type
        let return_type = self.parse_fn_return_type()?;

        // Parse the function body
        let body = self.parse_fn_body()?;

        Ok(FunctionDefinition {
            id: Identifier::from(func_name),
            parameters,
            return_type,
            body: Box::new(body),
        })
    }

    fn parse_fn_parameters(&mut self) -> Result<Vec<FunctionParam>, String> {
        let mut parameters = Vec::new();

        // Expect opening parenthesis
        if let Some(Token::LPar) = self.consume() {
            loop {
                match self.consume() {
                    Some(Token::RPar) => break,
                    Some(Token::Identifier(id)) => {
                        // Parse parameter identifier and type
                        if let Some(Token::Colon) = self.consume() {
                            if let Some(Token::Identifier(r#type)) = self.consume() {
                                parameters.push(FunctionParam {
                                    id: Identifier::from(id.clone()),
                                    r#type: r#type.clone(),
                                });
                            } else {
                                return Err("Expected type after ':'.".to_string());
                            }
                        } else {
                            return Err("Expected ':' after parameter name.".to_string());
                        }
                    }
                    Some(_) => return Err("Invalid token in function parameters.".to_string()),
                    None => return Err("Unexpected end of input in parameter list.".to_string()),
                }
            }
        } else {
            return Err("Expected '(' after function name.".to_string());
        }

        Ok(parameters)
    }

    fn parse_fn_return_type(&mut self) -> Result<FunctionReturnType, String> {
        let mut return_type = FunctionReturnType {
            r#type: "void".to_string(),
        };

        match self.consume() {
            Some(Token::Arrow) => {
                match self.consume() {
                    Some(Token::I32) => {
                        return_type.r#type = "i32".to_string();
                    }
                    _ => { todo!("parse_fn_return_type()") }
                }
            }
            Some(Token::Semicolon) => { return Ok(return_type); }
            Some(Token::LCurl) => { return Ok(return_type); }
            Some(_) => { todo!("parse_fn_return_type()") }
            None => { todo!("parse_fn_return_type()") }
        }

        return Ok(return_type);
    }

    fn parse_fn_body(&mut self) -> Result<FunctionBody, String> {
        let mut body = FunctionBody {
            children: Vec::new(),
        };

        if let Some(Token::LCurl) = self.current() {
            loop {
                match self.consume() {
                    Some(Token::RCurl) => break,
                    Some(_) => {
                        let statement = self.parse_statement()?;
                        body.children.push(statement);
                    }
                    None => return Err("Unexpected end of input in function body.".to_string()),
                }
            }
        }

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, String> {
        match self.consume() {
            Some(Token::Ret) => {
                return self.parse_return_statement();
            }
            None => {
                panic!("Expected a statement!");
            }
            _ => {
                dbg!(self.current().unwrap());
                todo!("parse_statement()")
            }
        }
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Node>, String> {
        match self.consume() {
            Some(Token::Number(num)) => {
                here!();
                match self.consume() {
                    Some(Token::Semicolon) => {
                        let ret = Return { value: num.clone() };
                        return Ok(Box::new(ret));
                    }
                    _ => { todo!("parse_return_statement()") }
                }
            }
            Some(Token::Semicolon) => {
                panic!("No value after 'ret' statement");
            }
            _ => {
                todo!("parse_return_statement()")
            }
        }
    }

    fn current(&self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        token
    }

    // Helper method to consume the current token and advance the position
    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }
}
