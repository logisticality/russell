use crate::error::Error;
use crate::lexer::Token;

pub enum Exp {
    Int(i64),
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Exp>, Error> {
    let mut exp_start = 0;
    let mut exp_end = 0;
    let mut expressions: Vec<Exp> = Vec::new();

    // Read each phrase (separated by semicolons).
    for token in &tokens {
        match token {
            Token::Semicolon => match next_expression(&tokens[exp_start..exp_end]) {
                Ok(exp) => {
                    expressions.push(exp);
                    exp_start = exp_end + 1;
                    exp_end = exp_end + 1;
                }
                Err(err) => return Err(err),
            },
            _ => exp_end = exp_end + 1,
        }
    }

    return Ok(expressions);
}

fn next_expression(tokens: &[Token]) -> Result<Exp, Error> {
    unimplemented!()
}
