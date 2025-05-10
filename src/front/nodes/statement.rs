use super::node::Node;

pub struct Statement(Box<dyn Node>);

impl Node for Statement {
    fn display(&self, _indentation: usize) {
        todo!()
    }

    fn analyze(&self, _ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, _ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRUnit> {
        Vec::new()
    }
}
