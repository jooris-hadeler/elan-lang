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
    for token in lexer {
        match token {
            Ok(token) => println!("{token:?}"),
            Err(err) => {
                eprintln!("error: {err:?}");
                break;
            }
        }
    }
}
