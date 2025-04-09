use std::collections::HashMap;
// front/parser.rs
use std::iter::Peekable;
use std::slice::Iter;
use std::sync::{Arc, Mutex};

use crate::front::token::Token;
use crate::front::ast::Ast;

use crate::front::nodes::{expr::Expr, operator::Operator};
use super::nodes::function::{Function, FunctionBody, FunctionParam};
use super::nodes::node::Node;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,

    ids: HashMap<Arc<Mutex<usize>>, Box<dyn Node>>,

    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    // Create a new parser from a slice of tokens.
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            errors: Vec::new(),
        }
    }

    // Entry point
    pub fn parse(&mut self) -> Result<Box<Ast>, String> {
        let mut ast: Box<Ast> = Box::new(Ast::new());

        while let Some(&token) = self.tokens.peek() {
            match token {
                Token::Fn => {
                    self.tokens.next();
                    match self.parse_fn() {
                        Ok(Some(func)) => {
                            ast.push_child(Box::new(func));
                        }
                        Ok(None) => {
                            self.errors.push("Function parsing returned None unexpectedly.".to_string());
                        }
                        Err(e) => {
                            self.errors.push(format!("Error parsing function: {}", e));
                        }
                    }
                }
                _ => {
                    self.tokens.next();
                    // Handle other tokens or log unexpected token
                }
            }
        }

        Ok(ast)
    }

    pub fn parse_fn(&mut self) -> Result<Option<Function>, String> {
        let func_name = match self.tokens.next() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err("Expected function name after 'fn' keyword.".to_string()),
        };
    
        // Step 2: Parse the function parameters
        let parameters = self.parse_fn_parameters()?;
    
        // Step 3: Parse the function body
        let body = self.parse_fn_body()?;
    
        // Step 4: Construct the Function object
        let function = Function {
            name: func_name,
            parameters,
            body,
        };
    
        Ok(Some(function))
    }

    pub fn parse_fn_parameters(&mut self) -> Result<Option<Vec<FunctionParam>>, String> {
        Ok(None)
    }

    pub fn parse_fn_body(&mut self) -> Result<Option<FunctionBody>, String> {
        Ok(None)
    }
}
