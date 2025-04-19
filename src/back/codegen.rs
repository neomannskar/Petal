use crate::middle::ir::{IRModule, IRInstruction};
use super::target::Target;

pub struct Generator {
    pub ir: IRModule,
    pub target: Target,
}

impl Generator {
    pub fn new(ir: IRModule, target: Target) -> Generator {
        Generator {
            ir,
            target: target,
        }
    }

    pub fn generate(&mut self) -> String {
        let mut asm = String::new();
        asm.push_str(".section .text\n\n");
    
        // Emit globals.
        for inst in &self.ir.globals {
            asm.push_str(&self.generate_instruction(inst));
            asm.push_str("\n");
        }
    
        // Emit functions.
        for function in &self.ir.functions {
            // Use the target API to get the global directive and function label.
            asm.push_str(
                format!(
                    "    {} {}\n",
                    self.target.global_directive(),
                    function.id
                )
                .as_str(),
            );
            asm.push_str(
                format!("{}:\n", self.target.function_label(&function.id)).as_str(),
            );
    
            // Function prologue.
            asm.push_str(format!("    {}   %rbp\n", self.target.push_instruction()).as_str());
            asm.push_str("    movq    %rsp, %rbp\n");
    
            for inst in &function.instructions {
                asm.push_str(&self.generate_instruction(inst));
                asm.push_str("\n");
            }
    
            // Function epilogue.
            asm.push_str(format!("    {}    %rbp\n", self.target.pop_instruction()).as_str());
            asm.push_str("    ret\n\n");
        }
    
        asm
    }
    
    /// Translates an IRInstruction into target-specific assembly code.
    fn generate_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Add { dest, op1, op2 } => {
                // Compute dest = op1 + op2.
                format!(
                    "    movl {}, {}\n    addl {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::Sub { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    subl {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::Mul { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    imull {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::Div { dest, op1, op2 } => {
                // For x86 division the dividend must be in %eax and the remainder is in %edx.
                format!(
                    "    movl {}, %eax\n    cltd\n    idivl {}\n    movl %eax, {}",
                    op1, op2, dest
                )
            }
            IRInstruction::Mod { dest, op1, op2 } => {
                format!(
                    "    movl {}, %eax\n    cltd\n    idivl {}\n    movl %edx, {}",
                    op1, op2, dest
                )
            }
            IRInstruction::And { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    andl {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::Or { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    orl {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::Xor { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    xorl {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::ShiftLeft { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    shll {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::ShiftRight { dest, op1, op2 } => {
                format!(
                    "    movl {}, {}\n    shrl {}, {}",
                    op1, dest, op2, dest
                )
            }
            IRInstruction::Not { dest, src } => {
                format!(
                    "    movl {}, {}\n    notl {}",
                    src, dest, dest
                )
            }
            IRInstruction::Load { dest, src } => {
                // Here, src is assumed to be complete—e.g. a memory operand like "-4(%rbp)".
                format!("    movl {}, {}", src, dest)
            }
            IRInstruction::Store { dest, src } => {
                // Here, dest is taken as a complete memory operand.
                format!("    movl {}, {}", src, dest)
            }
            IRInstruction::Call { dest, fn_id, args } => {
                // Push arguments in reverse order, call the function, then move the return value.
                let mut asm = String::new();
                for arg in args.iter().rev() {
                    asm.push_str(&format!("    pushl {}\n", arg));
                }
                asm.push_str(&format!("    call {}\n", fn_id));
                asm.push_str(&format!("    movl %eax, {}", dest));
                asm
            }
            IRInstruction::Branch { condition, true_label, false_label } => {
                format!(
                    "    cmpl $0, {}\n    jne {}\n    jmp {}",
                    condition, true_label, false_label
                )
            }
            IRInstruction::Return { value } => {
                format!("    movl {}, %eax", value)
            }
            IRInstruction::AllocStack { name, var_type, initial_value } => {
                // We emit a comment. In a complete system, this would be replaced by information
                // from your IRContext's allocation mapping.
                if let Some(val) = initial_value {
                    format!("    # AllocStack: {} of type {:?} with initializer {}", name, var_type, val)
                } else {
                    format!("    # AllocStack: {} of type {:?} without initializer", name, var_type)
                }
            }
            IRInstruction::Assign { dest, src } => {
                format!("    movl {}, {}", src, dest)
            }
            IRInstruction::Label(label) => format!("{}:", label),
            IRInstruction::LoadConstant { dest, value } => {
                format!("    movl ${}, {}", value, dest)
            }
            _ => format!("    # Unimplemented instruction: {:?}", inst),
        }
    }
}
