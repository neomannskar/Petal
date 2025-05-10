use colored::Colorize;

use crate::front::token::Position;

use super::node::Node;

#[derive(Debug, Clone)]
pub enum Cast {
    Primitive((String, Position)),
    NonPrimitive((String, Position)),
}

impl Node for Cast {
    fn display(&self, indentation: usize) {
        print!("{:>width$}└───[ {}", "", "As".magenta(), width = indentation);
        
        match self {
            Cast::Primitive((ct, _pos)) => {
                // let pos = format!("{}:{}", pos.line, pos.index);
                // print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                println!(" Primitive({})", ct.magenta());
            }
            Cast::NonPrimitive((ct, _pos)) => {
                // let pos = format!("{}:{}", pos.line, pos.index);
                // print!("{}{} |", pos, " ".repeat(10 - pos.len()));
                println!(" NonPrimitiveCast({})", ct.magenta());
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
