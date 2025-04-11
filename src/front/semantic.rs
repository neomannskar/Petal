use super::ast::Ast;

pub struct SemanticAnalyzer {
    ast: Box<Ast>,
}

impl SemanticAnalyzer {
    pub fn new(ast: Box<Ast>) -> SemanticAnalyzer {
        SemanticAnalyzer { ast }
    }

    pub fn analyze(self) -> Box<Ast> {
        // analyze
        self.ast
    }
}
