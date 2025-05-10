use colored::Colorize;

use crate::front::token::Position;
use super::{body::Body, expr::Expr, node::Node};

pub struct IfStatement {
    pub condition: (Expr, Position),
    pub branch: (Body, Position),
}

impl Node for IfStatement {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "If".red(),
            width = indentation
        );
        
        let pos = format!("{}:{}", self.condition.1.line, self.condition.1.index);
        print!("{}{} |", pos, " ".repeat(10 - pos.len()));
        self.condition.0.display(indentation + 4);
        
        for (child, pos) in &self.branch.0.children {
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

pub struct ElseStatement {
    pub condition: Option<(Expr, Position)>,
    pub branch: (Body, Position),
}

impl Node for ElseStatement {
    fn display(&self, indentation: usize) {
        match &self.condition {
            Some(con) => {
                println!(
                    "{:>width$}└───[ {}",
                    "",
                    "Else If".red(),
                    width = indentation
                );
                
                let pos = format!("{}:{}", con.1.line, con.1.index);
                print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                con.0.display(indentation + 4);
                
                for (child, pos) in &self.branch.0.children {
                    let pos = format!("{}:{}", pos.line, pos.index);
                    print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                    child.display(indentation + 4);
                }
            }
            None => {
                println!(
                    "{:>width$}└───[ {}",
                    "",
                    "Else".red(),
                    width = indentation
                );
                
                for (child, pos) in &self.branch.0.children {
                    let pos = format!("{}:{}", pos.line, pos.index);
                    print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                    child.display(indentation + 4);
                }
            }
        }
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}
