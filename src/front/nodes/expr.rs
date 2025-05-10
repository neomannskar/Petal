use colored::Colorize;

use crate::front::nodes::node::Node;
use crate::front::nodes::operator::Operator;
use crate::front::semantic::{SemanticContext, Symbol};
use crate::front::token::Position;
use crate::middle::ir::{flatten_units, IRContext, IRInstruction, IRUnit};

use super::cast::Cast;
use super::r#type::Type;

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Operator,
    pub left: (Expr, Position),
    pub right: (Expr, Position),
}

impl Node for BinaryExpr {
    fn display(&self, indentation: usize) {
        println!("{:>width$}└───[ {:?}", "", self.op, width = indentation);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Analyze left and right operands.

        self.left.0.analyze(ctx)?;
        self.right.0.analyze(ctx)?;

        // Infer types for both operands.
        let left_type = self.left.0.get_type(ctx);
        let right_type = self.right.0.get_type(ctx);

        // Check type compatibility (for example, both must be numbers for arithmetic ops).
        if left_type != right_type {
            return Err("Type mismatch in binary expression.".to_string());
        }

        // Further operator-specific checks could go here.
        Ok(())
    }

    fn ir(&self, _ctx: &mut IRContext) -> Vec<IRUnit> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Cast((Cast, Box<Expr>), Position),
    Number((String, Position)),
    Boolean((String, Position)),
    Character((String, Position)),
    String((String, Position)),
    Binary((Box<BinaryExpr>, Position)),
    VariableCall((String, Position)),
    FunctionCall {
        function: (String, Position),
        arguments: Vec<(Expr, Position)>,
    },
    // etc.
}

impl Expr {
    /// A non-fallible version returning the type of the expression.
    pub fn get_type(&self, ctx: &mut SemanticContext) -> Type {
        match self {
            Expr::Cast((cast, _expr), _) => {
                match &cast {
                    Cast::Primitive((cast_type, _)) => {
                        Type::basic(&cast_type)
                    }
                    Cast::NonPrimitive((cast_type, _)) => {
                        Type::Custom(cast_type.clone())
                    }
                }
            }
            Expr::Boolean(_) => {
                Type::basic("bool")
            }
            Expr::Number(_) => {
                // By default, we treat literal numbers as i32.
                Type::basic("i32")
            }
            Expr::Character(_) => Type::basic("char"),
            Expr::String(_) => Type::basic("str"),
            Expr::Binary((bin, _)) => {
                // For simplicity, we assume that a binary expression is valid and
                // its type is that of its left side.
                bin.left.0.get_type(ctx)
            }
            Expr::VariableCall((id, pos)) => {
                // dbg!(&ctx);
                if let Some((symbol, _)) = ctx.lookup(id) {
                    match symbol {
                        Symbol::Variable(t) => t.clone(),
                        Symbol::Function(func_type) => Type::Function(func_type.clone()),
                        Symbol::Struct(strct) => Type::Struct(strct.clone()),
                        // If you have other categories, you could add them here.
                    }
                } else {
                    panic!(
                        "Undefined identifier: {} on line {} at index {}",
                        id, pos.line, pos.index
                    );
                }
            }
            Expr::FunctionCall {
                function,
                arguments: _,
            } => {
                if let Some((symbol, _)) = ctx.lookup(&function.0) {
                    // Expect the looked-up symbol to be a function.
                    if let Symbol::Function(func_type) = symbol {
                        *func_type.return_type.clone()
                    } else {
                        panic!("Identifier `{}` is not a function", function.0);
                    }
                } else {
                    panic!("Failed to locate the function '{}'", function.0);
                }
            }
        }
    }

    /// A fallible version that returns an error string on failure.
    pub fn infer_type(&self, ctx: &mut SemanticContext) -> Result<Type, String> {
        match self {
            Expr::Cast((cast, _), _) => {
                match cast {
                    Cast::Primitive((cast_type, _)) => {
                        Ok(Type::basic(&cast_type))
                    }
                    Cast::NonPrimitive((cast_type, _)) => {
                        Ok(Type::Custom(cast_type.clone()))
                    }
                }
            }
            Expr::Boolean(_) => Ok(Type::basic("bool")),
            Expr::Number(_) => Ok(Type::basic("i32")),
            Expr::Character(_) => Ok(Type::basic("char")),
            Expr::String(_) => Ok(Type::basic("str")),
            Expr::Binary((bin_expr, _)) => bin_expr.left.0.infer_type(ctx),
            Expr::VariableCall((id, _)) => {
                if let Some((symbol, _)) = ctx.lookup(id) {
                    match symbol {
                        Symbol::Variable(t) => Ok(t.clone()),
                        Symbol::Function(func_type) => Ok(Type::Function(func_type.clone())),
                        Symbol::Struct(strct) => Ok(Type::Struct(strct.clone())),
                    }
                } else {
                    Err(format!("Undefined identifier: {}", id))
                }
            }
            Expr::FunctionCall {
                function,
                arguments: _,
            } => {
                if let Some((symbol, _)) = ctx.lookup(&function.0) {
                    if let Symbol::Function(func_type) = symbol {
                        Ok(*func_type.return_type.clone())
                    } else {
                        Err(format!("Identifier '{}' is not a function", function.0))
                    }
                } else {
                    Err(format!("Failed to locate function '{}'", function.0))
                }
            }
        }
    }

    /// Flatten the expression into TAC. Returns a temporary and a vector of IR instructions.
    pub fn tac(&self, ctx: &mut IRContext) -> (String, Vec<IRInstruction>) {
        match self {
            // Number literal: parse the string into an i64 and emit a LoadConstant.
            Expr::Number((n, _pos)) => {
                let tmp = ctx.allocate_temp();
                let num_value = n
                    .parse::<i64>()
                    .expect("Failed to parse number literal into i64");
                (
                    tmp.clone(),
                    vec![IRInstruction::LoadConstant {
                        dest: tmp,
                        value: num_value,
                    }],
                )
            }
            // Binary expression: get TAC for both sides, then emit an instruction based on the operator.
            Expr::Binary((boxed_bin_expr, _pos)) => {
                // Destructure the boxed BinaryExpr.
                let BinaryExpr { left, op, right } = &**boxed_bin_expr;
                let (t_left, mut inst_left) = left.0.tac(ctx);
                let (t_right, mut inst_right) = right.0.tac(ctx);
                let tmp = ctx.allocate_temp();
                let op_inst = match op {
                    Operator::Asterisk => IRInstruction::Mul {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::Fslash => IRInstruction::Div {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::Percent => IRInstruction::Mod {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::Plus => IRInstruction::Add {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::Minus => IRInstruction::Sub {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::And => IRInstruction::And {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::Or => IRInstruction::Or {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::Xor => IRInstruction::Xor {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::ShiftLeft => IRInstruction::ShiftLeft {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    Operator::ShiftRight => IRInstruction::ShiftRight {
                        dest: tmp.clone(),
                        op1: t_left,
                        op2: t_right,
                    },
                    // For other operators, you can add more cases or leave them unimplemented.
                    Operator::Walrus
                    | Operator::Asign
                    | Operator::Equals
                    | Operator::NotEquals
                    | Operator::Compare
                    | Operator::Not => unimplemented!("Operator not implemented in TAC"),
                };
                let mut instructions = Vec::new();
                instructions.append(&mut inst_left);
                instructions.append(&mut inst_right);
                instructions.push(op_inst);
                (tmp, instructions)
            }
            // Character literal: take the first character and convert it to its Unicode (i64) code.
            Expr::Character((ch, _pos)) => {
                let tmp = ctx.allocate_temp();
                let code = ch.chars().next().expect("Character literal is empty") as i64;
                (
                    tmp.clone(),
                    vec![IRInstruction::LoadConstant {
                        dest: tmp,
                        value: code,
                    }],
                )
            }
            // String literal: for now, mark as unimplemented (you might wish to add a LoadString variant later).
            Expr::String((_s, _pos)) => {
                todo!("TAC for string literals is not yet implemented")
            }
            // Variable call: load the variable's value (assumed to be accessible by its name).
            Expr::VariableCall((var_name, _pos)) => {
                let tmp = ctx.allocate_temp();
                // Look up the variable's allocated offset from IRContext.
                let mem_operand = if let Some(offset) = ctx.stack_allocations.get(var_name) {
                    // Build a memory operand using the offset.
                    format!("{}(%rbp)", offset)
                } else {
                    // Fall back to the variable name (shouldn't happen if analysis is complete).
                    var_name.clone()
                };
                (
                    tmp.clone(),
                    vec![IRInstruction::Load {
                        dest: tmp,
                        src: mem_operand,
                    }],
                )
            }
            // Function call: generate TAC for each argument and then call the function.
            Expr::FunctionCall {
                function,
                arguments,
            } => {
                let mut instructions = Vec::new();
                let mut arg_temps = Vec::new();
                for arg in arguments {
                    let (temp, arg_instrs) = arg.0.tac(ctx);
                    instructions.extend(arg_instrs);
                    arg_temps.push(temp);
                }
                let tmp = ctx.allocate_temp();
                instructions.push(IRInstruction::Call {
                    dest: tmp.clone(),
                    fn_id: function.0.clone(),
                    args: arg_temps,
                });
                (tmp, instructions)
            }
            _ => unimplemented!("TAC for this kind of expression is not implemented"),
        }
    }
}

impl Node for Expr {
    fn display(&self, indentation: usize) {
        match self {
            Expr::Cast((cast, expr), pos) => {
                cast.display(indentation + 4);
                let pos = format!("{}:{}", pos.line, pos.index);
                print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                expr.display(indentation + 8);
            }
            Expr::Boolean((bool, _)) => {
                println!("{:>width$}└───[ {}", "", bool.magenta(), width = indentation);
            }
            Expr::Number((value, _)) => {
                println!("{:>width$}└───[ `{}`", "", value, width = indentation);
            }
            Expr::Character((ch, _)) => {
                println!("{:>width$}└───[ '{}'", "", ch, width = indentation);
            }
            Expr::String((str, _)) => {
                println!(
                    "{:>width$}└───[ \"{}\"",
                    "",
                    str.replace("\n", "").replace("\r", ""),
                    width = indentation
                );
            }
            Expr::Binary((binary_expr, _)) => {
                println!(
                    "{:>width$}└───[ {}: {:?}",
                    "",
                    "Expr".magenta(),
                    binary_expr.op,
                    width = indentation
                );

                let pos = format!("{}:{}", binary_expr.left.1.line, binary_expr.left.1.index);
                print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                binary_expr.left.0.display(indentation + 4);

                let pos = format!("{}:{}", binary_expr.right.1.line, binary_expr.right.1.index);
                print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                binary_expr.right.0.display(indentation + 4);
            }
            Expr::VariableCall((id, _)) => {
                println!(
                    "{:>width$}└───[ {}: `{}`",
                    "",
                    "Id".magenta(),
                    id,
                    width = indentation
                );
            }
            Expr::FunctionCall {
                function,
                arguments,
            } => {
                println!(
                    "{:>width$}└───[ {}: `{}`",
                    "",
                    "FnCall".green(),
                    function.0,
                    width = indentation
                );

                for (expr, pos) in arguments {
                    let pos = format!("{}:{}", pos.line, pos.index);
                    print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                    expr.display(indentation + (10 - pos.len()) + 4);
                }
            }
        }
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        match self {
            Expr::Cast((_, _), _) => {
                Ok(())
            }
            Expr::Boolean(_) => {
                Ok(())
            }
            Expr::Number(_) => {
                // A literal number is always valid.
                Ok(())
            }
            Expr::Character(_) => Ok(()),
            Expr::String(_) => Ok(()),
            Expr::Binary((bin_expr, _)) => {
                // Delegate to BinaryExpr's analysis.
                bin_expr.analyze(ctx)
            }
            Expr::VariableCall((id, _)) => {
                // Analyze the identifier node (ensures it's defined).

                match ctx.lookup(id) {
                    Some(_s) => Ok(()),
                    None => {
                        println!("{:?}", id);
                        Err(String::from("Identifier not found in hashmap?!"))
                    }
                }
            }
            Expr::FunctionCall {
                function,
                arguments: _, // Add later a check for what the function in the symbol table accepts as arguments.
            } => match ctx.lookup(&function.0) {
                Some(_s) => Ok(()),
                None => {
                    println!("{:?}", function);
                    Err(String::from("Identifier not found in hashmap?!"))
                }
            },
        }
    }

    /// Generate IR for the expression, returning a vector of IRUnit.
    fn ir(&self, ctx: &mut IRContext) -> Vec<IRUnit> {
        match self {
            Expr::Cast((_, _), _) => {
                todo!()
            }
            Expr::Boolean(_) => {
                todo!()
            }
            // For a number literal, allocate a temporary and load a constant.
            Expr::Number((n, _pos)) => {
                let tmp = ctx.allocate_temp();
                let value = n
                    .parse::<i64>()
                    .expect("Failed to parse number literal into i64");
                vec![IRUnit::Global(vec![IRInstruction::LoadConstant {
                    dest: tmp,
                    value,
                }])]
            }
            // For characters and strings, you'll implement as needed.
            Expr::Character(_) => todo!("IR for character not implemented"),
            Expr::String(_) => todo!("IR for string not implemented"),
            // For a binary expression, use tac on both operands, then combine.
            Expr::Binary((boxed_bin_expr, _pos)) => {
                // Destructure the boxed BinaryExpr.
                let BinaryExpr { left, op, right } = &**boxed_bin_expr;
                // Use our tac helper to generate TAC for both operands.
                let (left_temp, left_instrs) = left.0.tac(ctx);
                let (right_temp, right_instrs) = right.0.tac(ctx);
                // Allocate a new temporary for the result.
                let dest = ctx.allocate_temp();
                // Build the appropriate arithmetic or logical IR instruction.
                let bin_inst = match op {
                    Operator::Plus => IRInstruction::Add {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::Minus => IRInstruction::Sub {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::Asterisk => IRInstruction::Mul {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::Fslash => IRInstruction::Div {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::Percent => IRInstruction::Mod {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::And => IRInstruction::And {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::Or => IRInstruction::Or {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::Xor => IRInstruction::Xor {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::ShiftLeft => IRInstruction::ShiftLeft {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    Operator::ShiftRight => IRInstruction::ShiftRight {
                        dest: dest.clone(),
                        op1: left_temp,
                        op2: right_temp,
                    },
                    // For operators you haven't implemented yet:
                    Operator::Walrus
                    | Operator::Asign
                    | Operator::Equals
                    | Operator::NotEquals
                    | Operator::Compare
                    | Operator::Not => todo!("Operator {:?} not implemented in IR", op),
                };
                // Combine the instructions: first compute the left operand, then the right,
                // and finally perform the binary operation.
                let mut instructions = Vec::new();
                instructions.extend(left_instrs);
                instructions.extend(right_instrs);
                instructions.push(bin_inst);
                vec![IRUnit::Global(instructions)]
            }
            // A variable call loads the variable's value (assumed to be stored at a known location).
            Expr::VariableCall((var_name, _pos)) => {
                let dest = ctx.allocate_temp();
                vec![IRUnit::Global(vec![IRInstruction::Load {
                    dest,
                    src: var_name.clone(),
                }])]
            }
            // For a function call, generate IR for each argument, collect their result temporaries,
            // and emit a call instruction.
            Expr::FunctionCall {
                function,
                arguments,
            } => {
                let mut arg_temps: Vec<String> = Vec::new();
                let mut instructions = Vec::new();
                for arg in arguments {
                    // Here we use tac so we get a temporary register for each argument.
                    let (temp, arg_instrs) = arg.0.tac(ctx);
                    instructions.extend(arg_instrs);
                    arg_temps.push(temp);
                }
                let dest = ctx.allocate_temp();
                instructions.push(IRInstruction::Call {
                    dest,
                    fn_id: function.0.clone(),
                    args: arg_temps,
                });
                vec![IRUnit::Global(instructions)]
            }
        }
    }
}

pub struct ExpressionStatement {
    pub expression: Expr,
}

impl Node for ExpressionStatement {
    fn display(&self, indentation: usize) {
        println!("{:>width$}└───[ ExprStat", "", width = indentation);
        // Display the underlying expression; you could customize this as needed.
        // For instance:
        match &self.expression {
            Expr::Cast((cast, _), _) => {
                match cast {
                    Cast::Primitive((cast_type, _)) => {
                        println!("{:>width$}-> PrimtiveCast({})", "", cast_type, width = indentation + 4);
                    }
                    Cast::NonPrimitive((cast_type, _)) => {
                        println!("{:>width$}-> NonPrimitiveCast({})", "", cast_type, width = indentation + 4);
                    }
                }
            }
            Expr::Boolean((bool, _)) => {
                println!("{:>width$}-> Number({})", "", bool, width = indentation + 4);
            }
            Expr::Number((n, _)) => {
                println!("{:>width$}-> Number({})", "", n, width = indentation + 4);
            }
            Expr::Character((ch, _)) => println!(
                "{:>width$}-> Character('{}')",
                "",
                ch,
                width = indentation + 4
            ),
            Expr::String((str, _)) => println!(
                "{:>width$}-> String(\"{}\")",
                "",
                str,
                width = indentation + 4
            ),
            Expr::Binary((bin, _)) => bin.display(indentation + 4),
            Expr::VariableCall((id, _)) => println!(
                "{:>width$}-> Identifier({})",
                "",
                id,
                width = indentation + 4
            ),
            Expr::FunctionCall {
                function,
                arguments,
            } => {
                println!(
                    "{:>width$}└───[ FnCall: `{}`",
                    "",
                    function.0,
                    width = indentation + 4
                );
                for (arg, pos) in arguments {
                    let pos = format!("{}:{}", pos.line, pos.index);
                    print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                    println!(
                        "{:>width$}└───[ Argument:",
                        "",
                        width = indentation + (10 - pos.len()) + 8
                    );
                    arg.display(indentation + 12);
                }
            }
        }
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        self.expression.analyze(ctx)
    }

    fn ir(&self, _ctx: &mut IRContext) -> Vec<IRUnit> {
        Vec::new()
    }
}
