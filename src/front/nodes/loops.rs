use colored::Colorize;

use crate::front::token::Position;

use super::{body::Body, expr::Expr, node::Node, variables::VariableDeclaration};

pub struct Break;

impl Node for Break {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "Break".red(),
            width = indentation
        );
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        todo!()
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}

pub struct Continue;

impl Node for Continue {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "Break".red(),
            width = indentation
        );
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        todo!()
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}


pub struct Loop {
    pub body: (Body, Position),
}

impl Node for Loop {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "Loop".red(),
            width = indentation
        );

        for (child, pos) in &self.body.0.children {
            let pos = format!("{}:{}", pos.line, pos.index);
            print!("{}{} |", pos, " ".repeat(10 - pos.len()));
            child.display(indentation + 4);
        }
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}

pub struct WhileLoop {
    pub condition: (Expr, Position),
    pub body: (Body, Position),
}

impl Node for WhileLoop {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "While".red(),
            width = indentation
        );

        let pos = format!("{}:{}", self.condition.1.line, self.condition.1.index);
        print!("{}{} |", pos, " ".repeat(10 - pos.len()));
        self.condition.0.display(indentation + 4);

        for (child, pos) in &self.body.0.children {
            let pos = format!("{}:{}", pos.line, pos.index);
            print!("{}{} |", pos, " ".repeat(10 - pos.len()));
            child.display(indentation + 4);
        }
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}

pub struct ForLoop {
    pub iter: VariableDeclaration,
    pub condition: (Expr, Position),
    pub body: (Body, Position),
}

impl Node for ForLoop {
    fn display(&self, indentation: usize) {
        todo!()
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}

pub struct ForEachLoop {
    pub iter: Expr,
    
}
