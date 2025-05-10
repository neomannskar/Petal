use colored::Colorize;
use crate::{front::{semantic::SemanticContext, token::Position}, middle::ir::{IRContext, IRUnit}};
use super::{node::Node, /* statement::Statement */};

pub struct Body {
    pub children: Vec<(Box<dyn Node>, Position)>,
}

impl Node for Body {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "Body".blue(),
            width = indentation
        );
        for (child, pos) in &self.children {
            let pos = format!("{}:{}", pos.line, pos.index);
            print!("{}{} |", pos, " ".repeat(10 - pos.len()));
            child.display(indentation + 4);
        }
    }

    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String> {
        ctx.enter_scope();
        for (stmt, _) in &self.children {
            stmt.analyze(ctx)?;
        }
        ctx.exit_scope();
        Ok(())
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRUnit> {
        let mut body_units = Vec::new();

        for (child, _) in &self.children {
            let units = child.ir(ctx);
            // dbg!(&units);
            body_units.extend(units);
        }

        body_units
    }
}
