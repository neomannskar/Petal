use std::{thread::sleep, time::Duration};

use colored::Colorize;

use crate::front::semantic::SemanticContext;

use super::{expr::Expr, node::Node, r#type::Type};

pub struct VariableDeclaration {
    pub id: String, // Variable name.
    pub var_type: Type,
}

impl Node for VariableDeclaration {
    fn push_child(&mut self, _c: Box<dyn Node>) {
        panic!("VariableDeclaration cannot have children!");
    }
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}` : {}",
            "",
            "VarDecl".red(),
            self.id,
            self.var_type.name.magenta(),
            width = indentation
        );
    }
    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Variable '{}' already declared", self.id));
        }
        // Add to symbol table.
        ctx.add_symbol(&self.id, self.var_type.clone());
        Ok(())
    }
    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}

pub struct Assignment {
    pub lhs: String, // For now, just the variable name.
    pub value: Expr,
}

impl Node for Assignment {
    fn push_child(&mut self, _c: Box<dyn Node>) {
        panic!("Assignment nodes do not have children!");
    }
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}`",
            "",
            "Assign".red(),
            self.lhs,
            width = indentation
        );
        
        self.value.display(indentation + 4);
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
    pub initializer: Expr, // storing the initializer expression
}

impl Node for WalrusDeclaration {
    fn push_child(&mut self, _c: Box<dyn Node>) {
        panic!("WalrusDeclaration doesn't support children!");
    }
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}` := ...",
            "",
            "WalrusDecl".red(),
            self.id,
            width = indentation
        );
    }
    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Variable '{}' already declared", self.id));
        }
        // In a later phase, infer the type from initializer.
        // For now you could postpone type inference or store a placeholder.
        ctx.add_symbol(
            &self.id,
            Type {
                name: "<inferred>".to_string(),
                basic: None,
            },
        );
        Ok(())
    }
    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}

// A combined declaration and assignment node.
pub struct DeclarationAssignment {
    pub declaration: VariableDeclaration,
    pub assignment: Assignment,
}

impl Node for DeclarationAssignment {
    fn push_child(&mut self, _c: Box<dyn Node>) {
        panic!("DeclarationAssignment cannot have children!");
    }
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "DeclAssign".red(),
            width = indentation
        );
        self.declaration.display(indentation + 4);
        self.assignment.display(indentation + 4);
    }
    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // First analyze the declaration.
        self.declaration.analyze(ctx)?;
        // Then check the assignment's lhs is declared.
        self.assignment.analyze(ctx)
    }
    fn ir(&self, ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        // Later: generate IR for both parts.
        Vec::new()
    }
}
