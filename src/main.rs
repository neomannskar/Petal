use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use front::nodes::node::Node;
use front::semantic::{SemanticAnalyzer, SemanticContext};
use front::token::Position;
use middle::ir::IRContext;

mod back;
mod config;
mod front;
mod middle;

macro_rules! here {
    () => {
        println!(
            "Execution passed through here:\n\tfile: {}\n\tline: {}",
            file!(),
            line!()
        )
    };
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let config = config::PetalConfig::from_args();
    // dbg!(&config);

    let src = match read_file_to_string(Path::new(&config.src)) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    println!("\n{}", src);

    let lexer = front::lexer::Lexer::new(&src);
    let tokens: Vec<(front::token::Token, Position)> = lexer.lex();

    /*
    for (token, _) in &tokens {
        println!("{:?}", token);
    }
    */

    let mut ctx = SemanticContext::new();

    let mut parser =
        front::parser::Parser::new(config.src.clone().to_string_lossy().into_owned(), tokens);
    match parser.parse(&mut ctx) {
        Ok(ast) => {
            ast.display(0);
            println!("");

            let analyzer = SemanticAnalyzer::new(ast);

            match analyzer.analyze(&mut ctx) {
                Ok(analyzed_ast) => {
                    println!("Semantic analysis successful!");
                    
                    let mut ctx = IRContext::new();
                    let ir = analyzed_ast.ir(&mut ctx);

                    for inst in ir {
                        println!("{:?}", inst);
                    }
                }
                Err(e) => {
                    eprintln!("Semantic analysis failed: {}", e);
                }
            }
            
            /*
            let mut s = config.src.clone().to_string_lossy().into_owned();
            s.push_str(".s");
            let mut output_file = File::create(s).unwrap();

            /*

            .section .text
                .globl main
            main:
                pushq  %rbp
                movq   %rsp, %rbp
                movl   $0, %eax
                popq   %rbp
                ret

            */

            let asm = String::from(
                "    .text
    .globl  main
main:
    pushq   %rbp
    movq    %rsp, %rbp
    movl    $0, %eax
    popq    %rbp
    ret
",
            );

            if let Ok(_) = output_file.write_all(asm.as_bytes()) {
                println!("Successfully wrote to .s file!");
            }
            */
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
        }
    }
}
