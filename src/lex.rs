pub enum Token {
    LET,
    EQUALS,
}

pub fn lex(program: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    loop {
        match  {
            (Option::Some(token), program_next) => {
                tokens.push(token);
            }
            (Option::None, _) => break,
        };
    }

    return tokens;
}

/// This function takes a string slice, finds the "next" token in the slice, and returns that token
/// and a new slice containing the rest of the program.
fn next_token(program_curr: &str) -> (Option<Token>, &str) {
    let token = match program_curr {
        s if (s.starts_with("let")) => Some(Token::LET),
        _ => Some(Token::EQUALS),
    };

    return (token, program_curr);
}
