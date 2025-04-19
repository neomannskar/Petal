use std::collections::HashMap;
use crate::middle::ir::IRInstruction;

// We assume you have defined a simple allocation type:
#[derive(Debug, Clone)]
pub enum Alloc {
    Reg(String), // e.g. "%eax"
    Spill(i32),  // e.g. a stack offset like -12 for spilled values
}

/// Extracts all virtual registers from an instruction.
/// We assume that any operand (dest, op1, or op2) that starts with "t" is a virtual register.
pub fn extract_virtual_registers(inst: &IRInstruction) -> Vec<String> {
    let mut regs = Vec::new();
    match inst {
        IRInstruction::Add { dest, op1, op2 }
        | IRInstruction::Sub { dest, op1, op2 }
        | IRInstruction::Mul { dest, op1, op2 }
        | IRInstruction::Div { dest, op1, op2 }
        | IRInstruction::Mod { dest, op1, op2 }
        | IRInstruction::And { dest, op1, op2 }
        | IRInstruction::Or  { dest, op1, op2 }
        | IRInstruction::Xor { dest, op1, op2 }
        | IRInstruction::ShiftLeft { dest, op1, op2 }
        | IRInstruction::ShiftRight { dest, op1, op2 } => {
            for reg in [dest, op1, op2].iter() {
                if reg.starts_with("t") {
                    regs.push((*reg).clone());
                }
            }
        }
        IRInstruction::Not { dest, src } => {
            if dest.starts_with("t") {
                regs.push(dest.clone());
            }
            if src.starts_with("t") {
                regs.push(src.clone());
            }
        }
        IRInstruction::Load { dest, src } => {
            if dest.starts_with("t") {
                regs.push(dest.clone());
            }
            // We assume src is often a memory operand.
        }
        IRInstruction::Store { dest, src } => {
            if src.starts_with("t") {
                regs.push(src.clone());
            }
            // dest is a memory operand.
        }
        IRInstruction::Call { dest, args, .. } => {
            if dest.starts_with("t") {
                regs.push(dest.clone());
            }
            for arg in args {
                if arg.starts_with("t") {
                    regs.push(arg.clone());
                }
            }
        }
        IRInstruction::Return { value } => {
            if value.starts_with("t") {
                regs.push(value.clone());
            }
        }
        IRInstruction::LoadConstant { dest, .. } => {
            if dest.starts_with("t") {
                regs.push(dest.clone());
            }
        }
        _ => {}
    }
    regs
}

/// A simple rewrite function that uses an allocation map from virtual registers to Alloc.
/// In a real compiler, you would inspect whether an operand is a definition or use and insert appropriate
/// load or store instructions. For our basic example, we substitute every virtual register operand with
/// its physical mapping (if it is a register) or with a memory operand string (if spilled).
pub fn rewrite_instruction(inst: IRInstruction, alloc_map: &HashMap<String, Alloc>) -> Vec<IRInstruction> {
    // Here we match on the instruction and simply substitute operands.
    // A full implementation must also insert extra load/store instructions for spilled registers.
    match inst {
        IRInstruction::Add { dest, op1, op2 } => {
            let new_dest = substitute(&dest, alloc_map);
            let new_op1 = substitute(&op1, alloc_map);
            let new_op2 = substitute(&op2, alloc_map);
            vec![IRInstruction::Add {
                dest: new_dest,
                op1: new_op1,
                op2: new_op2,
            }]
        }
        IRInstruction::Sub { dest, op1, op2 } => {
            let new_dest = substitute(&dest, alloc_map);
            let new_op1 = substitute(&op1, alloc_map);
            let new_op2 = substitute(&op2, alloc_map);
            vec![IRInstruction::Sub {
                dest: new_dest,
                op1: new_op1,
                op2: new_op2,
            }]
        }
        // … similarly for other binary operations.
        _ => vec![inst],  // For now, return the instruction unchanged.
    }
}

/// A helper function that looks up a virtual register in the alloc_map.
/// If found and allocated to a register, returns that register name;
/// if spilled, returns a memory operand (e.g. "-12(%rbp)").
fn substitute(virt: &String, alloc_map: &HashMap<String, Alloc>) -> String {
    if let Some(alloc) = alloc_map.get(virt) {
        match alloc {
            Alloc::Reg(r) => r.clone(),
            Alloc::Spill(offset) => format!("{}(%rbp)", offset),
        }
    } else {
        // If not allocated, leave virtual register unchanged.
        virt.clone()
    }
}

/// A simple register allocation and spill pass that operates on a vector of IRInstructions.
/// Note: This is a very simplified version.
/// In a real system you’d compute live ranges and build an interference graph.
pub fn perform_spill_pass(
    instructions: Vec<IRInstruction>,
    available_regs: &mut Vec<String>
) -> (Vec<IRInstruction>, HashMap<String, Alloc>) {
    let mut alloc_map: HashMap<String, Alloc> = HashMap::new();

    // First pass: extract all virtual registers from all instructions.
    for inst in &instructions {
        let virt_regs = extract_virtual_registers(inst);
        for reg in virt_regs {
            if !alloc_map.contains_key(&reg) {
                if let Some(phys) = available_regs.pop() {
                    alloc_map.insert(reg, Alloc::Reg(phys));
                } else {
                    // If no register available allocate a spill slot.
                    // Here we simply use a negative offset based on the number of spills.
                    let spill_offset = -4 * ((alloc_map.len() as i32) + 1);
                    alloc_map.insert(reg, Alloc::Spill(spill_offset));
                }
            }
        }
    }

    // Second pass: rewrite instructions.
    let mut new_insts = Vec::new();
    for inst in instructions {
        let rewritten = rewrite_instruction(inst, &alloc_map);
        new_insts.extend(rewritten);
    }
    (new_insts, alloc_map)
}
