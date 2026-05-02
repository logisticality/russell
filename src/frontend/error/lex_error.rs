use std::fmt;

use crate::frontend::lexer::token::{SpannedToken, Token};

/// An error encountered during lexing.
#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub offset: usize,
}

/// The kinds of errors the lexer can produce.
#[derive(Debug)]
pub enum LexErrorKind {
    InvalidCharacter(char),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LexErrorKind::InvalidCharacter(c) => write!(f, "invalid character '{}'", c),
        }
    }
}

impl std::error::Error for LexError {}

/// Scan a token stream and collect any lexer errors.
pub fn collect_errors(tokens: &[SpannedToken]) -> Vec<LexError> {
    tokens
        .iter()
        .filter_map(|t| match &t.token {
            Token::Invalid(c) => Some(LexError {
                kind: LexErrorKind::InvalidCharacter(*c),
                offset: t.offset,
            }),
            _ => None,
        })
        .collect()
}
