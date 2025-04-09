use std::io::{Result, Read};
use std::path::Path;
use std::fs::File;

mod config;
mod front;
mod middle;
mod back;

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
    let mut parser = front::parser::Parser::new(&tokens);
    let ast = parser.parse();

    dbg!(tokens);
}
