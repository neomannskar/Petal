use crate::front::nodes::node::Node;
use crate::front::nodes::operator::Operator;
use crate::front::semantic::SemanticContext;
use crate::middle::ir::{IRContext, IRInstruction};

use super::r#type::{BasicType, Type};

pub struct BinaryExpr {
    pub op: Operator,
    pub left: Expr,
    pub right: Expr,
}

impl Node for BinaryExpr {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}-> BinaryExpr: Operator({:?})",
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
    FunctionCall {
        function: String,
        arguments: Vec<Expr>,
    }
    // etc.
}

impl Expr {
    pub fn get_type(&self, ctx: &mut SemanticContext) -> Type {
        match self {
            Expr::Number(_) => Type {
                name: "i32".to_string(),
                basic: Some(BasicType::I32),
            },
            Expr::Binary(bin) => {
                match bin.analyze(ctx) {
                    Ok(()) => { Type::basic("i32") }
                    Err(e) => {
                        eprintln!("{}", e);
                        todo!();
                    }
                }
            }
            Expr::Identifier(id) => {
                if let Some(t) = ctx.lookup(id) {
                    t.clone()
                } else {
                    todo!("");
                }
            }
            Expr::FunctionCall { function, arguments } => {
                if let Some(t) = ctx.lookup(function) {
                    t.clone()
                } else {
                    todo!("");
                }
            }
        }
    }

    pub fn infer_type(&self, ctx: &mut SemanticContext) -> Result<Type, String> {
        match self {
            Expr::Number(_) => {
                // Example: consider numbers to be of type "i32" by default.
                Ok(Type::basic("i32"))
            }
            Expr::Binary(bin_expr) => {
                // Assuming that if the analysis passed, both sides share the same type.
                bin_expr.left.infer_type(ctx)
            }
            Expr::Identifier(id) => {
                if let Some(t) = ctx.lookup(&id) {
                    Ok(t.clone())
                } else {
                    Err(format!("Undefined identifier: {}", id))
                }
            }
            // Expr::FunctionCall { function, arguments }
            _ => { todo!("[_] Expr .get_type()") }
        }
    }
}

impl Node for Expr {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        match self {
            Expr::Number(value) => {
                println!(
                    "{:>width$}-> Expr: Number({})",
                    "",
                    value,
                    width = indentation
                );
            }
            Expr::Binary(binary_expr) => {
                println!("{:>width$}-> Expr: Binary", "", width = indentation);
                binary_expr.display(indentation + 4);
            }
            Expr::Identifier(id) => {
                println!("{:>width$}-> Expr: `{}`", "", id, width = indentation);
            }
            // Expr::FunctionCall { function, arguments }
            Expr::FunctionCall{ function, arguments} => {
                println!("{:>width$}-> FunctionCall: `{}`", "", function, width = indentation);
                
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
                    Some(t) => {
                        Ok(())
                    }
                    None => {
                        println!("{:?}", id);
                        Err(String::from("Identifier not found in hashmap?!"))
                    }
                }
            }
            Expr::FunctionCall { function, arguments } => {
                match ctx.lookup(function) {
                    Some(t) => {
                        Ok(())
                    }
                    None => {
                        println!("{:?}", function);
                        Err(String::from("Identifier not found in hashmap?!"))
                    }
                }
            }
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
            // Expr::FunctionCall { function, arguments }
            _ => { todo!("[_] Expr .get_type()") }
        }
    }
}
