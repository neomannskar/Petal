pub struct IfStatement {
    condition: Expr,
    branch: Vec<Box<dyn Node>>,
}
