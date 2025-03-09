#pragma once
#include <string>
#include <vector>
#include <tuple>

enum class ASTNodeType : char {
  ERROR_NODE = -1,
  UNKNOWN,

  BINARY_EXPR,

  LITERAL,

  TERM,
  OPERATOR,

  VAR_CALL,
  FN_CALL_PARAM,
  FN_CALL,
  FN_BODY,
  FN_RET_TYPE,
  FN_ARG,
  FN_ARG_LIST,
  FN_DEF,
  FN_DECL,
  FN_RETURN,

  PUB_STRUCT_MEMBER,
  STRUCT_MEMBER,
  STRUCT_BODY,
  STRUCT_DECL,
  STRUCT_DEF,

  TYPE,
  VAR_DEF,
  VAR_DECL,
};

struct ASTNode {
  // uint32_t id;
  ASTNodeType type;
  // std::tuple<size_t, size_t> span;
  std::string value;
  std::vector<ASTNode> children;
};

void print_ast(const std::vector<ASTNode> &ast);
