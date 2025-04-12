use std::{thread::sleep, time::Duration};

use colored::Colorize;

use crate::front::semantic::{SemanticContext, Symbol};

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
    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
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
    pub value: Expr,
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
    pub declaration: VariableDeclaration,
    pub assignment: Assignment,
}

impl Node for DeclarationAssignment {
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

/* Use later when refactoring for better node control

pub struct VariableCall {
    pub id: String,
    pub var_type: Type,
    pub value: Expr,
}

*/
