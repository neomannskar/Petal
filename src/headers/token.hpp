#pragma once
#include <string>
#include <vector>

enum class TokenType : char {
  UNKNOWN = -1,
  TOK_EOF = 0,
  
  ID,

  FN,
  RET,
  STRUCT,
  PUB,
  ENUM,
  IMPL,

  IF,
  ELSE,
  FOR,
  WHILE,

  INTEGER_LIT,

  PLUS,
  MINUS,
  ASTERISK,
  FSLASH,

  LPAR,
  RPAR,
  LCURL,
  RCURL,

  ARROW,

  I32,
  CHAR,

  COMMA,
  SEMICOLON,
  COLON,
};

struct Token {
  TokenType type;
  std::string lexeme;
};

void enumerate(const TokenType &type);
bool is_num(const TokenType &type);
bool is_operator(const TokenType &type);
TokenType match(const std::string &lexeme);
void print_tokens(const std::vector<Token> &tokens);
