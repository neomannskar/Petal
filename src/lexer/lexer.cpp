#include "lexer.hpp"

void enumerate(const TokenType &type) {
  switch (type) {
    case TokenType::ID:
      std::cout << "ID";
      break;
    case TokenType::PUB:
      std::cout << "PUB";
      break;
    case TokenType::FN:
      std::cout << "FN";
      break;
    case TokenType::STRUCT:
      std::cout << "STRUCT";
      break;
    case TokenType::ENUM:
      std::cout << "ENUM";
      break;
    case TokenType::IMPL:
      std::cout << "IMPL";
      break;
    case TokenType::IF:
      std::cout << "IF";
      break;
    case TokenType::ELSE:
      std::cout << "ELSE";
      break;
    case TokenType::FOR:
      std::cout << "FOR";
      break;
    case TokenType::WHILE:
      std::cout << "WHILE";
      break;
    case TokenType::INTEGER_LIT:
      std::cout << "INTEGER_LIT";
      break;
    case TokenType::PLUS:
      std::cout << "PLUS";
      break;
    case TokenType::MINUS:
      std::cout << "MINUS";
      break;
    case TokenType::ASTERISK:
      std::cout << "ASTERISK";
      break;
    case TokenType::FSLASH:
      std::cout << "FSLASH";
      break;
    case TokenType::SEMICOLON:
      std::cout << "SEMICOLON";
      break;
    case TokenType::LPAR:
      std::cout << "LPAR";
      break;
    case TokenType::RPAR:
      std::cout << "RPAR";
      break;
    case TokenType::LCURL:
      std::cout << "LCURL";
      break;
    case TokenType::RCURL:
      std::cout << "RCURL";
      break;
    case TokenType::ARROW:
      std::cout << "ARROW";
      break;
    case TokenType::CHAR:
      std::cout << "CHAR";
      break;
    case TokenType::I32:
      std::cout << "I32";
      break;
    case TokenType::RET:
      std::cout << "RET";
      break;
    case TokenType::COLON:
      std::cout << "COLON";
      break;
    case TokenType::UNKNOWN:
    default:
      std::cout << "UNKNOWN";
      break;
  }
}

bool is_num(const TokenType &type) {
  switch (type) {
    case TokenType::INTEGER_LIT:
      return true;
    default:
      return false;
  }
}

bool is_operator(const TokenType &type) {
  switch (type) {
    case TokenType::PLUS:
      return true;
    case TokenType::MINUS:
      return true;
    case TokenType::ASTERISK:
      return true;
    case TokenType::FSLASH:
      return true;
    default:
      return false;
  }
}

TokenType match(const std::string &lexeme) {
  if (lexeme == "fn") {
    return TokenType::FN;
  } else if (lexeme == "pub") {
    return TokenType::PUB;
  } else if (lexeme == "struct") {
    return TokenType::STRUCT;
  } else if (lexeme == "enum") {
    return TokenType::ENUM;
  } else if (lexeme == "impl") {
    return TokenType::IMPL;
  } else if (lexeme == "if") {
    return TokenType::IF;
  } else if (lexeme == "else") {
    return TokenType::ELSE;
  } else if (lexeme == "for") {
    return TokenType::FOR;
  } else if (lexeme == "char") {
    return TokenType::CHAR;
  } else if (lexeme == "i32") {
    return TokenType::I32;
  } else if (lexeme == "ret" || lexeme == "return") {
    return TokenType::RET;
  } else {
    return TokenType::ID;
  }
}

void print_tokens(const std::vector<Token> &_tokens) {
  for (const Token &token : _tokens) {
    std::cout << "[ ";
    enumerate(token.type);
    std::cout << " , \'" << token.lexeme << "\' ]\n";
  }
}

Lexer::Lexer(const std::string &content) {
  for (size_t i = 0; i < content.length(); ++i) {
    if (isspace(content[i]) || isblank(content[i])) {
      continue;
    } else if (isalpha(content[i]) || content[i] == '_') {
      std::string value;
      while (isalnum(content[i])) {
        value += static_cast<char>(content[i]);
        ++i;
      }
      --i;

      _tokens.emplace_back(Token{match(value), value});
    } else if (isdigit(content[i])) {
      std::string value;
      while (isdigit(content[i])) {
        value += static_cast<char>(content[i]);
        ++i;
      }
      --i;

      _tokens.emplace_back(Token {TokenType::INTEGER_LIT, value});
    } else if (!isalnum(content[i]) && !isdigit(content[i]) && isascii(content[i])) {
      std::string value(1, content[i]);
      if (content[i] == ';') {
        _tokens.emplace_back(Token{TokenType::SEMICOLON, value});
      } else if (content[i] == ':') {
        _tokens.emplace_back(Token{TokenType::COLON, value});
      } else if (content[i] == '*') {
        _tokens.emplace_back(Token{TokenType::ASTERISK, value});
      } else if (content[i] == '-') {
        if (content[i + 1] == '>') {
          _tokens.emplace_back(Token{TokenType::ARROW, "->"});
          i++;
        } else {
          _tokens.emplace_back(Token{TokenType::MINUS, value});
        }
      } else if (content[i] == '+') {
        _tokens.emplace_back(Token{TokenType::PLUS, value});
      } else if (content[i] == '(') {
        _tokens.emplace_back(Token{TokenType::LPAR, value});
      } else if (content[i] == ')') {
        _tokens.emplace_back(Token{TokenType::RPAR, value});
      } else if (content[i] == '{') {
        _tokens.emplace_back(Token{TokenType::LCURL, value});
      } else if (content[i] == '}') {
        _tokens.emplace_back(Token{TokenType::RCURL, value});
      } else if (content[i] == '/') {
        if (content[i + 1] == '*') {
          i += 2;
          while (content[i] != '*' && content[i + 1] != '/') i++;
            i++;
        } else if (content[i + 1] == '/') {
          i += 2;
          while (content[i] != '\n' && content[i] != '\r') i++;
        } else {
          _tokens.emplace_back(Token{TokenType::FSLASH, value});
        }
        
      } else {
        _tokens.emplace_back(Token{TokenType::UNKNOWN, value});
      }
    } else {
      _tokens.emplace_back(Token {TokenType::UNKNOWN, std::string(1, static_cast<char>(content[i]))});
    }
  }
}

Lexer::~Lexer() {}

const std::vector<Token>& Lexer::tokens() {
  return _tokens;
}
