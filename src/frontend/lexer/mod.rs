pub mod token;

use crate::frontend::lexer::token::{Token, TokenType};

// reserved keywords
const KEYWORDS: [(&str, TokenType); 12] = [
    ("echo", TokenType::Echo),
    ("else", TokenType::Else),
    ("false", TokenType::Bool(false)),
    ("fn", TokenType::Fn),
    ("if", TokenType::If),
    ("let", TokenType::Let),
    ("match", TokenType::Match),
    ("read", TokenType::Read),
    ("return", TokenType::Return),
    ("then", TokenType::Then),
    ("true", TokenType::Bool(true)),
    ("typedef", TokenType::Typedef),
];

// type keywords
const TYPES: [(&str, TokenType); 4] = [
    ("u64", TokenType::U64Type),
    ("i64", TokenType::I64Type),
    ("f64", TokenType::F64Type),
    ("bool", TokenType::BoolType),
];

// operators
const OPERATORS: [(&str, TokenType); 20] = [
    // two-char ops
    ("!=", TokenType::NotEq),
    ("&&", TokenType::And),
    ("<=", TokenType::LessThanOrEq),
    ("==", TokenType::Eq),
    (">=", TokenType::GreaterThanOrEq),
    ("|>", TokenType::Pipe),
    ("||", TokenType::Or),
    // one-char ops
    ("!", TokenType::Not),
    ("(", TokenType::LParen),
    (")", TokenType::RParen),
    ("*", TokenType::Times),
    ("+", TokenType::Plus),
    (",", TokenType::Comma),
    ("-", TokenType::Minus),
    ("/", TokenType::Divide),
    (";", TokenType::Semicolon),
    ("<", TokenType::LessThan),
    (">", TokenType::GreaterThan),
    ("{", TokenType::LBrace),
    ("}", TokenType::RBrace),
];

/// Given the entire program as a string, lexes it into a vector of tokens.
pub fn lex(program: &str) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut token;
    let mut rest_program = program;
    let mut done_lexing = false;

    while !done_lexing {
        (token, rest_program) = next_token(rest_program);
        done_lexing = matches!(token, TokenType::EoF);
        tokens.push(token);
    }

    tokens
}

// Lexes the next token in the given program.
// Returns the token, and the rest of the program, which has not been lexed.
fn next_token(program: &str) -> (TokenType, &str) {
    // eat whitespace at the start, and use first char to determine token type
    let program = program.trim_start();
    let first_char = match program.chars().next() {
        Some(c) => c,
        None => return (TokenType::EoF, program),
    };

    // determine if the token is an operator
    for (op_str, op_token) in OPERATORS {
        if program.starts_with(op_str) {
            return (op_token, &program[op_str.len()..]);
        }
    }

    // determine if the token is a float/int
    if is_digit(first_char) {
        return read_num(program);
    }

    // determine if the token is a keyword or variable
    if is_letter(first_char) {
        return read_ident(program);
    }

    // otherwise, the token is invalid
    (TokenType::Invalid(first_char), &program[1..])
}

fn read_num(program: &str) -> (TokenType, &str) {
    // greedily grab all characters until we see something that's not a digit
    let mut first_non_digit = program.len();
    for (index, char) in program.char_indices() {
        if !is_digit(char) {
            first_non_digit = index;
            break;
        }
    }
    let digits = &program[..first_non_digit];
    let rest = &program[first_non_digit..];

    match digits.find('.') {
        Some(_) => (TokenType::Float(digits.parse::<f64>().unwrap()), rest),
        None => (TokenType::Int(digits.parse::<u64>().unwrap()), rest),
    }
}

fn is_digit(c: char) -> bool {
    (c > '0') || (c < '9') || (c == '.')
}

fn read_ident(program: &str) -> (TokenType, &str) {
    // greedily grab all characters until we see something that's not a letter
    let mut first_non_letter = program.len();
    for (index, char) in program.char_indices() {
        if !is_letter(char) {
            first_non_letter = index;
            break;
        }
    }
    let ident = &program[..first_non_letter];
    let rest = &program[first_non_letter..];

    // check against keywords, fallback to identifier (varable) if no match
    for (keyword_str, keyword_token) in KEYWORDS {
        if ident == keyword_str {
            return (keyword_token, rest);
        }
    }

    for (type_str, type_token) in TYPES {
        if ident == type_str {
            return (type_token, rest);
        }
    }

    (TokenType::Id(ident.to_string()), rest)
}

// determines whether a character is a letter
fn is_letter(c: char) -> bool {
    (c > 'a' && c < 'z') || (c > '0' && c < '9')
}
