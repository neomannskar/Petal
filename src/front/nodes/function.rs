use colored::Colorize;

use crate::front::nodes::node::Node;
use crate::front::semantic::{SemanticContext, Symbol};
use crate::front::token::Position;
use crate::middle::ir::{IRContext, IRFunction, IRInstruction, IRType, IRUnit};

use super::body::Body;
use super::expr::Expr;
use super::parameter::Parameter;
use super::r#type::{FunctionType, Type};

pub struct FunctionDefinition {
    pub id: (String, Position),
    pub parameters: Vec<(Parameter, Position)>,
    pub body: (Box<Body>, Position),
    pub return_type: (FunctionReturnType, Position),
}

impl Node for FunctionDefinition {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}`",
            "",
            "FnDef".yellow(),
            self.id.0,
            width = indentation
        );

        for (param, param_pos) in &self.parameters {
            let param_pos = format!("{}:{}", param_pos.line, param_pos.index);
            print!("{}{} |", param_pos, " ".repeat(10 - param_pos.len()));
            param.display(indentation + 4);
        }

        let ret_pos = format!("{}:{}", self.return_type.1.line, self.return_type.1.index);
        print!("{}{} |", ret_pos, " ".repeat(10 - ret_pos.len()));
        self.return_type.0.display(indentation + 4);

        let body_pos = format!("{}:{}", self.body.1.line, self.body.1.index);
        print!("{}{} |", body_pos, " ".repeat(10 - body_pos.len()));
        self.body.0.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Check if this function name is already defined.
        if ctx.lookup(&self.id.0).is_some() {
            return Err(format!("Function '{}' already declared.", self.id.0));
        }
        // Here, you might want to create a function signature type.
        // For simplicity, we assume self.return_type can be converted into a Type.
        ctx.add_symbol(
            &self.id.0, // self.id is now (String, Position)
            (
                Symbol::Function(FunctionType {
                    // Refactor in future
                    parameters: self
                        .parameters
                        .iter()
                        .map(|(param, _param_pos)| param.var_type.clone())
                        .collect(),
                    return_type: Box::new(self.return_type.0 .0.clone()),
                }),
                self.id.1.clone(), // Use the position of the function's identifier
            ),
        );

        // Enter a new scope for the function body.
        ctx.enter_scope();
        // Set the expected return type.
        ctx.current_function_return = Some(self.return_type.0 .0.clone());

        // First, analyze each parameter.
        for (param, _) in &self.parameters {
            param.analyze(ctx)?;
        }

        // Analyze the function body.
        self.body.0.analyze(ctx)?;

        // Exit the function scope and clear the expected return type.
        ctx.current_function_return = None;
        ctx.exit_scope();

        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRUnit> {
        let mut instructions: Vec<IRInstruction> = Vec::new();

        // Process each parameter.
        for (i, (param, _)) in self.parameters.iter().enumerate() {
            let param_type = IRType::from_type(&param.var_type);
            // Allocate the parameter variable on the stack.
            ctx.allocate_variable(&param.id, &param_type);

            // Use the target to get the proper argument register.
            let arg_reg = ctx.target.arg_registers(i);

            let offset = ctx.stack_allocations.get(&param.id).unwrap();
            let offset_str = format!("{}(%rbp)", offset);

            // Emit a Store instruction to move the argument (in a register) to its stack slot.
            instructions.push(IRInstruction::Store {
                dest: offset_str,
                src: arg_reg,
            });
        }

        // Generate IR for the function body and flatten it.
        let body_units = self.body.0.ir(ctx);
        for unit in body_units {
            match unit {
                IRUnit::Global(insts) => instructions.extend(insts),
                IRUnit::Function(_func) => {
                    // We normally expect only global instructions within a function body.
                }
            }
        }

        // Wrap the final instruction list into an IRFunction unit.
        vec![IRUnit::Function(IRFunction {
            id: self.id.0.clone(),
            instructions,
        })]
    }
}

#[derive(Clone)]
pub struct FunctionReturnType(pub Type);

impl Node for FunctionReturnType {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: {:?}",
            "",
            "FnRetType".blue(),
            self.0, // .magenta()
            width = indentation
        );

        // self.0.display(indentation + 4);
    }

    fn analyze(&self, _ctx: &mut SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut IRContext) -> Vec<IRUnit> {
        Vec::new() // No IR instructions for return type, it is meant for semantic analysis.
    }
}

pub struct Return {
    pub value: (Expr, Position),
}

impl Node for Return {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}:",
            "",
            "Return".red(),
            width = indentation
        );
        let pos = format!("{}:{}", self.value.1.line, self.value.1.index);
        print!("{}{} |", pos, " ".repeat(10 - pos.len()));
        self.value.0.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Ensure there is a current function return type set.
        let expected_return_type: Type;

        if let Some(exp) = &ctx.current_function_return {
            // Analyze the expression and derive its type.
            // ... self.value.analyze(ctx)
            // Assuming self.expr (or self.value if you update your node) now holds an expression:
            expected_return_type = exp.clone();
        } else {
            return Err("Return statement found outside of a function.".to_string());
        }

        let expr_type = self.value.0.get_type(ctx); // hypothetical method to compute type; you would implement this
        if expr_type != expected_return_type {
            return Err(format!(
                "Type mismatch in return statement: expected {:?}, found {:?}",
                expected_return_type, expr_type
            ));
        }
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRUnit> {
        // Flatten the expression: generate TAC for computing the return value.
        // We assume `tac` returns (temporary, IR instructions) for the expression.
        let (temp, mut expr_insts) = self.value.0.tac(ctx);

        // Append a Return instruction that uses the temporary holding the return value.
        expr_insts.push(IRInstruction::Return { value: temp });

        // Wrap the sequence of instructions in an IRUnit.
        vec![IRUnit::Global(expr_insts)]
    }
}
