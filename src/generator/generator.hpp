#pragma once
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <unordered_map>
#include <filesystem>

#include "../headers/ast.hpp"

class Generator {
 public:
  Generator(const std::vector<ASTNode> &ast, const std::string &raw_path);
  ~Generator();
  
  std::ofstream output;
  std::string output_file_path;

 private:
  void link(const std::string &str);
  void setup_stack_ptr();
  void return_stack_ptr();

  void fn_call_param_gen(const ASTNode &param);
  void fn_call_gen(const ASTNode &call);
  void fn_return_gen(const ASTNode &ret);
  void fn_body_gen(const ASTNode &body);
  void fn_arg_gen(const ASTNode &arg);
  void fn_arg_list_gen(const ASTNode &args);
  void fn_gen(const ASTNode &fn);

  void struct_gen(const ASTNode &struc);
  
  //                  register      functions using them
  std::unordered_map<std::string, std::vector<std::string>> registers;

  std::vector<std::string> fn_arg_ids;
  std::string curr_fn_type;
};
