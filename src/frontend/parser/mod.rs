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

    defns
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> ParseResult<()> {
        if self.peek().kind() == expected {
            self.tokens.next();
            Ok(())
        } else {
            ParseError::new(expected, self.peek().clone())
        }
    }

    pub fn expect_id(&mut self) -> ParseResult<String> {
        match self.tokens.next_if(|t| t.kind() == TokenKind::Id) {
            Some(Token::Id(name)) => Ok(name),
            _ => ParseError::new(TokenKind::Id, self.peek().clone()),
        }
    }

    pub fn expect_int(&mut self) -> ParseResult<i64> {
        match self.tokens.next_if(|t| t.kind() == TokenKind::Int) {
            Some(Token::Int(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Int, self.peek().clone()),
        }
    }

    pub fn expect_float(&mut self) -> ParseResult<f64> {
        match self.tokens.next_if(|t| t.kind() == TokenKind::Float) {
            Some(Token::Float(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Float, self.peek().clone()),
        }
    }

    pub fn expect_bool(&mut self) -> ParseResult<bool> {
        match self.tokens.next_if(|t| t.kind() == TokenKind::Bool) {
            Some(Token::Bool(val)) => Ok(val),
            _ => ParseError::new(TokenKind::Bool, self.peek().clone()),
        }
    }

    pub fn expect_typeid(&mut self) -> ParseResult<String> {
        match self.tokens.next_if(|t| t.kind() == TokenKind::TypeId) {
            Some(Token::TypeId(name)) => Ok(name),
            _ => ParseError::new(TokenKind::TypeId, self.peek().clone()),
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

    // Unconditionally consume and return the next token.
    pub(super) fn advance(&mut self) -> Token {
        // EoF sentinel ensures this is always Some
        self.tokens.next().unwrap()
    }
}

pub struct ParseError {
    // TODO - should implement Error
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
