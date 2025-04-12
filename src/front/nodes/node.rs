use crate::{
    front::semantic::SemanticContext,
    middle::ir::{IRContext, IRInstruction},
};

pub trait Node {
    fn display(&self, indentation: usize);
    fn analyze(&self, ctx: &mut SemanticContext) -> Result<(), String>;
    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction>;
}
