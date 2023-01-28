use clap::Parser;
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use pewter;

#[derive(Parser)]
#[command(about, author, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build { src: PathBuf, out: PathBuf },
}

fn build(src: &PathBuf, out: &PathBuf) {
    let src = fs::read_to_string(src).unwrap();
    let module: pewter::ast::Module = match src.parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    module.codegen_to_object_file(&out);
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Build { src, out } => build(&src, &out),
    }
}
