use std::{collections::HashMap, rc::Rc};

use crate::{
    front::nodes::node::Node,
    middle::ir::{IRContext, IRInstruction},
};

use super::{semantic::SemanticContext, token::Position};

pub struct Ast {
    pub children: Vec<(Box<dyn Node>, Position)>,
    pub ids: HashMap<String, Rc<Box<dyn Node>>>,
}

impl Node for Ast {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}Abstract Syntax Tree\n",
            "",
            width = indentation
        );
        for (child, pos) in &self.children {
            let pos = format!("{}:{}", pos.line, pos.index);
            print!("{}{} |", pos, " ".repeat(10 - pos.len()));
            child.display(indentation);
        }
    }

    fn analyze(&self, _ctx: &mut SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        let mut instructions = Vec::new();

        // Generate IR for parameters
        for (child, _) in &self.children {
            instructions.extend(child.ir(ctx));
        }

        instructions
    }
}

impl Node for Box<Ast> {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}Abstract Syntax Tree\n",
            "",
            width = indentation
        );
        for (child, pos) in &self.children {
            let pos = format!("{}:{}", pos.line, pos.index);
            print!("{}{} |", pos, " ".repeat(10 - pos.len()));
            child.display(indentation);
        }
    }

    fn analyze(&self, _ctx: &mut SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        let mut instructions = Vec::new();

        // Generate IR for parameters
        for (child, _) in &self.children {
            instructions.extend(child.ir(ctx));
        }

        instructions
    }
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            children: Vec::new(),
            ids: HashMap::new(),
        }
    }
}
