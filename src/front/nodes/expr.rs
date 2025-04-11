use crate::front::nodes::node::Node;
use crate::front::nodes::operator::Operator;
use crate::middle::ir::{IRContext, IRInstruction};

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
    // etc.
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
            Expr::Identifier(name) => {
                println!(
                    "{:>width$}-> Expr: Identifier({})",
                    "",
                    name,
                    width = indentation
                );
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
            Expr::Identifier(name) => {
                // Reference an identifier
                let dest = ctx.allocate_temp();
                vec![IRInstruction::Load {
                    dest: dest.clone(),
                    src: name.clone(),
                }]
            }
        }
    }
}
