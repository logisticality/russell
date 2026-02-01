use std::env;
use std::fs;
use std::process::ExitCode;

mod error;
mod lexer;
mod parser;

fn main() -> ExitCode {
    // read the program
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let program = match fs::read_to_string(filename) {
        Ok(program) => program,
        Err(_) => {
            eprintln!("FATAL ERROR: Cannot read the given file: {filename}.");
            return ExitCode::FAILURE;
        }
    };

    // lex the program
    let tokens = match lexer::lex(&program) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("FATAL ERROR: {}", error::report_error(err, &program));
            return ExitCode::FAILURE;
        }
    };

    // parse the program
    let exps = match parser::parse(tokens) {
        Ok(exps) => exps,
        Err(err) => {
            eprintln!("FATAL ERROR: {}", error::report_error(err, &program));
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
