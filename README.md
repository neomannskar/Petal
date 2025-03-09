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
