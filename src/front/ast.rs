use std::{collections::HashMap, rc::Rc};

use crate::front::nodes::node::Node;

pub struct Ast {
    pub root: Vec<Box<dyn Node>>,
    pub ids: HashMap<String, Rc<Box<dyn Node>>>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            root: Vec::new(),
            ids: HashMap::new(),
        }
    }

    pub fn push_child(&mut self, c: Box<dyn Node>) {
        self.root.push(c);
    }
}
