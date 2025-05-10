use crate::front::ast::Ast;
use crate::front::token::Token;

use super::nodes::body::Body;
use super::nodes::cast::Cast;
use super::nodes::loops::{Break, Continue, ForLoop, Loop, WhileLoop};
use super::nodes::parameter::Parameter;
use super::nodes::expr::{BinaryExpr, Expr, ExpressionStatement};
use super::nodes::function::{
    FunctionDefinition, FunctionReturnType, Return,
};

use super::nodes::node::Node;
use super::nodes::operator::Operator;
use super::nodes::r#type::{PrimitiveType, Type};
use super::nodes::variables::{
    Assignment, DeclarationAssignment, VariableDeclaration, WalrusDeclaration,
};
use super::nodes::branching::{ElseStatement, IfStatement};
use super::token::Position;

macro_rules! _here {
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

    pub fn parse(&mut self) -> Result<Box<Ast>, ParserError> {
        let mut ast = Box::new(Ast::new());

        while let Ok((token, pos)) = self.consume() {
            match token {
                Token::Fn => {
                    match self.parse_fn() {
                        Ok((func, pos)) => {
                            ast.children.push((Box::new(func), pos));
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

    pub fn parse_fn<'a>(&mut self) -> Result<(FunctionDefinition, Position), ParserError> {
        // Expect a function name
        let (func_name, func_pos) = match self.consume() {
            Ok((Token::Identifier(name), pos)) => (name.clone(), pos.clone()),
            Ok((token, pos)) => {
                return Err(ParserError::UnexpectedToken {
                    token: token,
                    file: self.file.clone(),
                    position: pos,
                })
            }
            Err(e) => return Err(e),
        };

        let parameters = match self.parse_fn_parameters() {
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
        let (body, body_pos) = match self.parse_body() {
            Ok(bod) => bod,
            Err(e) => {
                // Change later
                return Err(e);
            }
        };

        Ok((
            FunctionDefinition {
                id: (func_name, func_pos.clone()),
                parameters,
                return_type: return_type,
                body: (Box::new(body), body_pos),
            },
            func_pos,
        ))
    }

    fn parse_fn_parameters(&mut self) -> Result<Vec<(Parameter, Position)>, ParserError> {
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

            // Create the function parameter.
            parameters.push((
                Parameter {
                    id: param_name,
                    var_type: param_type,
                    position: pos.clone(),
                },
                pos.clone(),
            ));

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

    fn parse_fn_return_type(&mut self) -> Result<(FunctionReturnType, Position), ParserError> {
        match self.consume() {
            Ok((Token::Arrow, _)) => match self.consume() {
                Ok((Token::I32, pos)) => Ok((FunctionReturnType(Type::basic("i32")), pos)),
                Ok((Token::Char, pos)) => Ok((FunctionReturnType(Type::basic("char")), pos)),
                Ok((Token::Str, pos)) => Ok((FunctionReturnType(Type::basic("str")), pos)),
                Ok(x) => {
                    dbg!(x);
                    todo!("[x] parse_fn_return_type()");
                }
                _ => todo!("[_] parse_fn_return_type()"),
            },
            Ok((Token::Semicolon, pos)) => {
                return Ok((FunctionReturnType(Type::basic("void")), pos));
            }
            Ok((Token::LCurl, pos)) => {
                return Ok((FunctionReturnType(Type::basic("void")), pos));
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
    }

    fn parse_body(&mut self) -> Result<(Body, Position), ParserError> {
        // Expect an opening curly brace and consume it.
        let (lcurly, pos) = self.consume()?;
        if lcurly != Token::LCurl {
            return Err(ParserError::MissingToken {
                expected: "opening '{'".to_string(),
                file: self.file.clone(),
                position: pos,
            });
        }

        let mut body = Body {
            children: Vec::new(),
        };

        // While the next token is not the closing curly, parse a statement.
        while let Some((token, _)) = self.peek() {
            if token == Token::RCurl {
                // End of function body reached.
                break;
            }
            let (stmt, pos) = self.parse_statement()?; // parse_statement uses peek internally
            body.children.push((stmt, pos));
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
        Ok((body, pos))
    }

    fn parse_fn_call(
        &mut self,
        function_id: (String, Position),
    ) -> Result<(Expr, Position), ParserError> {
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
        if let Some((Token::RPar, pos)) = self.peek() {
            self.consume()?; // Consume RPar
            return Ok((
                Expr::FunctionCall {
                    function: function_id,
                    arguments,
                },
                pos,
            ));
        }

        // Otherwise, loop to parse arguments.
        loop {
            // Parse an expression argument.
            let (arg, pos) = self.parse_cast()?;
            arguments.push((arg, pos.clone()));

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

        Ok((
            Expr::FunctionCall {
                function: function_id,
                arguments,
            },
            pos,
        ))
    }

    fn parse_type(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        if let Some((tok, tok_pos)) = self.peek() {
            match tok {
                Token::Identifier(id) => {
                    return Ok((Box::new(Type::Custom(id)), tok_pos))
                }
                Token::Bool =>  return Ok((Box::new(Type::Primitive(PrimitiveType::Bool)), tok_pos)),
                Token::I32 =>   return Ok((Box::new(Type::Primitive(PrimitiveType::I32)), tok_pos)),
                Token::I64 =>   return Ok((Box::new(Type::Primitive(PrimitiveType::I64)), tok_pos)),
                Token::U32 =>   return Ok((Box::new(Type::Primitive(PrimitiveType::U32)), tok_pos)),
                Token::U64 =>   return Ok((Box::new(Type::Primitive(PrimitiveType::U64)), tok_pos)),
                Token::Usize => return Ok((Box::new(Type::Primitive(PrimitiveType::Usize)), tok_pos)),
                Token::F32 =>   return Ok((Box::new(Type::Primitive(PrimitiveType::F32)), tok_pos)),
                Token::F64 =>   return Ok((Box::new(Type::Primitive(PrimitiveType::F64)), tok_pos)),
                Token::Char =>  return Ok((Box::new(Type::Primitive(PrimitiveType::Char)), tok_pos)),
                Token::Str =>   return Ok((Box::new(Type::Primitive(PrimitiveType::Str)), tok_pos)),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }

    fn parse_type_as_string(&mut self) -> Result<String, ParserError> {
        if let Some((tok, tok_pos)) = self.peek() {
            let t = match tok {
                Token::Identifier(id) => id.clone(),
                Token::Bool =>  "bool".to_string(),
                Token::I32 =>   "i32".to_string(),
                Token::I64 =>   "i64".to_string(),
                Token::U32 =>   "u32".to_string(),
                Token::U64 =>   "u64".to_string(),
                Token::Usize => "usize".to_string(),
                Token::F32 =>   "f32".to_string(),
                Token::F64 =>   "f64".to_string(),
                Token::Char =>  "char".to_string(),
                Token::Str =>   "str".to_string(),
                _ => todo!(),
            };

            let (_, _) = self.consume()?;

            Ok(t)
        } else {
            todo!()
        }
    }

    // --- Expression Parsing Functions ---

    fn parse_cast(&mut self) -> Result<(Expr, Position), ParserError> {
        // 1) Parse everything up to (but not including) an `as`
        let (expr, pos) = self.parse_expression()?;

        // 2) If the next token is `as`, consume it and parse a type name
        if let Some((Token::As, _)) = self.peek() {
            // consume the `as` keyword
            let (_, as_pos) = self.consume()?;

            // parse_type returns a String, e.g. "i32" or "MyStruct"
            let type_name = self.parse_type_as_string()?;

            const PRIMITIVES: &[&str] = &[
                "u8", "u16", "u32", "u64", "u128",
                "i8", "i16", "i32", "i64", "i128",
                "f32", "f64",
                "bool", "char", "str",
            ];

            // 3) Decide primitive vs non‐primitive
            let cast_to = if PRIMITIVES.contains(&type_name.as_str()) {
                Cast::Primitive((type_name, as_pos))
            } else {
                Cast::NonPrimitive((type_name, as_pos))
            };

            let cast_expr = Expr::Cast((cast_to, Box::new(expr)), pos.clone());

            Ok((cast_expr, pos))
        } else {
            // no `as` → just return the original expression
            Ok((expr, pos))
        }
    }

    /// Parses an expression, handling addition and subtraction.
    fn parse_expression(&mut self) -> Result<(Expr, Position), ParserError> {
        let (mut expr, pos) = self.parse_term()?;
        while let Some((token, _)) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    // Consume the operator.
                    let (op_token, _) = self.consume()?;
                    // Parse the right-hand side.
                    let (right, right_pos) = self.parse_term()?;
                    let op = match op_token {
                        Token::Plus => Operator::Plus,
                        Token::Minus => Operator::Minus,
                        _ => unreachable!(),
                    };
                    expr = Expr::Binary((
                        Box::new(BinaryExpr {
                            op,
                            left: (expr, pos.clone()),
                            right: (right, right_pos),
                        }),
                        pos.clone(),
                    ));
                }
                _ => break,
            }
        }
        Ok((expr, pos))
    }

    /// Parses a term, handling multiplication, division, and modulus.
    fn parse_term(&mut self) -> Result<(Expr, Position), ParserError> {
        let (mut expr, expr_pos) = self.parse_factor()?;
        while let Some((token, _)) = self.peek() {
            match token {
                Token::Asterisk | Token::Fslash | Token::Percent => {
                    let (op_token, _op_pos) = self.consume()?; // capture the operator (its position is available if needed)
                    let (right, right_pos) = self.parse_factor()?;
                    let op = match op_token {
                        Token::Asterisk => Operator::Asterisk,
                        Token::Fslash => Operator::Fslash,
                        Token::Percent => Operator::Percent,
                        _ => unreachable!(),
                    };

                    // Since Position only marks the start in your setup,
                    // we simply consistently use expr_pos for the expression's position.
                    expr = Expr::Binary((
                        Box::new(BinaryExpr {
                            left: (expr, expr_pos.clone()),
                            op,
                            right: (right, right_pos),
                        }),
                        expr_pos.clone(),
                    ));
                }
                _ => break,
            }
        }
        Ok((expr, expr_pos))
    }

    /// Parses a factor: a number, an identifier, or a parenthesized expression.
    fn parse_factor(&mut self) -> Result<(Expr, Position), ParserError> {
        let (token, pos) = self.consume()?;
        match token {
            Token::BooleanLiteral(bool) => Ok((Expr::Boolean((bool, pos.clone())), pos)),
            Token::NumberLiteral(num) => Ok((Expr::Number((num, pos.clone())), pos)),
            Token::CharacterLiteral(ch) => Ok((Expr::Character((ch, pos.clone())), pos)),
            Token::StringLiteral(str) => Ok((Expr::String((str, pos.clone())), pos)),
            Token::Identifier(id) => {
                // If a left paren follows, this is a function call.
                if let Some((next_token, _)) = self.peek() {
                    if next_token == Token::LPar {
                        return self.parse_fn_call((id, pos));
                    }
                }
                // Otherwise, it's a variable reference.

                Ok((Expr::VariableCall((id, pos.clone())), pos))
            }
            Token::LPar => {
                let expr = self.parse_cast()?;
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

    fn parse_if_statement(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        let (_, if_pos) = self.consume()?; // Consume 'if'

        // Parse the condition (expression)

        let (expr, expr_pos) = self.parse_cast()?;
        
        /* match expr {
            Expr::Number((num, pos)) => {
                println!("Number here: {} at {:?}", num, pos);
                panic!("");
            },
            Expr::Character(_) => todo!(),
            Expr::String(_) => todo!(),
            Expr::Binary(binary_expr) => todo!(),
            Expr::VariableCall((id, resolved)) => todo!(),
            Expr::FunctionCall { function, arguments } => todo!(),
        } */

        let (body, body_pos) = self.parse_body()?;

        return Ok((Box::new(IfStatement { condition: (expr, expr_pos), branch: (body, body_pos) }), if_pos))
    }

    fn parse_else_statement(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        let (_, else_pos) = self.consume()?; // Consume 'else'

        // Parse the condition (expression)

        if let Some((Token::If, _)) = self.peek() {
            let (_, else_pos) = self.consume()?; // Consume 'if'

            let (expr, expr_pos) = self.parse_cast()?;
        
            let (body, body_pos) = self.parse_body()?;

            return Ok((Box::new(ElseStatement { condition: Some((expr, expr_pos)), branch: (body, body_pos) }), else_pos));
        } else {
            let (body, body_pos) = self.parse_body()?;
            
            return Ok((Box::new(ElseStatement { condition: None, branch: (body, body_pos) }), else_pos));
        }

        /* match expr {
            Expr::Number((num, pos)) => {
                println!("Number here: {} at {:?}", num, pos);
                panic!("");
            },
            Expr::Character(_) => todo!(),
            Expr::String(_) => todo!(),
            Expr::Binary(binary_expr) => todo!(),
            Expr::VariableCall((id, resolved)) => todo!(),
            Expr::FunctionCall { function, arguments } => todo!(),
        } */
    }

    fn parse_loop(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        let (_, loop_pos) = self.consume()?;

        let (body, body_pos) = self.parse_body()?;

        return Ok((Box::new(Loop { body: (body, body_pos) }), loop_pos));
    }

    fn parse_while_loop(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        let (_, while_loop_pos) = self.consume()?;

        let (expr, expr_pos) = self.parse_cast()?;

        let (body, body_pos) = self.parse_body()?;

        return Ok((Box::new(WhileLoop {
            condition: (expr, expr_pos),
            body: (body, body_pos),
        }), while_loop_pos));
    }

    fn parse_for_loop(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        todo!();
        
        /* 
        let (_, for_loop_pos) = self.consume()?;

        let (iter, iter_pos) = self.parse_explicit_decl()?;

        let (expr, expr_pos) = self.parse_cast()?;

        let (body, body_pos) = self.parse_body()?;

        return Ok((Box::new(ForLoop {
            iter: (iter, iter_pos),
            condition: (expr, expr_pos),
            body: (body, body_pos),
        }), while_loop_pos));
        */
    }

    fn parse_let_decl(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        let (_, let_pos) = self.consume()?; // Consume the 'let'
        
        if let Some((Token::Identifier(id), id_pos)) = self.peek(){
            if let Some((next, next_pos)) = self.tokens.get(self.position + 1) {
                match next {
                    Token::Colon => {
                        // Call our declaration (or declaration-assignment) helper.
                        return self.parse_explicit_decl();
                    }
                    Token::Walrus => {
                        // You could add a dedicated helper for walrus declarations if desired.
                        return self.parse_walrus_decl();
                    }
                    _ => {
                        todo!();
                    }
                }
            }
        }

        let (tok, pos) = self.consume()?;
        Err(ParserError::UnexpectedToken {
            token: tok,
            file: self.file.clone(),
            position: pos,
        })
    }

    fn parse_statement(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        // First, if the statement starts with 'ret', handle it.
        if let Some((Token::Ret, ret_pos)) = self.peek() {
            let (_, _) = self.consume()?; // Consume 'ret'
            let (expr, expr_pos) = self.parse_cast()?;
            // dbg!(&expr);
            let (next_token, next_pos) = self.consume()?;
            // dbg!(&next_token);
            if next_token != Token::Semicolon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ';' after return expression.".to_string(),
                    file: self.file.clone(),
                    position: next_pos,
                });
            }
            return Ok((
                Box::new(Return {
                    value: (expr, expr_pos),
                }),
                ret_pos,
            ));
        }

        if let Some((Token::If, _)) = self.peek() {
            let (if_stat, pos) = self.parse_if_statement()?;
            return Ok((if_stat, pos));
        }

        if let Some((Token::Else, _)) = self.peek() {
            let (else_stat, pos) = self.parse_else_statement()?;
            return Ok((else_stat, pos));
        }

        if let Some((Token::Loop, _)) = self.peek() {
            let (loop_stat, pos) = self.parse_loop()?;
            return Ok((loop_stat, pos));
        }

        if let Some((Token::While, _)) = self.peek() {
            let (while_loop, pos) = self.parse_while_loop()?;
            return Ok((while_loop, pos));
        }

        if let Some((Token::For, _)) = self.peek() {
            let (for_loop, pos) = self.parse_for_loop()?;
            return Ok((for_loop, pos));
        }

        if let Some((Token::Break, pos)) = self.peek() {
            let (_, _) = self.consume()?; // Consume 'ret'
            let (next_token, next_pos) = self.consume()?;
            if next_token != Token::Semicolon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ';' after 'break.".to_string(),
                    file: self.file.clone(),
                    position: next_pos,
                });
            }
            return Ok((Box::new(
                    Break
                ),
                pos,
            ));
        }
        
        if let Some((Token::Continue, pos)) = self.peek() {
            let (_, _) = self.consume()?; // Consume 'break'
            let (next_token, next_pos) = self.consume()?;
            if next_token != Token::Semicolon {
                return Err(ParserError::SyntaxError {
                    message: "Expected ';' after 'continue'.".to_string(),
                    file: self.file.clone(),
                    position: next_pos,
                });
            }
            return Ok((Box::new(
                    Continue
                ),
                pos,
            ));
        }

        if let Some((Token::Let, _)) = self.peek() {
            return self.parse_let_decl();
        }

        // If the statement begins with an identifier, check the second token.
        if let Some((Token::Identifier(_), _)) = self.peek() {
            let second = self.tokens.get(self.position + 1);
            if let Some((second_token, _)) = second {
                match second_token {
                    Token::Equal => {
                        // If assignment appears without a preceding declaration, handle it.
                        return self.parse_assignment();
                    }
                    _ => {
                        // Fall back to parsing an expression statement.
                        let (expr, pos) = self.parse_cast()?;
                        if let Some((Token::Semicolon, _)) = self.peek() {
                            self.consume()?; // consume semicolon.
                        }
                        return Ok((Box::new(ExpressionStatement { expression: expr }), pos));
                    }
                }
            }
        }

        // If starting token is a number or left parenthesis, treat it as an expression.
        if let Some((Token::NumberLiteral(_) | Token::LPar, _)) = self.peek() {
            let (expr, pos) = self.parse_cast()?;
            if let Some((Token::Semicolon, _)) = self.peek() {
                self.consume()?;
            }
            return Ok((Box::new(ExpressionStatement { expression: expr }), pos));
        }

        // Otherwise, unexpected token.
        let (tok, pos) = self.consume()?;
        Err(ParserError::UnexpectedToken {
            token: tok,
            file: self.file.clone(),
            position: pos,
        })
    }

    fn parse_assignment(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        // Pattern: Identifier, Equal, Expression, Semicolon.

        // Consume the LHS identifier.
        let (id_token, _) = self.consume()?;
        let lhs = if let Token::Identifier(name) = id_token {
            name
        } else {
            unreachable!("Expected an identifier as the left-hand side of an assignment.")
        };

        // Consume the '=' token.
        let (_, _) = self.consume()?;

        // Parse the expression for the right-hand side.
        let (expr, pos) = self.parse_cast()?;

        // Expect a terminating semicolon.
        let (semi, semi_pos) = self.consume()?;
        if semi != Token::Semicolon {
            return Err(ParserError::SyntaxError {
                message: "Expected ';' after assignment.".to_string(),
                file: self.file.clone(),
                position: semi_pos,
            });
        }

        // Build and return an Assignment node.
        Ok((
            Box::new(Assignment {
                lhs,
                value: (expr, pos.clone()),
            }),
            pos,
        ))
    }

    fn parse_explicit_decl(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        // Consume the identifier.
        let (id_token, id_pos) = self.consume()?;
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

        // At this point, we've parsed "<id> : <type>"
        // Check if the next token is an assignment operator.
        if let Some((Token::Equal, _)) = self.peek() {
            // Consume the '=' token.
            self.consume()?;
            // Parse initializer expression.
            let (initializer_expr, pos) = self.parse_cast()?;
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
                position: id_pos.clone(),
            };

            // ...and an assignment node with lhs being the variable name.
            let assign = Assignment {
                lhs: id,
                value: (initializer_expr, pos.clone()),
            };
            // Combine them into a DeclarationAssignment node.
            Ok((
                Box::new(DeclarationAssignment {
                    declaration: (decl, id_pos.clone()),
                    assignment: (assign, pos),
                }),
                id_pos,
            ))
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
            Ok((
                Box::new(VariableDeclaration {
                    id: id,
                    var_type,
                    position: id_pos.clone(),
                }),
                id_pos,
            ))
        }
    }

    fn parse_walrus_decl(&mut self) -> Result<(Box<dyn Node>, Position), ParserError> {
        // Pattern: Identifier, Walrus, Expression, Semicolon.
        let (id_token, pos) = self.consume()?; // Identifier
        let id = if let Token::Identifier(name) = id_token {
            name
        } else {
            unreachable!()
        };

        let (walrus, walrus_pos) = self.consume()?; // Expect the walrus operator (":=")
        if walrus != Token::Walrus {
            return Err(ParserError::SyntaxError {
                message: "Expected ':=' after identifier for walrus declaration.".to_string(),
                file: self.file.clone(),
                position: walrus_pos,
            });
        }

        // Parse the initializer expression.
        let (expr, expr_pos) = self.parse_cast()?;

        // Expect semicolon.
        let (semi, semi_pos) = self.consume()?;
        if semi != Token::Semicolon {
            return Err(ParserError::SyntaxError {
                message: "Expected ';' after walrus declaration.".to_string(),
                file: self.file.clone(),
                position: semi_pos,
            });
        }

        Ok((
            Box::new(WalrusDeclaration {
                id: id,
                _initializer: (expr, expr_pos),
            }),
            pos,
        ))
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
            Err(ParserError::GenericError(String::from("Reached end of token list for unknown reason, it should have stopped at `Token::Eof`")))
        }
    }
}
