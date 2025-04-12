use colored::Colorize;

use crate::front::nodes::node::Node;
use crate::front::nodes::operator::Operator;
use crate::front::semantic::{SemanticContext, Symbol};
use crate::middle::ir::{IRContext, IRInstruction};

use super::r#type::Type;

pub struct BinaryExpr {
    pub op: Operator,
    pub left: Expr,
    pub right: Expr,
}

impl Node for BinaryExpr {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {:?}",
            "",
            self.op,
            width = indentation
        );
        self.left.display(indentation + 4);
        self.right.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Analyze left and right operands.
        self.left.analyze(ctx)?;
        self.right.analyze(ctx)?;

        // Infer types for both operands.
        let left_type = self.left.get_type(ctx);
        let right_type = self.right.get_type(ctx);

        // Check type compatibility (for example, both must be numbers for arithmetic ops).
        if left_type != right_type {
            return Err("Type mismatch in binary expression.".to_string());
        }

        // Further operator-specific checks could go here.
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        let mut instructions = Vec::new();

        // Generate IR for the left operand
        let left_ir = self.left.ir(ctx);
        instructions.extend(left_ir); // Add left operand's instructions

        // Generate IR for the right operand
        let right_ir = self.right.ir(ctx);
        instructions.extend(right_ir); // Add right operand's instructions

        // Allocate a temporary register for the result of this binary operation
        let dest = ctx.allocate_temp();

        // Emit an instruction for the binary operation
        let op_instruction = match self.op {
            Operator::Plus => IRInstruction::Add {
                dest: dest.clone(),
                lhs: ctx.get_last_temp(), // Use the last allocated temp for the left operand
                rhs: ctx.get_second_last_temp(), // Use the second-to-last allocated temp for the right operand
            },
            Operator::Minus => IRInstruction::Sub {
                dest: dest.clone(),
                lhs: ctx.get_last_temp(),
                rhs: ctx.get_second_last_temp(),
            },
            // Extend to support more operators (e.g., Multiply, Divide, etc.)
            _ => panic!("Unsupported operator in BinaryExpr."),
        };

        instructions.push(op_instruction);

        // IMPORTANT!
        // ctx.add_dest(dest);

        instructions
    }
}

pub enum Expr {
    Number(i64),
    Binary(Box<BinaryExpr>),
    Identifier(String),
    VariableCall {
        id: String,
        resolved: Option<Symbol>,
    },
    FunctionCall {
        function: String,
        arguments: Vec<Expr>,
    },
    // etc.
}

impl Expr {
    /// A non-fallible version returning the type of the expression.
    pub fn get_type(&self, ctx: &mut SemanticContext) -> Type {
        match self {
            Expr::Number(_) => {
                // By default, we treat literal numbers as i32.
                Type::basic("i32")
            }
            Expr::Binary(bin) => {
                // For simplicity, we assume that a binary expression is valid and
                // its type is that of its left side.
                bin.left.get_type(ctx)
            }
            Expr::Identifier(id) => {
                if let Some(symbol) = ctx.lookup(id) {
                    match symbol {
                        Symbol::Variable(t) => t.clone(),
                        Symbol::Function(func_type) => Type::Function(func_type.clone()),
                        Symbol::Struct(strct) => Type::Struct(strct.clone()),
                        // If you have other categories, you could add them here.
                    }
                } else {
                    panic!("Undefined identifier: {}", id);
                }
            }
            Expr::VariableCall { id, resolved: _ } => {
                if let Some(symbol) = ctx.lookup(id) {
                    if let Symbol::Variable(var_type) = symbol {
                        var_type.clone()
                    } else {
                        panic!("Identifier `{}` is not a variable", id);
                    }
                } else {
                    panic!("Failed to locate the variable `{}`", id);
                }
            }
            Expr::FunctionCall { function, arguments: _ } => {
                if let Some(symbol) = ctx.lookup(function) {
                    // Expect the looked-up symbol to be a function.
                    if let Symbol::Function(func_type) = symbol {
                        *func_type.return_type.clone()
                    } else {
                        panic!("Identifier `{}` is not a function", function);
                    }
                } else {
                    panic!("Failed to locate the function '{}'", function);
                }
            }
        }
    }

    /// A fallible version that returns an error string on failure.
    pub fn infer_type(&self, ctx: &mut SemanticContext) -> Result<Type, String> {
        match self {
            Expr::Number(_) => Ok(Type::basic("i32")),
            Expr::Binary(bin_expr) => bin_expr.left.infer_type(ctx),
            Expr::Identifier(id) => {
                if let Some(symbol) = ctx.lookup(id) {
                    match symbol {
                        Symbol::Variable(t) => Ok(t.clone()),
                        Symbol::Function(func_type) => Ok(Type::Function(func_type.clone())),
                        Symbol::Struct(strct) => Ok(Type::Struct(strct.clone())),
                    }
                } else {
                    Err(format!("Undefined identifier: {}", id))
                }
            }
            Expr::VariableCall { id, resolved: _ } => {
                if let Some(symbol) = ctx.lookup(id) {
                    if let Symbol::Variable(var_type) = symbol {
                        Ok(var_type.clone())
                    } else {
                        Err(format!("Identifier '{}' is not a function", id))
                    }
                } else {
                    Err(format!("Failed to locate function '{}'", id))
                }
            }
            Expr::FunctionCall { function, arguments: _ } => {
                if let Some(symbol) = ctx.lookup(function) {
                    if let Symbol::Function(func_type) = symbol {
                        Ok(*func_type.return_type.clone())
                    } else {
                        Err(format!("Identifier '{}' is not a function", function))
                    }
                } else {
                    Err(format!("Failed to locate function '{}'", function))
                }
            }
        }
    }
}

impl Node for Expr {
    fn display(&self, indentation: usize) {
        match self {
            Expr::Number(value) => {
                println!("{:>width$}└───[ `{}`", "", value, width = indentation);
            }
            Expr::Binary(binary_expr) => {
                // println!("{:>width$}└───[ Expr: Binary", "", width = indentation);
                binary_expr.display(indentation /* + 4 */);
            }
            Expr::Identifier(id) => {
                println!(
                    "{:>width$}└───[ {}: `{}`",
                    "",
                    "Id".magenta(),
                    id,
                    width = indentation
                );
            }
            Expr::VariableCall { id, resolved } => {
                println!(
                    "{:>width$}└───[ {}: `{}` : {:?}",
                    "",
                    "VarCall".red(),
                    id,
                    resolved,
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
                    function,
                    width = indentation
                );

                for expr in arguments {
                    expr.display(indentation + 4);
                }
            }
        }
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        match self {
            Expr::Number(_) => {
                // A literal number is always valid.
                Ok(())
            }
            Expr::Binary(bin_expr) => {
                // Delegate to BinaryExpr's analysis.
                bin_expr.analyze(ctx)
            }
            Expr::Identifier(id) => {
                // Analyze the identifier node (ensures it's defined).

                match ctx.lookup(id) {
                    Some(_s) => Ok(()),
                    None => {
                        println!("{:?}", id);
                        Err(String::from("Identifier not found in hashmap?!"))
                    }
                }
            }
            Expr::VariableCall { id, resolved: _ } => {
                if let Some(symbol) = ctx.lookup(id) {
                    if let Symbol::Variable(_var_type) = symbol {
                        // Optionally, you could even update the node with the resolved symbol,
                        // so later phases have immediate access to things like memory offsets.
                        // resolved = Some(symbol.clone());
                        Ok(())
                    } else {
                        Err(format!("Identifier '{}' is not a variable", id))
                    }
                } else {
                    Err(format!("Undefined variable: {}", id))
                }
            }
            Expr::FunctionCall {
                function,
                arguments,
            } => match ctx.lookup(function) {
                Some(_s) => Ok(()),
                None => {
                    println!("{:?}", function);
                    Err(String::from("Identifier not found in hashmap?!"))
                }
            },
        }
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        match self {
            Expr::Number(value) => {
                // Load the constant into a new temporary register
                let dest = ctx.allocate_temp();
                vec![IRInstruction::Load {
                    dest: dest.clone(),
                    src: value.to_string(),
                }]
            }
            Expr::Binary(binary_expr) => {
                // Delegate to the BinaryExpr's ir() method
                binary_expr.ir(ctx)
            }
            Expr::Identifier(id) => {
                // Reference an identifier
                let dest = ctx.allocate_temp();
                vec![IRInstruction::Load {
                    dest: dest.clone(),
                    src: id.clone(),
                }]
            }
            Expr::VariableCall { id, resolved } => {
                // Here you would generate the proper IR load instruction.
                // If `resolved` is set, you can retrieve extra info (e.g. memory location).
                let symbol = resolved.as_ref().expect("Symbol should be resolved by now");
                // For example:
                vec![IRInstruction::LoadVariable {
                    dest: ctx.allocate_temp(),
                    variable: id.clone(),
                    // possibly more fields based on 'symbol'
                }]
            },
            // Expr::FunctionCall { function, arguments }
            _ => {
                todo!("[_] Expr .get_type()")
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
            Expr::Number(n) => println!("{:>width$}-> Number({})", "", n, width = indentation + 4),
            Expr::Binary(bin) => bin.display(indentation + 4),
            Expr::Identifier(id) => println!(
                "{:>width$}-> Identifier({})",
                "",
                id,
                width = indentation + 4
            ),
            Expr::VariableCall { id, resolved } => {
                println!(
                    "{:>width$}└───[ VarCall: `{}` : {:?}",
                    "",
                    id,
                    resolved,
                    width = indentation + 4
                );
            }
            Expr::FunctionCall {
                function,
                arguments,
            } => {
                println!(
                    "{:>width$}└───[ FnCall: `{}`",
                    "",
                    function,
                    width = indentation + 4
                );
                for arg in arguments {
                    // You could call display recursively if type Expr implements Node-like behavior.
                    println!("{:>width$}└───[ Argument:", "", width = indentation + 8);
                    arg.display(indentation + 12);
                }
            }
        }
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        self.expression.analyze(ctx)
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        self.expression.ir(ctx)
    }
}
