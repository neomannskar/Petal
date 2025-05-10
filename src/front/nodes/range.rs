use colored::Colorize;

use super::node::Node;

pub struct Range {
    start: String,
    end: String,
}

impl Node for Range {
    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}└───[ {}",
            "",
            "Range".red(),
            width = indentation
        );
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}
