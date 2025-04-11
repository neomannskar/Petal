use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use front::nodes::node::Node;
use front::semantic::SemanticAnalyzer;
use front::token::Position;
use middle::ir::IRContext;

mod back;
mod config;
mod front;
mod middle;

fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let config = config::PetalConfig::from_args();
    dbg!(&config);

    let src = match read_file_to_string(Path::new(&config.src)) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    println!("\n{}", src);

    let lexer = front::lexer::Lexer::new(&src);
    let tokens: Vec<(front::token::Token, Position)> = lexer.collect();

    for (token, _) in &tokens {
        dbg!(token);
    }

    let mut parser = front::parser::Parser::new(config.src.clone().to_string_lossy().into_owned(), tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("\n");
            ast.display(0);
            let analyzer = SemanticAnalyzer::new(ast);

            let analyzed_ast = analyzer.analyze();

            let mut ctx = IRContext::new();
            let ir = analyzed_ast.ir(&mut ctx);

            for inst in ir {
                println!("{:?}", inst);
            }

            // let ir = generator.generate();
            // println!("\nIntermediate Representation:\n\n{}", ir);
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
        }
    }
}
