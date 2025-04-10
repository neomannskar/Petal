use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use front::nodes::node::Node;

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
    let tokens: Vec<front::token::Token> = lexer.collect();
    dbg!(&tokens);
    let mut parser = front::parser::Parser::new(tokens);
    if let Ok(ast) = parser.parse() {
        ast.display(0);
    } else {
        println!("Failed to parse Lotus program.");
    }
}
