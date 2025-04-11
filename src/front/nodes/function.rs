use crate::front::nodes::id::Identifier;
use crate::front::nodes::node::Node;
use crate::middle::ir::{IRContext, IRInstruction};

pub struct FunctionDefinition {
    pub id: Identifier,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: FunctionReturnType,
    pub body: Box<FunctionBody>,
}

impl Node for FunctionDefinition {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}-> FunctionDefinition: {}",
            "",
            self.id.name,
            width = indentation
        );
        for param in &self.parameters {
            param.display(indentation + 4);
        }
        self.return_type.display(indentation + 4);
        self.body.display(indentation + 4);
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        let mut instructions = Vec::new();

        // instructions.extend(self.id.ir(ctx));

        // Generate IR for parameters
        for param in &self.parameters {
            instructions.extend(param.ir(ctx));
        }

        // Generate IR for body
        instructions.extend(self.body.ir(ctx));

        // Add a return instruction if necessary
        if !instructions
            .iter()
            .any(|instr| matches!(instr, IRInstruction::Ret(_)))
        {
            instructions.push(IRInstruction::Ret("0".to_string()));
        }

        instructions
    }
}

pub struct FunctionParameter {
    pub id: Identifier,
    pub r#type: String,
}

impl Node for FunctionParameter {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}-> FunctionParam: {} : {}",
            "",
            self.id.name,
            self.r#type,
            width = indentation
        );
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        Vec::new()
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

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        Vec::new()
    }
}

pub struct FunctionReturnType {
    // pub r#type: Type,
    pub r#type: String,
}

impl Node for FunctionReturnType {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}-> FunctionReturnType: {}",
            "",
            self.r#type,
            width = indentation
        );
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        Vec::new()
    }
}

pub struct Return {
    // pub value: Box<dyn Node>,
    pub value: String,
}

impl Node for Return {
    fn push_child(&mut self, c: Box<dyn Node>) {}

    fn display(&self, indentation: usize) {
        println!(
            "{:>width$}-> Return: {}",
            "",
            self.value,
            width = indentation
        );
    }

    fn ir(&self, ctx: &mut IRContext) -> Vec<IRInstruction> {
        vec![IRInstruction::Ret(self.value.clone())]
    }
}
