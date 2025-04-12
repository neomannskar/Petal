use crate::front::semantic::SemanticContext;

use super::{node::Node, r#type::Type};

pub struct VariableDeclaration {
    id: String,
    var_type: Type,
}

impl Node for VariableDeclaration {
    fn push_child(&mut self, c: Box<dyn Node>) {
        panic!("Node: `VariableDeclaration` can't bear any children!");
    }

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> VariableDeclaration: `{}`", "", self.id, width = indentation);
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        // Check if this variable was already declared in the current scope.
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Variable '{}' already declared", self.id));
        }
        // Register it in the symbol table and current scope.
        ctx.add_symbol(&self.id, self.var_type.clone());
        Ok(())
    }

    fn ir(&self, ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}
