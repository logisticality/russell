pub mod ast;

mod parse_defn;
mod parse_expr;
mod parse_stmnt;
mod parse_type;

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::Defn;
use crate::frontend::parser::parse_defn::parse_defn;

pub fn parse(tokens: Vec<Token>) -> Vec<Defn> {
    let mut parser = Parser::new(tokens);
    let mut defns = Vec::new();

    while parser.peek().kind() != TokenKind::EoF {
        match parse_defn(&mut parser) {
            Ok(defn) => defns.push(defn),
            Err(err) => unimplemented!(), // TODO - handle errors
        }
    }

    return defns;
}

pub struct Parser {
    pub tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> ParseResult<Token> {
        if self.peek().kind() == expected {
            Ok(self.tokens.next().unwrap())
        } else {
            ParseError::new(expected, self.peek().clone())
        }
    }

    pub fn expect_many(&mut self, expected: Vec<TokenKind>) -> ParseResult<Token> {
        for token in &expected {
            if self.peek().kind() == *token {
                return Ok(self.tokens.next().unwrap());
            }
        }

        ParseError::many(expected, self.peek().clone())
    }

    pub fn peek(&mut self) -> &Token {
        // EoF sentinel ensures this is always Some
        self.tokens.peek().unwrap()
    }
}

pub struct ParseError {
    expected: Vec<TokenKind>,
    actual: Token,
}

impl ParseError {
    pub fn new<A>(expected: TokenKind, actual: Token) -> ParseResult<A> {
        Err(ParseError {
            expected: vec![expected],
            actual,
        })
    }

    pub fn many<A>(expected: Vec<TokenKind>, actual: Token) -> ParseResult<A> {
        Err(ParseError { expected, actual })
    }
}

pub type ParseResult<A> = Result<A, ParseError>;
