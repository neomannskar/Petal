#pragma once
#include <iostream>
#include <string>
#include <vector>

#include "../headers/token.hpp"

class Lexer {
 public:
  Lexer(const std::string &src);
  ~Lexer();
  const std::vector<Token>& tokens();

 private:
  std::vector<Token> _tokens;
};
