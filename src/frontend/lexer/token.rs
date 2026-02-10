pub struct Token {
    pub offset: usize,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    // primitive values
    Id(String),
    Int(u64),
    Float(f64),
    Bool(bool),

    // keywords
    Echo,
    Else,
    Fn,
    If,
    Let,
    Match,
    Read,
    Return,
    Then,
    Typedef,

    // type keywords
    U64Type,
    I64Type,
    F64Type,
    BoolType,

    // punctuation
    LParen,
    RParen,
    Semicolon,
    LBrace,
    RBrace,

    // operators
    Not,
    NotEq,
    And,
    Times,
    Plus,
    Comma,
    Minus,
    Divide,
    LessThan,
    LessThanOrEq,
    Eq,
    GreaterThan,
    GreaterThanOrEq,
    Pipe,
    Or,

    // miscellaneous
    Invalid(char),
    EoF,
}
