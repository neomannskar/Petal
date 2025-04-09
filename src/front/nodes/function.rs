use std::rc::Rc;
use crate::front::nodes::node::Node;
use crate::front::nodes::id::Identifier;

pub struct Function {
    pub id: Identifier,
    pub parameters: Vec<Box<FunctionParam>>,
    pub body: Box<FunctionBody>,

    pub errors: Vec<String>
}

impl Node for Function {
    fn push_child(&mut self, c: Box<dyn Node>) {
        
    }
}

pub struct FunctionParam {
    pub id: Identifier,
    // pub r#type: Type
}

pub struct FunctionBody {
    pub children: Vec<Box<dyn Node>>
}
