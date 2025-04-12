use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::front::ast::Ast;
use crate::front::token::Token;

use super::nodes::expr::{BinaryExpr, Expr, ExpressionStatement};
use super::nodes::function::{
    FunctionBody, FunctionDefinition, FunctionParameter, FunctionReturnType, Return,
};

use super::nodes::node::Node;
use super::nodes::operator::Operator;
use super::nodes::r#type::Type;
use super::nodes::variables::{
    Assignment, DeclarationAssignment, VariableDeclaration, WalrusDeclaration,
};
use super::semantic::{SemanticContext, Symbol};
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
}

impl Parser {
    pub fn new(file: String, tokens: Vec<(Token, Position)>) -> Self {
        Parser {
            file,
            tokens: tokens.to_vec(),
            position: 0,
        }
    }

    pub fn parse(&mut self, ctx: &mut SemanticContext) -> Result<Box<Ast>, ParserError> {
        let mut ast = Box::new(Ast::new());

        while let Ok((token, pos)) = self.consume() {
            match token {
                Token::Fn => {
                    match self.parse_fn(ctx) {
                        Ok(func) => {
                            ast.children.push(Box::new(func));
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                    // Add the parsed function to the AST
                }
                token => {
                    // Skip unexpected tokens or handle other cases
                    println!(
                        "Token: {:?} on line {} at index {}",
                        token, pos.line, pos.index
                    );
                    todo!("[token] parse()")
                }
            }
        }

        Ok(ast)
    }

    pub fn parse_fn<'a>(
        &mut self,
        ctx: &mut SemanticContext,
    ) -> Result<FunctionDefinition, ParserError> {
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

        let parameters = match self.parse_fn_parameters(ctx) {
            Ok(params) => params,
            Err(e) => {
                return Err(e);
            }
        };

        // Parse return type
        let return_type = match self.parse_fn_return_type() {
            Ok(ret) => ret,
            Err(e) => {
                // Change later
                return Err(e);
            }
        };

        // Parse the function body
        let body = match self.parse_fn_body(ctx) {
            Ok(bod) => bod,
            Err(e) => {
                // Change later
                return Err(e);
            }
        };

        Ok(FunctionDefinition {
            id: func_name,
            parameters,
            return_type,
            body: Box::new(body),
        })
    }

    fn parse_fn_parameters(
        &mut self,
        ctx: &mut SemanticContext,
    ) -> Result<Vec<FunctionParameter>, ParserError> {
        let mut parameters = Vec::new();

        // Expect an opening parenthesis.
        let (lpar, pos) = self.consume()?;
        if lpar != Token::LPar {
            return Err(ParserError::MissingToken {
                expected: "opening '('".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        // If immediately a right parenthesis, then there are no parameters.
        if let Some((Token::RPar, _)) = self.peek() {
            self.consume()?; // Consume the closing parenthesis.
            return Ok(parameters);
        }

        // Loop to parse one parameter at a time.
        loop {
            // Parse the parameter name.
            let (token, pos) = self.consume()?;
            let param_name = if let Token::Identifier(name) = token {
                name
            } else {
                return Err(ParserError::UnexpectedToken {
                    token,
                    file: self.file.clone(),
                    position: pos,
                });
            };

            // Expect a colon after the parameter name.
            let (colon, colon_pos) = self.consume()?;
            if colon != Token::Colon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ':' after parameter name.".to_string(),
                    file: self.file.clone(),
                    position: colon_pos,
                });
            }

            // Parse the parameter type.
            let (type_token, type_pos) = self.consume()?;
            let param_type = match type_token {
                Token::I32 => Type::basic("i32"),
                Token::I64 => Type::basic("i64"),
                Token::U32 => Type::basic("u32"),
                Token::U64 => Type::basic("u64"),
                // For types that are not built-in primitives,
                // we assume the token is an identifier (e.g. a struct name or type alias)
                Token::Identifier(id) => {
                    /*
                    match ctx.lookup(id) {
                        Some(t) => {
                            unreachable!()
                        }
                        None => { unreachable!() }
                    }
                    */

                    // Need to lookup the type to see if it exists

                    Type::Custom(id)
                }
                _ => {
                    return Err(ParserError::MissingToken {
                        expected: "parameter type".to_string(),
                        file: self.file.clone(),
                        position: type_pos,
                    });
                }
            };

            ctx.add_symbol(&param_name, Symbol::Variable(param_type.clone()));

            // Create the function parameter.
            parameters.push(FunctionParameter {
                id: param_name,
                r#type: param_type,
            });

            // Now, check if there is a comma or the close parenthesis.
            if let Some((next_token, _)) = self.peek() {
                match next_token {
                    Token::Comma => {
                        self.consume()?; // Consume the comma.
                                         // Continue to parse the next parameter.
                        continue;
                    }
                    Token::RPar => {
                        self.consume()?; // Consume the closing parenthesis.
                        break;
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken {
                            token: next_token,
                            file: self.file.clone(),
                            // Using a cloned current position (you might want to create a helper for this):
                            position: self
                                .tokens
                                .get(self.position)
                                .map(|(_, pos)| pos.clone())
                                .unwrap_or(pos),
                        });
                    }
                }
            } else {
                return Err(ParserError::MissingToken {
                    expected: "',' or ')'".to_string(),
                    file: self.file.clone(),
                    position: type_pos,
                });
            }
        }

        Ok(parameters)
    }

    fn parse_fn_return_type(&mut self) -> Result<FunctionReturnType, ParserError> {
        let mut return_type = FunctionReturnType(Type::basic("void"));

        match self.consume() {
            Ok((Token::Arrow, _)) => match self.consume() {
                Ok((Token::I32, _)) => {
                    return_type.0 = Type::basic("i32");
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

    fn parse_fn_body(&mut self, ctx: &mut SemanticContext) -> Result<FunctionBody, ParserError> {
        // Expect an opening curly brace and consume it.
        let (lcurly, pos) = self.consume()?;
        if lcurly != Token::LCurl {
            return Err(ParserError::MissingToken {
                expected: "opening '{'".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        let mut body = FunctionBody {
            children: Vec::new(),
        };

        // While the next token is not the closing curly, parse a statement.
        while let Some((token, _)) = self.peek() {
            if token == Token::RCurl {
                // End of function body reached.
                break;
            }
            let stmt = self.parse_statement(ctx)?; // parse_statement uses peek internally
            body.children.push(stmt);
        }

        // Now, expect and consume the closing curly.
        let (rcurly, pos) = self.consume()?;
        if rcurly != Token::RCurl {
            return Err(ParserError::MissingToken {
                expected: "closing '}'".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }
        Ok(body)
    }

    fn parse_fn_call(
        &mut self,
        ctx: &mut SemanticContext,
        function_id: String,
    ) -> Result<Expr, ParserError> {
        // Consume the left parenthesis. We already know the next token is LPar.
        let (lpar, pos) = self.consume()?;
        if lpar != Token::LPar {
            return Err(ParserError::SyntaxError {
                message: "Expected '(' after function name".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        let mut arguments = Vec::new();

        // If the next token is immediately a right parenthesis, then there are no arguments.
        if let Some((Token::RPar, _)) = self.peek() {
            self.consume()?; // Consume RPar
            return Ok(Expr::FunctionCall {
                function: function_id,
                arguments,
            });
        }

        // Otherwise, loop to parse arguments.
        loop {
            // Parse an expression argument.
            let arg = self.parse_expression(ctx)?;
            arguments.push(arg);

            // Peek at the next token to decide what to do.
            if let Some((next_token, pos)) = self.peek() {
                match next_token {
                    Token::Comma => {
                        self.consume()?; // Consume the comma and continue
                    }
                    Token::RPar => {
                        self.consume()?; // Consume the closing parenthesis and exit the loop.
                        break;
                    }
                    _ => {
                        return Err(ParserError::SyntaxError {
                            message: "Expected ',' or ')' in function call".to_string(),
                            file: self.file.clone(),
                            position: pos, // or better, use the position from peek
                        });
                    }
                }
            } else {
                return Err(ParserError::MissingToken {
                    expected: "',' or ')' in function call".to_string(),
                    file: self.file.clone(),
                    position: pos,
                });
            }
        }

        Ok(Expr::FunctionCall {
            function: function_id,
            arguments,
        })
    }

    // --- Expression Parsing Functions ---

    /// Parses an expression, handling addition and subtraction.
    fn parse_expression(&mut self, ctx: &mut SemanticContext) -> Result<Expr, ParserError> {
        let mut expr = self.parse_term(ctx)?;
        while let Some((token, _)) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    // Consume the operator.
                    let (op_token, _) = self.consume()?;
                    // Parse the right-hand side.
                    let right = self.parse_term(ctx)?;
                    let op = match op_token {
                        Token::Plus => Operator::Plus,
                        Token::Minus => Operator::Minus,
                        _ => unreachable!(),
                    };
                    expr = Expr::Binary(Box::new(BinaryExpr {
                        op,
                        left: expr,
                        right,
                    }));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parses a term, handling multiplication, division, and modulus.
    fn parse_term(&mut self, ctx: &mut SemanticContext) -> Result<Expr, ParserError> {
        let mut expr = self.parse_factor(ctx)?;
        while let Some((token, _)) = self.peek() {
            match token {
                Token::Asterisk | Token::Fslash | Token::Percent => {
                    let (op_token, _) = self.consume()?; // consume the operator
                    let right = self.parse_factor(ctx)?;
                    let op = match op_token {
                        Token::Asterisk => Operator::Asterisk,
                        Token::Fslash => Operator::Fslash,
                        Token::Percent => Operator::Percent,
                        _ => unreachable!(),
                    };
                    expr = Expr::Binary(Box::new(BinaryExpr {
                        op,
                        left: expr,
                        right,
                    }));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parses a factor: a number, an identifier, or a parenthesized expression.
    fn parse_factor(&mut self, ctx: &mut SemanticContext) -> Result<Expr, ParserError> {
        let (token, pos) = self.consume()?;
        match token {
            Token::NumberLiteral(num) => Ok(Expr::Number(num.parse::<i64>().unwrap())),
            Token::CharacterLiteral(ch) => Ok(Expr::Character(ch)),
            Token::StringLiteral(str) => Ok(Expr::String(str)),
            Token::Identifier(id) => {
                // If a left paren follows, this is a function call.
                if let Some((next_token, _)) = self.peek() {
                    if next_token == Token::LPar {
                        return self.parse_fn_call(ctx, id);
                    }
                }
                // Otherwise, it's a variable reference.

                dbg!(&ctx.symbol_table);

                match ctx.lookup(&id) {
                    Some(s) => {
                        Ok(Expr::VariableCall{ id, resolved: Some(s.clone()) } )
                    }
                    None => {
                        Ok(Expr::Identifier(id))
                    }
                }
            }
            Token::LPar => {
                let expr = self.parse_expression(ctx)?;
                match self.consume()? {
                    (Token::RPar, _) => Ok(expr),
                    (unexpected, pos) => Err(ParserError::UnexpectedToken {
                        token: unexpected,
                        file: self.file.clone(),
                        position: pos,
                    }),
                }
            }
            _ => Err(ParserError::UnexpectedToken {
                token,
                file: self.file.clone(),
                position: pos,
            }),
        }
    }

    fn parse_statement(&mut self, ctx: &mut SemanticContext) -> Result<Box<dyn Node>, ParserError> {
        // First, if the statement starts with 'ret', handle it.
        if let Some((Token::Ret, _)) = self.peek() {
            let (_, _) = self.consume()?; // Consume 'ret'
            let expr = self.parse_expression(ctx)?;
            let (next_token, next_pos) = self.consume()?;
            if next_token != Token::Semicolon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ';' after return expression.".to_string(),
                    file: self.file.clone(),
                    position: next_pos,
                });
            }
            return Ok(Box::new(Return { value: expr }));
        }

        // If the statement begins with an identifier, check the second token.
        if let Some((Token::Identifier(_), pos)) = self.peek() {
            let second = self.tokens.get(self.position + 1);
            if let Some((second_token, _)) = second {
                match second_token {
                    Token::Colon => {
                        // Call our declaration (or declaration-assignment) helper.
                        return self.parse_explicit_decl(ctx);
                    }
                    Token::Walrus => {
                        // You could add a dedicated helper for walrus declarations if desired.
                        return self.parse_walrus_decl(ctx);
                    }
                    Token::Equal => {
                        // If assignment appears without a preceding declaration, handle it.
                        return self.parse_assignment(ctx);
                    }
                    _ => {
                        // Fall back to parsing an expression statement.
                        let expr = self.parse_expression(ctx)?;
                        if let Some((Token::Semicolon, _)) = self.peek() {
                            self.consume()?; // consume semicolon.
                        }
                        return Ok(Box::new(ExpressionStatement { expression: expr }));
                    }
                }
            }
        }

        // If starting token is a number or left parenthesis, treat it as an expression.
        if let Some((Token::NumberLiteral(_) | Token::LPar, _)) = self.peek() {
            let expr = self.parse_expression(ctx)?;
            if let Some((Token::Semicolon, _)) = self.peek() {
                self.consume()?;
            }
            return Ok(Box::new(ExpressionStatement { expression: expr }));
        }

        // Otherwise, unexpected token.
        let (tok, pos) = self.consume()?;
        Err(ParserError::UnexpectedToken {
            token: tok,
            file: self.file.clone(),
            position: pos,
        })
    }

    fn parse_assignment(
        &mut self,
        ctx: &mut SemanticContext,
    ) -> Result<Box<dyn Node>, ParserError> {
        // Pattern: Identifier, Equal, Expression, Semicolon.

        // Consume the LHS identifier.
        let (id_token, _) = self.consume()?;
        let lhs = if let Token::Identifier(name) = id_token {
            name
        } else {
            unreachable!("Expected an identifier as the left-hand side of an assignment.")
        };

        // Consume the '=' token.
        let (equal, pos) = self.consume()?;
        if equal != Token::Equal {
            return Err(ParserError::SyntaxError {
                message: "Expected '=' in assignment statement.".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        // Parse the expression for the right-hand side.
        let expr = self.parse_expression(ctx)?;

        // Expect a terminating semicolon.
        let (semi, pos) = self.consume()?;
        if semi != Token::Semicolon {
            return Err(ParserError::SyntaxError {
                message: "Expected ';' after assignment.".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        // Build and return an Assignment node.
        Ok(Box::new(Assignment { lhs, value: expr }))
    }

    fn parse_explicit_decl(
        &mut self,
        ctx: &mut SemanticContext,
    ) -> Result<Box<dyn Node>, ParserError> {
        // Consume the identifier.
        let (id_token, _) = self.consume()?;
        let id = if let Token::Identifier(name) = id_token {
            name
        } else {
            return Err(ParserError::UnexpectedToken {
                token: id_token,
                file: self.file.clone(),
                position: self
                    .tokens
                    .get(self.position)
                    .map(|(_, pos)| pos.clone())
                    .unwrap_or_default(),
            });
        };

        // Expect a colon.
        let (colon, colon_pos) = self.consume()?;
        if colon != Token::Colon {
            return Err(ParserError::SyntaxError {
                message: "Expected ':' after identifier in variable declaration.".to_string(),
                file: self.file.clone(),
                position: colon_pos,
            });
        }

        // Parse the type.
        let (type_token, type_pos) = self.consume()?;
        let var_type = match type_token {
            Token::I32 => Type::basic("i32"),
            Token::Char => Type::basic("char"),
            Token::Str => Type::basic("str"),
            Token::Identifier(type_name) => Type::basic(type_name.as_str()),
            _ => {
                return Err(ParserError::MissingToken {
                    expected: "variable type".to_string(),
                    file: self.file.clone(),
                    position: type_pos,
                });
            }
        };

        match ctx.lookup(&id) {
            Some(s) => {
                return Err(ParserError::GenericError(String::from(format!("Id: `{}` is already defined as {:?}", id, s))))
            }
            None => { ctx.add_symbol(&id, Symbol::Variable(var_type.clone())) }
        }

        // At this point, we've parsed "<id> : <type>"
        // Check if the next token is an assignment operator.
        if let Some((Token::Equal, _)) = self.peek() {
            // Consume the '=' token.
            self.consume()?;
            // Parse initializer expression.
            let initializer_expr = self.parse_expression(ctx)?;
            // Expect a semicolon.
            let (semi, semi_pos) = self.consume()?;
            if semi != Token::Semicolon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ';' after declaration assignment.".to_string(),
                    file: self.file.clone(),
                    position: semi_pos,
                });
            }
            // Build the plain declaration (with no initializer)...
            let decl = VariableDeclaration {
                id: id.clone(),
                var_type: var_type.clone(),
            };
            // ...and an assignment node with lhs being the variable name.
            let assign = Assignment {
                lhs: id,
                value: initializer_expr,
            };
            // Combine them into a DeclarationAssignment node.
            Ok(Box::new(DeclarationAssignment {
                declaration: decl,
                assignment: assign,
            }))
        } else {
            // Otherwise, if there's no '=' token, this is a plain declaration.
            let (semi, semi_pos) = self.consume()?;
            if semi != Token::Semicolon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ';' after variable declaration.".to_string(),
                    file: self.file.clone(),
                    position: semi_pos,
                });
            }
            Ok(Box::new(VariableDeclaration {
                id: id,
                var_type,
            }))
        }
    }

    fn parse_walrus_decl(
        &mut self,
        ctx: &mut SemanticContext,
    ) -> Result<Box<dyn Node>, ParserError> {
        // Pattern: Identifier, Walrus, Expression, Semicolon.
        let (id_token, _) = self.consume()?; // Identifier
        let id = if let Token::Identifier(name) = id_token {
            name
        } else {
            unreachable!()
        };

        let (walrus, pos) = self.consume()?; // Expect the walrus operator (":=")
        if walrus != Token::Walrus {
            return Err(ParserError::SyntaxError {
                message: "Expected ':=' after identifier for walrus declaration.".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        // Parse the initializer expression.
        let expr = self.parse_expression(ctx)?;

        // Expect semicolon.
        let (semi, pos) = self.consume()?;
        if semi != Token::Semicolon {
            return Err(ParserError::SyntaxError {
                message: "Expected ';' after walrus declaration.".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        ctx.add_symbol(&id, Symbol::Variable(Type::Custom(String::from("<inferred>"))));

        Ok(Box::new(WalrusDeclaration {
            id: id,
            initializer: expr,
        }))
    }

    fn peek(&self) -> Option<(Token, Position)> {
        self.tokens.get(self.position).cloned()
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
