pub trait Node {
    fn push_child(&mut self, c: Box<dyn Node>);
    fn display(&self, indentation: usize);
}
