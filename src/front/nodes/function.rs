use crate::front::nodes::id::Identifier;
use crate::front::nodes::node::Node;
use crate::front::token::Token;
use std::rc::Rc;

pub struct FunctionDefinition {
    pub id: Identifier,
    pub parameters: Vec<FunctionParam>,
    pub return_type: FunctionReturnType,
    pub body: Box<FunctionBody>,
}

impl Node for FunctionDefinition {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionDefinition: {}", "", self.id.name, width = indentation);
        for param in &self.parameters {
            param.display(indentation + 4);
        }
        self.return_type.display(indentation + 4);
        self.body.display(indentation + 4);
    }
}

pub struct FunctionParam {
    pub id: Identifier,
    pub r#type: String,
}

impl Node for FunctionParam {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionParam: {} : {}", "", self.id.name, self.r#type, width = indentation);
    }
}

pub struct FunctionBody {
    pub children: Vec<Box<dyn Node>>,
}

impl Node for FunctionBody {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionBody", "", width = indentation);
        for child in &self.children {
            child.display(indentation + 4);
        }
    }
}

pub struct FunctionReturnType {
    // pub r#type: Type,
    pub r#type: String,
}

impl Node for FunctionReturnType {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> FunctionReturnType: {}", "", self.r#type, width = indentation);
    }
}

pub struct Return {
    // pub value: Box<dyn Node>,
    pub value: String,
}

impl Node for Return {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!("{:>width$}-> Return: {}", "", self.value, width = indentation);
    }
}
