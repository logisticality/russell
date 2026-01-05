use crate::error::Error;

pub enum Token {
    Let,
    Id(String),
    Equals,
    Int(u64),
    Semicolon,
}

/// Given the entire program as a String, lexes it into a vector of tokens.
/// If it cannot lex, it returns an `Error.`
pub fn lex(program: &String) -> Result<Vec<Token>, Error> {
    let mut position = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while position <= program.len() {
        let (lex_result, next_position) = next_token(program, position);
        position = next_position;
        match lex_result {
            Ok(token) => tokens.push(token),
            Err(error) => return Err(error),
        }
    }

    return Ok(tokens);
}

/// Lexes the next available token in the given program, starting at the given
/// offset
fn next_token(program: &String, offset: usize) -> (Result<Token, Error>, usize) {
    // eat whitespace
    for (position, character) in &program[offset..] {}

    unimplemented!()
}
