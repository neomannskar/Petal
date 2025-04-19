use crate::{
    front::nodes::node::Node,
    middle::ir::{IRContext, IRModule, IRUnit},
};

use super::{semantic::SemanticContext, token::Position};

pub struct Ast {
    pub children: Vec<(Box<dyn Node>, Position)>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            children: Vec::new(),
        }
    }

    pub fn display(&self, indentation: usize) {
        println!(
            "{:>width$}Abstract Syntax Tree\n",
            "",
            width = indentation
        );
        for (child, pos) in &self.children {
            let pos_str = format!("{}:{}", pos.line, pos.index);
            print!("{}{} |", pos_str, " ".repeat(10 - pos_str.len()));
            child.display(indentation);
        }
    }

    pub fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        for (child, _) in &self.children {
            child.analyze(ctx)?;
        }
        Ok(())
    }

    pub fn ir(&self, ctx: &mut IRContext) -> IRModule {
        let mut globals = Vec::new();
        let mut functions = Vec::new();
        
        for (child, _) in &self.children {
            let units = child.ir(ctx);
            for unit in units {
                match unit {
                    IRUnit::Global(instrs) => globals.extend(instrs),
                    IRUnit::Function(func)  => functions.push(func),
                }
            }
        }
        
        IRModule { globals, functions }
    }
}
