use colored::Colorize;

use crate::{front::{semantic::{SemanticContext, Symbol}, token::Position}, middle::ir::{IRContext, IRInstruction, IRType, IRUnit}};

use super::{node::Node, r#type::Type};

pub struct Parameter {
    pub id: String,
    pub var_type: Type,
    pub position: Position,
}

impl Node for Parameter {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}: `{}` : {:?}",
            "",
            "Parameter".blue(),
            self.id,
            self.var_type,
            width = indentation
        );
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        if ctx.lookup(&self.id).is_some() {
            return Err(format!("Parameter `{}` is already declared.", self.id));
        }
        // Insert the parameter into the symbol table.
        ctx.add_symbol(
            &self.id,
            (
                Symbol::Variable(self.var_type.clone()),
                self.position.clone(),
            ),
        );
        
        Ok(())
    }

    fn ir(&self, _ctx: &mut IRContext) -> Vec<IRUnit> {
        // Instead of a DeclareVariable instruction, we allocate stack space for the parameter.
        let inst = IRInstruction::AllocStack {
            name: self.id.clone(),
            var_type: IRType::from_type(&self.var_type),
            // Parameters usually don't come with an initializer because their
            // value is moved from the argument register during the function prologue.
            initial_value: None,
        };

        vec![IRUnit::Global(vec![inst])]
    }
}
