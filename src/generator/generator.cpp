#include "generator.hpp"

std::string ASSEMBLY_TYPE = "x86_64";
std::string ASSEMBLY_TYPE_FILE_EXT = ".s";

Generator::Generator(const std::vector<ASTNode> &ast, const std::string &raw_path) {
  std::filesystem::path path(raw_path);
  output_file_path = path.string() + ASSEMBLY_TYPE_FILE_EXT;
  output.open(output_file_path, std::ios::binary);
  if (!output.is_open()) {
    std::cout << "Failed to open output file!" << output_file_path << std::endl;
    exit(1);
  }

  output << "# translation unit '" << path.filename().string() << "'\n";
  output << "  .file \"" << path.filename().string() << "\"\n";
  output << "  .text\n\n";

  for (size_t i = 0; i < ast.size(); ++i) {
    switch (ast[i].type) {
      case ASTNodeType::FN_DECL:
        fn_gen(ast[i]);
        continue;
      case ASTNodeType::FN_DEF:
        // fn_gen(ast[i]);
        break;
      case ASTNodeType::STRUCT_DECL:
        struct_gen(ast[i]);
        break;
      case ASTNodeType::STRUCT_DEF:
        break;
      default:
        std::cerr << "<implement>" << std::endl;
    }
    break;
  }

  output << "  .ident	\"tLotus: (@neomannskar, 2025)\"\n";
  output.close();
}

Generator::~Generator() {}


void Generator::link(const std::string &str) {
  if (str == "main") {
    output << "# fn 'main'\n";
    output << "  .globl main\n";
    // output << "  .def main\n"; <-- Doesn't work for some reason
    output << "main:\n";
  } else {
    output << "# fn '" << str << "'\n";
    output << "  .globl _" << str << "\n";
    // output << "  .def _" << str << "\n"; <-- Doesn't work for some reason
    output << "_" << str << ":\n";
  }
}

void Generator::setup_stack_ptr() {
  output << "# setup stack ptr\n";
  output << "  pushq %rbp\n";
  output << "  movq  %rsp, %rbp\n\n";
}

void Generator::return_stack_ptr() {
  output << "# return stack ptr\n";
  output << "  popq  %rbp\n";
  output << "  ret\n\n";
}

void Generator::fn_arg_gen(const ASTNode &arg) {
  fn_arg_ids.push_back(arg.value);

  for (size_t i = 0; i < arg.children.size(); ++i) {
    switch (arg.children[i].type) {
      case ASTNodeType::TYPE:
        if (arg.children[i].value == "i32") {
          output << "  movl  %ecx, 16(%rbp)\n";
        } else if (arg.children[i].value == "&[char]") {
          output << "  movq  %rdx, 24(%rbp)\n";
        } else {
          std::cerr << "IMPLEMENT(fn_arg_gen):" << std::endl;
        }
        break;
      default:
        std::cerr << "IMPLEMENT(fn_arg_gen):" << std::endl;
        break;
    }
  }
}

void Generator::fn_arg_list_gen(const ASTNode &args) {
  for (size_t i = 0; i < args.children.size(); ++i) {
    switch (args.children[i].type) {
      case ASTNodeType::FN_ARG:
        fn_arg_gen(args.children[i]);
        break;
      default:
        std::cerr << "IMPLEMENT(fn_arg_list_gen): " << (int)args.children[i].type << std::endl;
        break;
    }
  }
}

void Generator::fn_call_param_gen(const ASTNode &param) {
  for (size_t i = 0; i < param.children.size(); ++i) {
    switch (param.children[i].type) {
      case ASTNodeType::LITERAL:
        output << "  movl  $" << param.children[i].value << ", %ecx\n";
        break;
      default:
        std::cerr << "IMPLEMENT(fn_call_param_gen):" << std::endl;
        break;
    }
  }
}

void Generator::fn_call_gen(const ASTNode &call) {
  for (size_t i = 0; i < call.children.size(); ++i) {
    switch (call.children[i].type) {
      case ASTNodeType::FN_CALL_PARAM:
        fn_call_param_gen(call.children[i]);
        break;
      default:
        std::cerr << "IMPLEMENT(fn_call_gen):" << std::endl;
        break;
    }
  }

  output << "  call  ";
  output << (call.value == "main" ? "main" : "_" + call.value) << "\n";
}

void Generator::fn_return_gen(const ASTNode &ret) {
  switch (ret.children[0].type) {
    case ASTNodeType::FN_CALL:
      fn_call_gen(ret.children[0]);
      break;
    case ASTNodeType::LITERAL:
      // Usch
      if (isdigit(ret.children[0].value[0])) {
        output << "  movl  $" << ret.children[0].value << ", %eax\n";
      } else {
        std::cerr << "IMPLEMENT(fn_return_gen):" << std::endl;
      }
      break;
    case ASTNodeType::VAR_CALL:

      // Remove later
      output << "  movl 	16(%rbp), %eax\n";
      break;
    default:
      break;
  }
}

void Generator::fn_body_gen(const ASTNode &body) {
  for (size_t i = 0; i < body.children.size(); ++i) {
    switch (body.children[i].type) {
      case ASTNodeType::FN_RETURN:
        fn_return_gen(body.children[i]);
        break;
      case ASTNodeType::VAR_DEF:
        std::cout << "IMPLEMENT(fn_body_gen): VAR_DEF" << std::endl;
        break;
      case ASTNodeType::VAR_DECL:
        std::cout << "IMPLEMENT(fn_body_gen): VAR_DECL" << std::endl;
        break;
      default:
        std::cerr << "IMPLEMENT(fn_body_gen): " << body.children[i].value << std::endl;
        break;
    }
  }
}

void Generator::fn_gen(const ASTNode &fn) {
  curr_fn_type = fn.value;
  link(fn.value);
  setup_stack_ptr();

  for (size_t i = 0; i < fn.children.size(); ++i) {
    switch (fn.children[i].type) {
      case ASTNodeType::FN_ARG_LIST:
        if (fn.value == "main") {
          if (fn.children[i].children.size() == 2) {
            fn_arg_list_gen(fn.children[i]);
          } else if (fn.children[i].children.empty()) {
            continue;
          } else {
            std::cerr << "WARNING(fn_gen): 'main' function must take either zero or two arguments." << std::endl;
            fn_arg_list_gen(fn.children[i]);
          }
        } else {
          if (fn.children[i].children.empty()) {
            continue;
          } else {
            fn_arg_list_gen(fn.children[i]);
          }
        }
        break;
      case ASTNodeType::FN_BODY:
        fn_body_gen(fn.children[i]);
        break;
      default:
        break;
    }
  }

  return_stack_ptr();
}

void Generator::struct_gen(const ASTNode &struc) {
  if (struc.children.empty()) {
    // struc_declarations.push_back();
  } else {
    // struct_templates.push_back();
  }
}
