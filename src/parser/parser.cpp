#include <iostream>
#include <string>
#include <vector>

#include "./parser.hpp"

Parser::Parser(const std::vector<Token> &_tokens): tokens(_tokens), index(0) {
  while (index < tokens.size()) {
    switch (tokens[index].type) {
      case TokenType::FN:
        curr = consume();
        _ast.push_back(parse_fn());
        continue;
      case TokenType::STRUCT:
        curr = consume();
        _ast.push_back(parse_struct());
        continue;
      default:
        break;
    }

    index++;
  }
}

Parser::~Parser() {}

const std::vector<ASTNode>& Parser::ast() {
  return _ast;
}

Token Parser::consume() {
  if (index + 1 < tokens.size()) {
    return tokens[++index];
  } else {
    return Token{TokenType::TOK_EOF, "\0"};
  }
}

bool Parser::expect(const TokenType &type) {
  if (tokens[index].type == type) {
    return true;
  }
  return false;
}

ASTNode Parser::parse_literal() {
  ASTNode literal;
  literal.type = ASTNodeType::LITERAL; // match_tok_to_node_type() later
  literal.children = {};
  literal.value = curr.lexeme;
  return literal;
}

ASTNode Parser::parse_var_call() {
  ASTNode var;
  var.type = ASTNodeType::VAR_CALL;
  var.value = curr.lexeme;
  // Check if the variable exists and matches the expected type
  return var;
}

ASTNode Parser::parse_fn_call_param() {
  ASTNode param;
  param.type = ASTNodeType::FN_CALL_PARAM;

  switch (curr.type) {
    case TokenType::ID:
      if (index + 1 < tokens.size()) {
        if (tokens[index + 1].type == TokenType::LPAR) {
          param.children.push_back(parse_fn_call());
        } else {
          param.children.push_back(parse_var_call());
        }
      } else {
        std::cerr << "ERROR(parse_fn_call_param): Index out of bounds" << std::endl;
      }
      break;
    case TokenType::INTEGER_LIT:
      param.children.push_back(parse_literal());
      break;
    case TokenType::COMMA:
      break;
    default:
      std::cerr << "IMPLEMENT(parse_fn_call):" << std::endl;
      break;
  }

  return param;
}

ASTNode Parser::parse_fn_call() {
  ASTNode call;
  call.type = ASTNodeType::FN_CALL;
  call.value = curr.lexeme;
  curr = consume(); // = '('
  curr = consume(); // = ')' / literal / var / fn_call / expression

  while (curr.type != TokenType::RPAR) {
    call.children.push_back(parse_fn_call_param());
    curr = consume();
  }

  curr = consume();

  return call;
}

ASTNode Parser::parse_ret_statement() {
  ASTNode ret;
  ret.type = ASTNodeType::FN_RETURN;

  curr = consume();
  switch (curr.type) {
    case TokenType::ID:
      if (index + 1 < tokens.size()) {
        if (tokens[index + 1].type == TokenType::LPAR) {
          ret.children.push_back(parse_fn_call());
        } else {
          ret.children.push_back(parse_var_call());
        }
      } else {
        std::cerr << "ERROR(parse_ret_statement()): Index out of bounds" << std::endl;
      }
      break;
    case TokenType::INTEGER_LIT:
      ret.children.push_back(parse_literal());
      break;
    // case TokenType::CHAR_LITERAL
    //   break;
    default:
      std::cerr << "IMPLEMENTATION(parse_ret_statement): Not implemented" << std::endl;
      break;
  }

  return ret;
}

ASTNode Parser::parse_statement() {
  ASTNode statement;
  statement.type = ASTNodeType::UNKNOWN;
  statement.children = {};

  switch (curr.type) {
    case TokenType::RET:
      statement = parse_ret_statement();
      statement.type = ASTNodeType::FN_RETURN;
      curr = consume();
      if (curr.type == TokenType::INTEGER_LIT) {
        statement.value = curr.lexeme;
      } else 
      break;
    case TokenType::ID:
      // ID
      break;
    default:
      break;
  }

  
  return statement;
}

ASTNode Parser::parse_fn_body() {
  ASTNode body;
  body.type = ASTNodeType::FN_BODY;
  body.children = {};

  if (curr.type == TokenType::RCURL) {
    curr = consume();
    return body;
  } else {
    body.children.push_back(parse_statement());
  }

  return body;
}

ASTNode Parser::parse_fn_ret_type() {
  ASTNode ret;
  ret.children = {};
  ret.type = ASTNodeType::FN_RET_TYPE;

  curr = tokens[index];
  switch (curr.type) {
    case TokenType::ID:
      // look-up table (unordered_map)
      
      // if there:
      //  set as ret-type
      // else:
      //  error
      break;
    case TokenType::CHAR:
    case TokenType::I32:
      ret.value = curr.lexeme;
      break;
    default:
      std::cerr << "ERROR(parse_fn_ret_type):\n Expected a type after '->', found " << curr.lexeme << std::endl;
      break;
  }

  return ret;
}

ASTNode Parser::parse_fn_arg_type() {
  ASTNode type;
  type.type = ASTNodeType::TYPE;
  type.value = curr.lexeme;
  return type;
}

ASTNode Parser::parse_fn_arg() {
  ASTNode arg;
  arg.value = curr.lexeme;
  arg.type = ASTNodeType::FN_ARG;
  curr = consume();
  if (curr.type == TokenType::COLON) {
    curr = consume();
    switch (curr.type) {
      case TokenType::I32:
      case TokenType::CHAR:
        arg.children.push_back(parse_fn_arg_type());
        break;
      case TokenType::ID:
        std::cerr << "IMPLEMENT(parse_fn_arg):" << std::endl;
        break;
      default:
        std::cerr << "IMPLEMENT(parse_fn_arg): Other types" << std::endl;
        break;
    }
  } else {
    std::cerr << "ERROR(parse_fn_arg): Expected ':' after identifier in argument list\n\tmeant to determine type" << std::endl;
  }

  return arg;
}

ASTNode Parser::parse_arg_list() {
  ASTNode args;
  args.children = {};
  args.type = ASTNodeType::FN_ARG_LIST;

  while (curr.type != TokenType::RPAR) {
    switch (curr.type) {
      case TokenType::ID:
        args.children.push_back(parse_fn_arg());
        break;
      case TokenType::COMMA:
        curr = consume();
        break;
      default:
        std::cerr << "ERROR(parse_arg_list): Expected identifier after '(' in argument list" << std::endl;
        // Remove later
        exit(1);
        break;
    }

    curr = consume();
  }

  return args;
}

ASTNode Parser::parse_fn() {
  ASTNode fn;
  fn.children = {};

  if (expect(TokenType::ID)) {
    fn.value = tokens[index].lexeme;
  } else {
    std::cerr << "ERROR(parse_fn):\n Expected an id after keyword 'fn', found " << curr.lexeme << std::endl;
    return fn;
  }

  curr = consume();
  if (curr.type == TokenType::LPAR) {
    curr = consume();
    fn.children.push_back(parse_arg_list());
  } else {
    std::cerr << "ERROR(parse_fn_head):\n Expected a '(' after function identifier, found " << curr.lexeme << std::endl;
  }

  curr = consume();
  switch (curr.type) {
    case TokenType::ARROW:
      curr = consume();
      fn.children.push_back(parse_fn_ret_type());
      curr = consume();
      if (expect(TokenType::LCURL)) {
        curr = consume();
        fn.type = ASTNodeType::FN_DECL;
        fn.children.push_back(parse_fn_body());
      } else if (expect(TokenType::SEMICOLON)) {
        curr = consume();
        fn.type = ASTNodeType::FN_DEF;
      } else {
        std::cerr << "ERROR: Expected a '{' or ';' after return type, found " << curr.lexeme << std::endl;
      }
      break;
    case TokenType::LCURL:
      curr = consume();
      fn.children.push_back(parse_fn_body());
      break;
    case TokenType::SEMICOLON:
      fn.type = ASTNodeType::FN_DEF;
      break;
    default:
      std::cerr << "ERROR: Expected an '->', '{' or ';' after function head, found " << curr.lexeme << std::endl;
      break;
  }

  return fn;
}

ASTNode Parser::parse_pub_struct_member() {
  ASTNode pub_mem;
  pub_mem.type = ASTNodeType::PUB_STRUCT_MEMBER;
  pub_mem.value = curr.lexeme;

  curr = consume();
  switch (curr.type) {
    case TokenType::ID:
      curr = consume();
      switch (curr.type) {
          // Check ids
        case TokenType::I32:
          pub_mem.children.push_back(ASTNode{ASTNodeType::TYPE, curr.lexeme, {}});
          break;
        default:
          std::cerr << "ERROR(parse_pub_struct_member): Expected a type after ':'" << std::endl;
          break;
      }
      break;
    default:
      std::cerr << "ERROR(parse_pub_struct_member): Expected ':' + <type>" << std::endl;
      break;
  }

  return pub_mem;
}

ASTNode Parser::parse_struct_member() {
  ASTNode member;
  member.type = ASTNodeType::STRUCT_MEMBER;
  member.value = curr.lexeme;

  

  return member;
}

ASTNode Parser::parse_struct_body() {
  ASTNode body;
  body.type = ASTNodeType::STRUCT_BODY;

  while (curr.type != TokenType::RCURL) {
    switch (curr.type) {
      case TokenType::PUB:
        curr = consume();
        body.children.push_back(parse_pub_struct_member());
        break;
      case TokenType::ID:
        body.children.push_back(parse_struct_member());
      default:
        break;
    }

    curr = consume();
  }

  return body;
}

ASTNode Parser::parse_struct() {
  ASTNode struc;
  switch (curr.type) {
    case TokenType::ID:
      struc.value = curr.lexeme;
      curr = consume();
      if (curr.type == TokenType::SEMICOLON) {
        struc.type = ASTNodeType::STRUCT_DECL;
      } else if (curr.type == TokenType::LCURL) {
        struc.type = ASTNodeType::STRUCT_DEF;
        curr = consume();
        struc.children.push_back(parse_struct_body());
      } else {
        std::cerr << "ERROR(parse_struct): Expected ';' or '{' after struct id" << std::endl;
      }
      break;
    default:
      break;
  }

  return struc;
}

ASTNode Parser::parse_expr() {
  ASTNode expr;
  // ASTNode lhs, rhs;
  expr.type = ASTNodeType::BINARY_EXPR;
  expr.value = "";
  expr.children = {};

  return expr;
}

void print_ast_i(const ASTNode &ast, size_t indentation) {
  // Indent based on depth
  for (size_t i = 0; i < indentation; ++i) {
    std::cout << "     |";
  }
  
  std::string color = "\033[0m";

  // Print node type and value
  std::cout << "-> ";
  switch (ast.type) {
    case ASTNodeType::FN_DEF:
      std::cout << "FN_DEF";
      color = "\033[32m";
      break;
    case ASTNodeType::FN_DECL:
      std::cout << "FN_DECL";
      color = "\033[32m";
      break;
    case ASTNodeType::FN_ARG_LIST:
      std::cout << "FN_ARG_LIST";
      color = "\033[33m";
      break;
    case ASTNodeType::FN_RET_TYPE:
      std::cout << "FN_RET_TYPE";
      color = "\033[33m";
      break;
    case ASTNodeType::FN_BODY:
      std::cout << "FN_BODY";
      break;
    case ASTNodeType::TERM:
      color = "\033[31m";
      std::cout << "TERM";
      break;
    case ASTNodeType::LITERAL:
      color = "\033[31m";
      std::cout << "LITERAL";
      break;
    case ASTNodeType::FN_CALL:
      color = "\033[32m";
      std::cout << "FN_CALL";
      break;
    case ASTNodeType::FN_CALL_PARAM:
      std::cout << "FN_CALL_PARAM";
      break;
    case ASTNodeType::FN_RETURN:
      std::cout << "FN_RETURN";
      break;
    case ASTNodeType::FN_ARG:
      color = "\033[32m";
      std::cout << "FN_ARG";
      break;
    case ASTNodeType::VAR_CALL:
      color = "\033[32m";
      std::cout << "VAR_CALL";
      break;
    case ASTNodeType::TYPE:
      color = "\033[33m";
      std::cout << "TYPE";
      break;
    case ASTNodeType::STRUCT_DEF:
      color = "\033[1;33m";
      std::cout << "STRUCT_DEF";
      break;
    case ASTNodeType::STRUCT_DECL:
      color = "\033[1;33m";
      std::cout << "STRUCT_DECL";
      break;
    case ASTNodeType::STRUCT_MEMBER:
      color = "\033[31m";
      std::cout << "STRUCT_MEMBER";
      break;
    case ASTNodeType::PUB_STRUCT_MEMBER:
      color = "\033[1;31";
      std::cout << "PUB_STRUCT_MEMBER";
      break;
    default:
      color = "\033[33m";
      std::cout << "<implement>";
      break;
  }
  
  std::cout << " : \"" << color << ast.value << "\033[0m" << "\"" << std::endl;
  
  // Recursively print children
  for (const auto &child : ast.children) {
    print_ast_i(child, indentation + 1);
  }

  for (size_t i = 0; i < indentation; ++i) {
    std::cout << "     |";
  }
  std::cout << "\n";
}

void print_ast(const std::vector<ASTNode> &ast) {
  for (size_t i = 0; i < ast.size(); ++i) {
    print_ast_i(ast[i], 0);
  }
}
