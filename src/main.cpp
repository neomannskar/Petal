#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <chrono>
#include <filesystem>

#include "./lexer/lexer.hpp"
#include "./parser/parser.hpp"
// #include "./semantic analyzer/semantic-analyzer.hpp"
#include "./generator/generator.hpp"

enum class DebugCode {
  E001,
};

void log(const DebugCode &code, const std::string &info) {
  switch (code) {
    case DebugCode::E001:
      std::cout << "ERROR: Unknown argument: " << info << "\n\t'--help' for help." << std::endl;
  }
}

std::string COMPILER_PATH = "";
std::string SRC = "";
std::string OUTPUT_FILE_PATH = "";
extern std::string ASSEMBLY_TYPE;
extern std::string ASSEMBLY_TYPE_FILE_EXT;
bool PRINT_INTERNAL_PROCESS = false;

void setup_environment(int argc, char *argv[]) {
  std::vector<std::string> args;
  for (int i = 0; i < argc; ++i)
    args.push_back(argv[i]);

  COMPILER_PATH = args[0];

  for (size_t i = 1; i < args.size(); ++i) {
    if (i + 1 < args.size() && args[i] == "-o") {
      OUTPUT_FILE_PATH = args[i++];
    } else if (args[i] == "--show-internal-process" || args[i] == "--sip") {
      PRINT_INTERNAL_PROCESS = true;
    } else if (args[i] == "-asm=RP2040" || args[i] == "-") {
      ASSEMBLY_TYPE = args[i].substr(4, args[i].size());
      ASSEMBLY_TYPE_FILE_EXT = ".s";
    } else {
      if (SRC == "") {
        SRC = args[i];
      } else {
        log(DebugCode::E001, args[i]);
      }
    }
  }

  if (OUTPUT_FILE_PATH == "") {
    OUTPUT_FILE_PATH = std::filesystem::path(SRC).string();
  }
}

std::string load_file(const std::string &path) {
  std::ifstream fst(path, std::ios::binary | std::ios::ate);
  std::streamsize size = fst.tellg();
  fst.seekg(0, std::ios::beg);

  char *buffer = new char[size + 1];

  if (!buffer) {
    std::cerr << "Failed to allocate enough memory for buffer!" << std::endl;
    exit(-1);
  }

  if (!fst.read(buffer, size)) {
    std::cerr << "Failed to read file into buffer!" << std::endl;
    exit(-1);
  }

  return std::string(buffer, size);
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    std::printf("Usage: %s <path-to-file-to-compile>", argv[0]);
    return 1;
  }

  std::printf("\033[1;32m%15s \033[0m%s\n", "Building", argv[0]);
  std::printf("\033[1;32m%15s \033[0m%s\n", "Compiling", argv[1]);

  auto start_time = std::chrono::high_resolution_clock::now();

  setup_environment(argc, argv);
  std::string src = load_file(SRC);
  Lexer lexer(src);
  Parser parser(lexer.tokens());
  Generator generator(parser.ast(), OUTPUT_FILE_PATH);
  
  if (PRINT_INTERNAL_PROCESS) {
    std::cout << "============= Contents ==============\n\n" << src << "\n";
    std::cout << "\n============== Tokens ===============\n\n";
    print_tokens(lexer.tokens());
    std::cout << "\n======= Abstract Syntax Tree ========\n\n";
    print_ast(parser.ast());
    std::cout << "\n============ Generator ==============\n\n";
    std::cout << "\n============ Assembly ===============\n\n";
    std::string asm_file = load_file(generator.output_file_path.c_str());
    std::cout << asm_file << std::endl;
    std::cout << "\n=====================================\n\n";
  }

  auto end_time = std::chrono::high_resolution_clock::now();
  auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);

  std::printf("\033[1;32m%15s \033[0m%lld ms\n", "Finished", duration.count());
  return 0;
}
