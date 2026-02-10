use std::env;
use std::fs;
use std::process::ExitCode;

mod frontend;

fn main() -> ExitCode {
    // read the program
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(name) => name,
        None => {
            eprintln!("FATAL ERROR: No file provided.");
            return ExitCode::FAILURE;
        }
    };
    let program = match fs::read_to_string(filename) {
        Ok(program) => program,
        Err(_) => {
            eprintln!("FATAL ERROR: Cannot read the given file: {filename}.");
            return ExitCode::FAILURE;
        }
    };

    // lex the program
    let tokens = frontend::lexer::lex(&program);

    println!("{:?}", tokens);
    ExitCode::SUCCESS
}
