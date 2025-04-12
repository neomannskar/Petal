use rand::seq::index;

use crate::front::nodes::node::Node;
use crate::front::semantic::SemanticContext;
use crate::middle::ir::{IRContext, IRInstruction};

use super::expr::Expr;
use super::r#type::Type;

pub struct FunctionDefinition {
    pub id: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: FunctionReturnType,
    pub body: Box<FunctionBody>,
}

impl Node for FunctionDefinition {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionDefinition: `{}`", "", self.id, width = indentation);

        for param in &self.parameters {
            param.display(indentation + 4);
        }
        self.return_type.display(indentation + 4);
        self.body.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Check if this function name is already defined.
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Function '{}' already declared.", self.id));
        }
        // Here, you might want to create a function signature type.
        // For simplicity, we assume self.return_type can be converted into a Type.
        ctx.add_symbol(&self.id, self.return_type.0.clone());

        // Enter a new scope for the function body.
        ctx.enter_scope();
        // Set the expected return type.
        ctx.current_function_return = Some(self.return_type.0.clone());

        // First, analyze each parameter.
        for param in &self.parameters {
            param.analyze(ctx)?;
        }

        // Analyze the function body.
        self.body.analyze(ctx)?;

        // Exit the function scope and clear the expected return type.
        ctx.current_function_return = None;
        ctx.exit_scope();

        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        let mut instructions = Vec::new();

        // instructions.extend(self.id.ir(ctx));

        // Generate IR for parameters
        for param in &self.parameters {
            instructions.extend(param.ir(ctx));
        }

        // Generate IR for body
        instructions.extend(self.body.ir(ctx));

        // Add a return instruction if necessary
        if !instructions
            .iter()
            .any(|instr| matches!(instr, IRInstruction::Ret(_)))
        {
            instructions.push(IRInstruction::Ret("0".to_string()));
        }

        instructions
    }
}

pub struct FunctionParameter {
    pub id: String,
    pub r#type: Type,
}

impl Node for FunctionParameter {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionParameter: `{}`", "", self.id, width = indentation);
        
        self.r#type.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Parameter '{}' is already declared.", self.id));
        }
        // Insert the parameter into the symbol table.
        ctx.add_symbol(&self.id, self.r#type.clone());
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        Vec::new()
    }
}

pub struct FunctionBody {
    pub children: Vec<Box<dyn Node>>,
}

impl Node for FunctionBody {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionBody", "", width = indentation);
        for child in &self.children {
            child.display(indentation + 4);
        }
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        ctx.enter_scope();
        for stmt in &self.children {
            stmt.analyze(ctx)?;
        }
        ctx.exit_scope();
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        Vec::new()
    }
}

#[derive(Clone)]
pub struct FunctionReturnType(pub Type);

impl Node for FunctionReturnType {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionReturnType:", "", width = indentation);

        self.0.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        Vec::new()
    }
}

pub struct Return {
    pub value: Expr,
}

impl Node for Return {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> Return:", "", width = indentation);

        self.value.display(indentation + 4);
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

        let expr_type = self.value.get_type(ctx); // hypothetical method to compute type; you would implement this
        if expr_type != expected_return_type {
            return Err(format!(
                "Type mismatch in return statement: expected {:?}, found {:?}",
                expected_return_type, expr_type
            ));
        }
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        // vec![IRInstruction::Ret(self.value.clone())]
        Vec::new()
    }
}
