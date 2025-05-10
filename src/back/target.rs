// target.rs
#[derive(Debug)]
pub enum Target {
    X86_64,
    AArch64,
    Rp2040,
}

impl Target {
    pub fn new(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "x86_64" => Self::X86_64,
            "aarch64" => Self::AArch64,
            "rp2040" => Self::Rp2040,
            _ => panic!("Unknown target architecture: {}", s),
        }
    }

    pub fn arg_registers(&self, index: usize) -> String {
        match self {
            Target::X86_64 => match index {
                0 => "rdi".to_string(),
                1 => "rsi".to_string(),
                2 => "rdx".to_string(),
                3 => "rcx".to_string(),
                4 => "r8".to_string(),
                5 => "r9".to_string(),
                _ => format!("arg{}", index), // fallback for additional params
            },
            Target::AArch64 => {
                // For example, on AArch64, the first 8 registers are used as arguments.
                match index {
                    0 => "x0".to_string(),
                    1 => "x1".to_string(),
                    2 => "x2".to_string(),
                    3 => "x3".to_string(),
                    4 => "x4".to_string(),
                    5 => "x5".to_string(),
                    6 => "x6".to_string(),
                    7 => "x7".to_string(),
                    _ => format!("arg{}", index),
                }
            }
            Target::Rp2040 => {
                // For Rp2040, adjust as needed (for example, using r0 through r3)
                match index {
                    0 => "r0".to_string(),
                    1 => "r1".to_string(),
                    2 => "r2".to_string(),
                    3 => "r3".to_string(),
                    _ => format!("arg{}", index),
                }
            }
        }
    }

    pub fn available_registers(&self) -> Vec<String> {
        match self {
            Target::X86_64 => vec![
                "%eax".to_string(),
                "%ebx".to_string(),
                "%ecx".to_string(),
                "%edx".to_string(),
                "%esi".to_string(),
                "%edi".to_string(),
            ],
            Target::AArch64 => vec![
                "x0".to_string(),
                "x1".to_string(),
                "x2".to_string(),
                "x3".to_string(),
                "x4".to_string(),
                "x5".to_string(),
            ],
            Target::Rp2040 => vec![
                "r0".to_string(),
                "r1".to_string(),
                "r2".to_string(),
                "r3".to_string(),
                // Extend based on the actual available registers for your architecture.
            ],
        }
    }

    /// Returns the default size for an Int in this target.
    pub fn default_int_size(&self) -> i32 {
        match self {
            Target::X86_64 => 4,
            Target::AArch64 => 4,
            Target::Rp2040 => 4, // adjust if necessary
        }
    }

    /// Returns the directive for declaring global symbols.
    pub fn global_directive(&self) -> &'static str {
        match self {
            Target::X86_64 => ".globl",
            Target::AArch64 => ".global",
            Target::Rp2040 => ".global",
        }
    }

    /// Returns the mnemonic for move instructions.
    pub fn mov_instruction(&self) -> &'static str {
        match self {
            Target::X86_64 => "movl",
            Target::AArch64 => "mov",
            Target::Rp2040 => "MOV", // Placeholder for your target
        }
    }

    /// Returns the mnemonic for push instructions.
    pub fn push_instruction(&self) -> &'static str {
        match self {
            // x86 uses "pushq", while aarch64 and others might need a different expansion.
            Target::X86_64 => "pushq",
            Target::AArch64 => "stp", // Note: AArch64 does not have push/pop per se.
            Target::Rp2040 => "PUSH", // Placeholder
        }
    }

    /// Returns the mnemonic for pop instructions.
    pub fn pop_instruction(&self) -> &'static str {
        match self {
            Target::X86_64 => "popq",
            Target::AArch64 => "ldp", // Again, AArch64 uses paired loads.
            Target::Rp2040 => "POP",  // Placeholder
        }
    }

    /// Returns the proper function label.
    pub fn function_label(&self, id: &str) -> String {
        match self {
            // You may decide whether to prefix underscores or not. This is just an example.
            Target::X86_64 => id.to_owned(),
            Target::AArch64 => format!("_{}", id),
            Target::Rp2040 => id.to_owned(),
        }
    }

    // You can add more helper methods for other target-specific instructions,
    // addressing modes, or calling conventions.
}
