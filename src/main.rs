use std::{env, fs, path::Path, process::exit};

use pewter;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let src_path = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Error: expected argument 'source path'");
            exit(1);
        }
    };
    let out_path = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Error: expected argument 'output path'");
            exit(1);
        }
    };
    let src = fs::read_to_string(src_path).unwrap();
    let module: pewter::ast::Module = match src.parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    module.codegen_to_object_file(&Path::new(out_path.as_str()));
}
