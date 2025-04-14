use colored::Colorize;

use crate::front::{semantic::{SemanticContext, Symbol}, token::Position};

use super::{expr::Expr, node::Node, r#type::Type};

pub struct VariableDeclaration {
    pub id: String, // Variable name.
    pub var_type: Type,
}

impl Node for VariableDeclaration {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}` : {:?}",
            "",
            "VarDecl".red(),
            self.id,
            self.var_type, // .magenta(),
            width = indentation
        );
    }
    fn analyze(&self, _ctx: &mut SemanticContext) -> Result<(), String> {
        /* This is removed for now, later this logic should do this and not the parser

        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Variable '{}' already declared", self.id));
        }
        // Add to symbol table.
        ctx.add_symbol(&self.id, Symbol::Variable(self.var_type.clone()));
        
        */
        Ok(())
    }
    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}

pub struct Assignment {
    pub lhs: String, // For now, just the variable name.
    pub value: (Expr, Position),
}

impl Node for Assignment {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}`",
            "",
            "Assign".red(),
            self.lhs,
            width = indentation
        );
        
        let pos = format!("{}:{}", self.value.1.line, self.value.1.index);
        print!("{}{} |", pos, " ".repeat(10 - pos.len()));
        self.value.0.display(indentation + 4);
    }
    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        if ctx.lookup(&self.lhs).is_none() {
            return Err(format!("Assignment to undeclared variable '{}'", self.lhs));
        }
        Ok(())
    }
    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}

pub struct WalrusDeclaration {
    pub id: String,        // variable name
    pub _initializer: (Expr, Position), // storing the initializer expression
}

impl Node for WalrusDeclaration {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}` := ...",
            "",
            "WalrusDecl".red(),
            self.id,
            width = indentation
        );
    }
    fn analyze(&self, _ctx: &mut SemanticContext) -> Result<(), String> {
        /*

        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Variable '{}' already declared", self.id));
        }
        // In a later phase, infer the type from initializer.
        // For now you could postpone type inference or store a placeholder.
        ctx.add_symbol(
            &self.id,
            Symbol::Variable(Type::Custom("<inferred>".to_string()))
        );

        */
        Ok(())
    }
    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}

// A combined declaration and assignment node.
pub struct DeclarationAssignment {
    pub declaration: (VariableDeclaration, Position),
    pub assignment: (Assignment, Position),
}

impl Node for DeclarationAssignment {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "DeclAssign".red(),
            width = indentation
        );
        let pos = format!("{}:{}", self.declaration.1.line, self.declaration.1.index);
        print!("{}{} |", pos, " ".repeat(10 - pos.len()));
        self.declaration.0.display(indentation + 4);
        
        let pos = format!("{}:{}", self.assignment.1.line, self.assignment.1.index);
        print!("{}{} |", pos, " ".repeat(10 - pos.len()));
        self.assignment.0.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // First analyze the declaration.
        self.declaration.0.analyze(ctx)?;
        // Then check the assignment's lhs is declared.
        self.assignment.0.analyze(ctx)
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        // Later: generate IR for both parts.
        Vec::new()
    }
}

/* Use later when refactoring for better node control

pub struct VariableCall {
    pub id: String,
    pub var_type: Type,
    pub value: Expr,
}

*/
