use crate::front::nodes::node::Node;
use crate::front::nodes::operator::Operator;

pub struct BinaryExpr {
    pub op: Operator,
    pub left: Expr,
    pub right: Expr,
    pub errors: Vec<String>,
}

impl Node for BinaryExpr {
    fn push_child(&mut self, c: Box<dyn Node>) {
        
    }
}

pub enum Expr {
    Number(i64),
    Binary(Box<BinaryExpr>),
    Identifier(String),
    // etc.
}

impl Node for Expr {
    fn push_child(&mut self, c: Box<dyn Node>) {
        
    }
}
