#pragma once
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

#include "../headers/token.hpp"
#include "../headers/ast.hpp"

class Parser {
 public:
  Parser(const std::vector<Token> &_tokens);
  ~Parser();
  const std::vector<ASTNode>& ast();

  std::unordered_map<std::string, std::string> ids;
  std::vector<ASTNode> _ast;

 private:
  Token consume();
  bool expect(const TokenType &type);

  ASTNode parse_literal();
  ASTNode parse_var_call();
  ASTNode parse_fn_call_param();
  ASTNode parse_fn_call();
  ASTNode parse_ret_statement();
  ASTNode parse_statement();
  ASTNode parse_fn_body();
  ASTNode parse_fn_ret_type();
  ASTNode parse_fn_arg_type();
  ASTNode parse_fn_arg();
  ASTNode parse_arg_list();
  ASTNode parse_fn();

  ASTNode parse_pub_struct_member();
  ASTNode parse_struct_member();
  ASTNode parse_struct_body();
  ASTNode parse_struct();
  ASTNode parse_expr();

  const std::vector<Token> &tokens;
  size_t index;
  Token curr;
};
