use crate::middle::ir::{IRContext, IRInstruction};

pub trait Node {
    fn push_child(&mut self, c: Box<dyn Node>);
    fn display(&self, indentation: usize);
    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction>;
}
