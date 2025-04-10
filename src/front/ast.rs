use std::{collections::HashMap, rc::Rc};

use crate::front::nodes::node::Node;

pub struct Ast {
    pub children: Vec<Box<dyn Node>>,
    pub ids: HashMap<String, Rc<Box<dyn Node>>>,
}

impl Node for Ast {
    fn push_child(&mut self, c: Box<dyn Node>) {
        self.children.push(c);
    }

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> AST Root", "", width = indentation);
        for child in &self.children {
            child.display(indentation + 4);
        }
    }
}

impl Node for Box<Ast> {
    fn push_child(&mut self, c: Box<dyn Node>) {
        self.children.push(c);
    }

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> AST Root", "", width = indentation);
        for child in &self.children {
            child.display(indentation + 4);
        }
    }
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            children: Vec::new(),
            ids: HashMap::new(),
        }
    }

    pub fn push_child(&mut self, c: Box<dyn Node>) {
        self.children.push(c);
    }
}
