use std::env;
use std::fs;

pub struct Args {
    pub input_file: String,
}

pub fn read_cli_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Must provide path to input file");
    }

    Args {
        input_file: args[1].clone(),
    }
}

pub fn read_input_file(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Unable to read {}", path))
}
