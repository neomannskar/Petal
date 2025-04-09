use clap::{Arg, Command};
use std::path::PathBuf;

#[derive(Debug)]
pub struct PetalConfig {
    pub src: PathBuf,
    pub output_file_name: String,
    pub debug_mode: bool,
}

impl PetalConfig {
    pub fn from_args() -> Self {
        let matches = Command::new("My Compiler")
            .version("0.1.0")
            .author("Your Name")
            .about("A custom language compiler")
            .arg(
                Arg::new("src")
                    .help("The source file to compile")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Sets the output file name")
                    .num_args(1),
            )
            .arg(
                Arg::new("debug")
                    .long("debug")
                    .help("Enables debug mode")
                    .action(clap::ArgAction::SetTrue),
            )
            .get_matches();

        let src = matches.get_one::<String>("src").expect("Source file is required")
            .into();
        let output_file_name = matches.get_one::<String>("output")
            .unwrap_or(&"a.out".to_string())
            .clone();
        let debug_mode = matches.get_flag("debug");

        PetalConfig {
            src,
            output_file_name,
            debug_mode,
        }
    }
}
