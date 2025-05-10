use crate::{
    back::target::Target,
    front::nodes::r#type::{PrimitiveType, Type},
};

use std::collections::HashMap;

pub struct IRContext {
    pub temp_count: usize,
    pub label_count: usize,
    /// Maps variable names to their IR types (for type checking, etc.)
    pub symbol_table: HashMap<String, IRType>,
    /// Maps variable names to their stack offsets (relative to %rbp)
    pub stack_allocations: HashMap<String, i32>,
    /// A running counter representing the current stack offset.
    /// Typically starts at 0 and subtracts the size of each allocated variable.
    pub current_stack_offset: i32,
    pub target: Target,
}

impl IRContext {
    pub fn new(target: Target) -> Self {
        IRContext {
            temp_count: 0,
            label_count: 0,
            symbol_table: HashMap::new(),
            stack_allocations: HashMap::new(),
            current_stack_offset: 0, // could start at 0 and go negative as space is allocated.
            target,
        }
    }

    /// Allocate a new temporary register name.
    pub fn allocate_temp(&mut self) -> String {
        self.temp_count += 1;
        format!("t{}", self.temp_count)
    }

    /// Allocate a new label name.
    pub fn allocate_label(&mut self) -> String {
        self.label_count += 1;
        format!("L{}", self.label_count)
    }

    /// Allocate a variable on the stack. This method records the stack offset
    /// for a variable based on its type size.
    pub fn allocate_variable(&mut self, name: &str, var_type: &IRType) -> i32 {
        let size = var_type.size(); // e.g., IRType::Int(4) returns 4.
                                    // Update the running stack offset (note: the offset becomes more negative)
        self.current_stack_offset -= size;
        let offset = self.current_stack_offset;
        self.stack_allocations.insert(name.to_owned(), offset);
        offset
    }
}

#[derive(Debug, Clone)]
pub enum IRType {
    Void,
    Int(i32),
    Float(i32),
    Char(i32),
    Str(i32),
    Bool(i32),
    Compound(Vec<IRType>), // For compound types
}

impl IRType {
    pub fn from_str(r#type: &str) -> Self {
        match r#type {
            "int" => IRType::Int(4),
            "float" => IRType::Float(4),
            "char" => IRType::Char(1),
            "string" => IRType::Str(8), // Assuming a pointer size for strings
            "bool" => IRType::Bool(1),
            _ => panic!("Unknown type: {}", r#type),
        }
    }

    pub fn from_type(r#type: &Type) -> Self {
        match r#type {
            Type::Primitive(t) => {
                match t {
                    PrimitiveType::I32 => IRType::Int(4),
                    PrimitiveType::F32 => IRType::Float(4),
                    PrimitiveType::Char => IRType::Char(1),
                    PrimitiveType::Str => IRType::Str(8), // Assuming a pointer size for strings
                    PrimitiveType::Bool => IRType::Bool(1),
                    _ => panic!("Unknown type: {:?}", t),
                }
            }
            _ => {
                todo!("Unknown type, implement compound types handling")
                // panic!("Unknown type: {:?}", r#type)
            }
        }
    }

    pub fn size(&self) -> i32 {
        match self {
            IRType::Void => 0,
            IRType::Int(width) => *width,
            IRType::Float(width) => *width,
            IRType::Char(width) => *width,
            IRType::Str(width) => *width,
            IRType::Bool(width) => *width,
            IRType::Compound(types) => types.iter().map(|ty| ty.size()).sum(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
    // Arithmetic in TAC style:
    Add {
        dest: String,
        op1: String,
        op2: String,
    },
    Sub {
        dest: String,
        op1: String,
        op2: String,
    },
    Mul {
        dest: String,
        op1: String,
        op2: String,
    },
    Div {
        dest: String,
        op1: String,
        op2: String,
    },
    Mod {
        dest: String,
        op1: String,
        op2: String,
    },

    // Boolean/logical/bitwise operations:
    And {
        dest: String,
        op1: String,
        op2: String,
    },
    Or {
        dest: String,
        op1: String,
        op2: String,
    },
    Xor {
        dest: String,
        op1: String,
        op2: String,
    },

    // Shift operations:
    ShiftLeft {
        dest: String,
        op1: String,
        op2: String,
    },
    ShiftRight {
        dest: String,
        op1: String,
        op2: String,
    },

    // Unary operator:
    Not {
        dest: String,
        src: String,
    },

    // Memory operations for load/store:
    Load {
        dest: String,
        src: String,
    },
    Store {
        dest: String,
        src: String,
    },

    // A function call returns its value in a “dest” temporary.
    Call {
        dest: String,
        fn_id: String,
        args: Vec<String>,
    },

    // Branch instructions, typically comparing temporaries.
    Branch {
        condition: String,
        true_label: String,
        false_label: String,
    },
    Return {
        value: String,
    },

    // Instead of a plain DeclareVariable, we instead allocate space and initialize it.
    AllocStack {
        name: String,     // variable identifier
        var_type: IRType, // the type (for stack layout or later type information)
        // Optionally, we initialize it immediately.
        initial_value: Option<String>,
    },

    // A simple assignment that stores a computed temporary into a variable.
    Assign {
        dest: String,
        src: String,
    },

    // Labels for jumps and branch targets.
    Label(String),

    // Constant loading instruction.
    LoadConstant {
        dest: String,
        value: i64,
    },
}

#[derive(Debug)]
pub struct IRFunction {
    pub id: String, // Change to 'IRIdentifier' later
    pub instructions: Vec<IRInstruction>,
}

#[derive(Debug)]
pub enum IRUnit {
    Global(Vec<IRInstruction>),
    Function(IRFunction),
    // In the future you might add:
    // TypeDef(IRTypeDef),
    // ImplBlock(IRImplBlock),
}

#[derive(Debug)]
pub struct IRModule {
    pub globals: Vec<IRInstruction>,
    pub functions: Vec<IRFunction>,
    // You could add more fields later (e.g., types, impls, etc.)
}

impl IRModule {
    pub fn new() -> Self {
        IRModule {
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }
}

pub fn flatten_units(units: Vec<IRUnit>) -> Vec<IRInstruction> {
    let mut result = Vec::new();
    for unit in units {
        match unit {
            IRUnit::Global(instrs) => result.extend(instrs),
            IRUnit::Function(func) => result.extend(func.instructions),
        }
    }
    result
}
