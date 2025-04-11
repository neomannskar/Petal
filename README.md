# Petal

Lotus compiler for embedded programming.

## Building the project

```
g++ ./src/lexer/lexer.cpp ./src/parser/parser.hpp ./src/generator/generator.cpp ./src/main.cpp -o ./bin/petal
```

## Building with UnifyAll

```c++
COMP: "g++"

ARGS:
  "./src/lexer/lexer.cpp",
  "./src/parser/parser.cpp",
  "./src/generator/generator.cpp",
  "./src/main.cpp",
  "-o",
  "./bin/petal",
```

## Compiler flags

### Select Assembly Output

* `-asm=<platform>`

### Supported platforms

1. RP2040 (Raspberry Pi Pico & Raspberry Pi Pico W)
2. x86_64 (Meant for testing)

### Example

* `-asm=RP2040`

### Internals

* `--sip` or `--show-internal-process`

### DEVELOPMENT

I was thinking, you know how the parser takes one step at a time with each token? It works, it works fine, but I don't know if it would scale well. I thought, maybe it should look ahead to try and find evidence of a pattern? Let me simplify: If lets say the parser finds: fn it will expect an id after that, and then a "(" and some args (or none) and then ")" to close it of. What if the parser seperated the work? It could find that first "(", create a new vec with references to the big one with all of the tokens (to save memory) and push tokens until the ")". Here there should be no stray ")", that would be a syntax error, so when it finds it it can be certain it is the end of the list of parameters. When it finds the ")" it starts a function which takes in a list of tokens and is suppost ot output a mini-tree of the parameters? Maybe something like parse_fn_param_list(tokens: Vec<&'a Token>) -> Result<FunctionParamList, String>, then the caller could push that entire node into its children vector. Heck, it might even dispatch a thread, hold the end of the node open and "waiting" for the child to be appended, while the main thread parses the function return type and body? What are your thoughts?

### Designing a Good IR for Lotus

Designing an IR is an art, as it balances simplicity and power. Here's how you can approach it:
1. Structure of the IR

Low-Level Abstraction: The IR should be closer to assembly than your high-level language. For example:

Use simple, three-address instructions like add r1, r2, r3 instead of complex language-specific constructs.

Avoid high-level types (e.g., structs or classes) and break them into primitives (e.g., memory offsets, pointers).

Control Flow Representation: Support basic constructs like blocks, branches, and jumps (e.g., SSA or Control Flow Graph structures).

Typed IR: If your language has strong typing, ensure the IR retains type information for analysis and optimizations.

2. Example IR Instructions

```go
; Function definition
func add(i32 a, i32 b) -> i32 {
entry:
    t1 = add a, b       ; Add two integers
    ret t1              ; Return the result
}

; If statement
if x < y:
    br cond_true
else:
    br cond_false
```

3. Key Components

Instruction Set: Define a minimal set of operations (add, sub, load, store, br, ret, etc.).

Registers/Temporaries: Use virtual registers or temporaries (e.g., t1, t2) for operands.

Control Flow Constructs: Represent loops and conditionals with basic blocks and br (branch) instructions.

## Pipeline from IR to Code Generation

Once you have an IR, the compilation pipeline looks like this:
1. Parse High-Level Code into IR

Use your parser to generate IR by translating high-level constructs (e.g., if, for, function calls) into IR instructions.

2. Optimize the IR

Implement optimization passes on the IR. Here are some common optimizations:

Constant Folding: Precompute constant expressions (e.g., x = 2 + 3 -> x = 5).

Dead Code Elimination: Remove code that doesn’t impact program output.

Common Subexpression Elimination (CSE): Reuse repeated computations.

Inlining: Replace function calls with the function body if it reduces overhead.

For these optimizations, you’ll work with data flow analysis, dependency graphs, or SSA (Static Single Assignment) form for efficiency.

3. Generate Assembly from IR

Once optimized, translate the IR into assembly for your target architecture.

Map IR operations to assembly instructions, allocate registers, and handle calling conventions.

IR Design in Practice

Here’s a sketch of how you might define a simple IR in Rust:

```rust
pub enum IRInstruction {
    Add { dest: String, lhs: String, rhs: String },
    Sub { dest: String, lhs: String, rhs: String },
    Load { dest: String, src: String },
    Store { dest: String, src: String },
    Branch { condition: String, true_label: String, false_label: String },
    Label(String),
    Ret(String),
}

pub struct IRFunction {
    pub name: String,
    pub instructions: Vec<IRInstruction>,
}

pub struct IRModule {
    pub functions: Vec<IRFunction>,
}
```

Iterative IR to Assembly Pipeline

Here’s how the process could look in your code:

High-Level to IR:

  Use IRFunction and IRInstruction to represent program components.

Optimization:

  Implement passes to transform and optimize the IRModule.

Assembly Code Generation:

  Translate IR to target-specific assembly by mapping instructions (e.g., Add -> ADD).
