use colored::Colorize;

use crate::{front::{semantic::{SemanticContext, Symbol}, token::Position}, middle::ir::{IRContext, IRInstruction, IRType, IRUnit}};

use super::{expr::Expr, node::Node, r#type::Type};

pub struct VariableDeclaration {
    pub id: String, // Variable name.
    pub var_type: Type,
    pub position: Position,
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
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Variable '{}' already declared", self.id));
        }
        // Add to symbol table.
        ctx.add_symbol(&self.id, (Symbol::Variable(self.var_type.clone()), self.position.clone()));
        
        Ok(())
    }

    fn ir(&self, _ctx: &mut IRContext) -> Vec<IRUnit> {
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
    fn ir(&self, _ctx: &mut IRContext) -> Vec<IRUnit> {
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

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        if ctx.lookup(&self.id).is_some() {
            return Ok(());
            // return Err(format!("Variable '{}' already declared", self.id));
        } else {
            // ctx.add_symbol(&self.id, Symbol());
        }
        // In a later phase, infer the type from initializer.
        // For now you could postpone type inference or store a placeholder.

        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<IRUnit> {
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
        let pos_decl = format!("{}:{}", self.declaration.1.line, self.declaration.1.index);
        print!("{}{} |", pos_decl, " ".repeat(10 - pos_decl.len()));
        self.declaration.0.display(indentation + 4);
        
        let pos_assign = format!("{}:{}", self.assignment.1.line, self.assignment.1.index);
        print!("{}{} |", pos_assign, " ".repeat(10 - pos_assign.len()));
        self.assignment.0.display(indentation + 4);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // First analyze the declaration.
        self.declaration.0.analyze(ctx)?;
        // Then check that the assignment's lhs is declared.
        self.assignment.0.analyze(ctx)
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRUnit> {
        let mut instructions: Vec<IRInstruction> = Vec::new();

        // Generate TAC for the initializer expression contained in the assignment.
        // We assume the Assignment node's `value` field holds (Expr, Position) and that
        // Expr has a `tac` method returning (temp: String, Vec<IRInstruction>).
        let (init_temp, mut init_insts) = self.assignment.0.value.0.tac(ctx);
        instructions.append(&mut init_insts);

        // Get the variable's type from the declaration. This could also be looked up in ctx.symbol_table.
        let var_type = IRType::from_type(&self.declaration.0.var_type);

        // Allocate the variable on the stack.
        let _offset = ctx.allocate_variable(&self.declaration.0.id, &var_type);

        instructions.push(IRInstruction::AllocStack {
            name: self.declaration.0.id.clone(),
            var_type,
            initial_value: Some(init_temp),
        });
        vec![IRUnit::Global(instructions)]
    }
}

