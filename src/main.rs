use std::{fs, path::PathBuf};

use clap::Parser;
use syntax::lexer::Lexer;

use crate::cli::Command;

mod cli;

fn main() {
    match Command::parse() {
        Command::Tokenize { file } => tokenize_file(file),
    }
}

fn tokenize_file(path: PathBuf) {
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("error: failed to read file");
            eprintln!("{err:?}");
            return;
        }
    };

    let lexer = Lexer::new(&content);
    let tokens = lexer.collect_tokens();

    match tokens {
        Ok(tokens) => {
            for token in tokens {
                println!("{token:?}");
            }
        }
        Err(err) => eprintln!("error: {err:?}"),
    }
}
