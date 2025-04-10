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
