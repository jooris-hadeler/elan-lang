use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "elanc", about = "ELAN Compiler")]
pub enum Command {
    Tokenize { file: PathBuf },
}
