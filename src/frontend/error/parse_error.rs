use std::fmt;

use crate::frontend::lexer::token::{SpannedToken, Token, TokenKind};

#[derive(Debug)]
pub struct ParseError {
    pub expected: Vec<TokenKind>,
    pub actual: Token,
    pub offset: usize,
}

impl ParseError {
    pub fn new<A>(expected: TokenKind, actual: &SpannedToken) -> ParseResult<A> {
        Err(ParseError {
            expected: vec![expected],
            actual: actual.token.clone(),
            offset: actual.offset,
        })
    }

    pub fn many<A>(expected: &[TokenKind], actual: &SpannedToken) -> ParseResult<A> {
        Err(ParseError {
            expected: expected.to_vec(),
            actual: actual.token.clone(),
            offset: actual.offset,
        })
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.expected.as_slice() {
            [single] => write!(f, "expected {}, found {}", single, self.actual.kind()),
            many => {
                write!(f, "expected one of ")?;
                for (i, kind) in many.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", kind)?;
                }
                write!(f, "; found {}", self.actual.kind())
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<A> = Result<A, ParseError>;
