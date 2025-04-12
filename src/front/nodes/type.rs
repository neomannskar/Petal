use super::node::Node;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReturnType {
    Void,
    I32,
    I64,
    U32,
    U64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BasicType {
    FunctionDefinition(ReturnType),
    
    Void,
    I32,
    I64,
    U32,
    U64,
    // etc.
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: String,
    pub basic: Option<BasicType>,
}

impl Type {
    pub fn basic(name: &str) -> Type {
        match name {
            "i32" => Type {
                name: name.to_string(),
                basic: Some(BasicType::I32),
            },
            _ => {
                todo!("[_] Type::basic()");
            }
        }
    }
}

impl Node for Type {
    fn push_child(&mut self, c: Box<dyn Node>) {
        panic!("Node: `Type` can't bear children!");
    }

    fn display(&self, indentation: usize) {
        if let Some(basic) = &self.basic {
            println!("{:>width$}-> Type: {:?}", "", basic, width = indentation);
        } else {
            println!("{:>width$}-> Type: {}", "", self.name, width = indentation);
        }
    }

    fn analyze(&self, ctx: &mut crate::front::semantic::SemanticContext) -> Result<(), String> {
        Ok(())
    }

    fn ir(&self, ctx: &mut crate::middle::ir::IRContext) -> Vec<crate::middle::ir::IRInstruction> {
        Vec::new()
    }
}
