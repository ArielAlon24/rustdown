mod parser;
mod tag;
mod tokenizer;

use parser::Parser;
use std::env;

use std::fs;
use std::process;
use tokenizer::Tokenizer;

#[derive(PartialEq)]
pub enum Mode {
    Tokenizer,
    Parser,
    Converter,
}

impl<'a> From<&'a str> for Mode {
    fn from(value: &'a str) -> Self {
        match value {
            "-t" => Self::Tokenizer,
            "-p" => Self::Parser,
            "-c" => Self::Converter,
            _ => {
                usage("Unknown mode.");
                process::exit(1);
            }
        }
    }
}

fn usage(error: &'static str) {
    eprintln!("Usage:");
    eprintln!("    rustdown <file> <mode>");
    eprintln!("");
    eprintln!("Modes (optional):");
    eprintln!("    -t    Tokenizer");
    eprintln!("    -p    Parser");
    eprintln!("    -c    Converter");
    eprintln!("");
    eprintln!("Error: {}", error);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut mode = Mode::Parser;

    if args.len() < 2 {
        usage("Path was not provided.");
        process::exit(1);
    } else if args.len() == 3 {
        mode = Mode::from(args[2].as_str());
    } else if args.len() > 3 {
        usage("Too many arguments.");
        process::exit(1);
    }

    let content = match fs::read_to_string(args.into_iter().nth(1).unwrap()) {
        Ok(c) => c,
        Err(_) => {
            panic!("Error: Invalid Path.");
        }
    };

    let tokenizer = Tokenizer::from(content.as_str());

    if mode == Mode::Tokenizer {
        for token in tokenizer {
            println!("Token: {:?}", token);
        }
    } else {
        let mut parser = Parser::from(tokenizer);
        let tags = parser.parse();
        println!("Tags: \n{:#?}", tags);
    }
}
