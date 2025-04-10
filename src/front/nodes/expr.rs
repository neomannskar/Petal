use crate::front::nodes::node::Node;
use crate::front::nodes::operator::Operator;

pub struct BinaryExpr {
    pub op: Operator,
    pub left: Expr,
    pub right: Expr,
}

impl Node for BinaryExpr {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> BinaryExpr: Operator({:?})", "", self.op, width = indentation);
        self.left.display(indentation + 4);
        self.right.display(indentation + 4);
    }
}

pub enum Expr {
    Number(i64),
    Binary(Box<BinaryExpr>),
    Identifier(String),
    // etc.
}

impl Node for Expr {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        match self {
            Expr::Number(value) => {
                println!("{:>width$}-> Expr: Number({})", "", value, width = indentation);
            }
            Expr::Binary(binary_expr) => {
                println!("{:>width$}-> Expr: Binary", "", width = indentation);
                binary_expr.display(indentation + 4);
            }
            Expr::Identifier(name) => {
                println!("{:>width$}-> Expr: Identifier({})", "", name, width = indentation);
            }
        }
    }
}
