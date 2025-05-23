use std::collections::{HashMap, HashSet};

use super::{ast::Ast, nodes::r#type::{FunctionType, StructType, Type}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Variable(Type),
    Function(FunctionType),
    Struct(StructType),
    // etc.
}

pub struct SemanticContext {
    // Keyed by name (String) for ease of lookup.
    pub symbol_table: HashMap<String, Symbol>,
    pub current_scope: Vec<HashSet<String>>,
    pub current_function_return: Option<Type>,
}

impl SemanticContext {
    pub fn new() -> Self {
        SemanticContext {
            symbol_table: HashMap::new(),
            current_scope: vec![HashSet::new()],
            current_function_return: None,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope.push(HashSet::new());
    }

    pub fn exit_scope(&mut self) {
        self.current_scope.pop();
    }

    pub fn add_symbol(&mut self, id: &str, symbol: Symbol) {
        self.symbol_table.insert(id.to_string(), symbol);
        if let Some(scope) = self.current_scope.last_mut() {
            scope.insert(id.to_string());
        }
    }

    pub fn lookup(&self, id: &str) -> Option<&Symbol> {
        for scope in self.current_scope.iter().rev() {
            if scope.contains(id) {
                return self.symbol_table.get(id);
            }
        }
        None
    }
}

pub struct SemanticAnalyzer {
    ast: Box<Ast>,
}

impl SemanticAnalyzer {
    pub fn new(ast: Box<Ast>) -> SemanticAnalyzer {
        SemanticAnalyzer { ast }
    }

    pub fn analyze(self, ctx: &mut SemanticContext) -> Result<Box<Ast>, String> {
        // Analyze each child node of the AST
        for node in self.ast.children.iter() {
            node.analyze(ctx)?;
        }

        // dbg!(&ctx.symbol_table);

        Ok(self.ast)
    }
}
