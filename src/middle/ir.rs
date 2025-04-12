pub struct IRContext {
    temp_count: usize, // Counter for temporary register names
}

impl IRContext {
    pub fn new() -> Self {
        IRContext { temp_count: 0 }
    }

    // Allocate a new temporary register
    pub fn allocate_temp(&mut self) -> String {
        self.temp_count += 1;
        format!("t{}", self.temp_count) // Generates t1, t2, t3, ...
    }

    // Helper functions to get previous temps for binary operations
    pub fn get_last_temp(&self) -> String {
        format!("t{}", self.temp_count) // Last temp (e.g., t3)
    }

    pub fn get_second_last_temp(&self) -> String {
        format!("t{}", self.temp_count - 1) // Second-to-last temp (e.g., t2)
    }
}

#[derive(Debug)]
pub enum IRInstruction {
    Add {
        dest: String,
        lhs: String,
        rhs: String,
    },
    Sub {
        dest: String,
        lhs: String,
        rhs: String,
    },
    Load {
        dest: String,
        src: String,
    },
    Store {
        dest: String,
        src: String,
    },
    Branch {
        condition: String,
        true_label: String,
        false_label: String,
    },
    LoadVariable {
        dest: String,
        variable: String,
    },
    Label(String),
    Ret(String),
}

pub struct IRFunction {
    pub id: String, // Change to 'IRIdentifier' later
    pub instructions: Vec<IRInstruction>,
}

pub struct IRModule {
    pub functions: Vec<IRFunction>,
}
