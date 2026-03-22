use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::Expr;
use crate::frontend::parser::parse_type::{parse_binding, parse_binding_list};
use crate::frontend::parser::{ParseError, ParseResult, Parser};

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
enum Precedence {
    NotBinOp = isize::MIN,
    Pipe = 1, // pipe: |>
    Or = 2,   // logical or: ||
    And = 3,  // logical and: &&
    Eq = 4,   // equality: ==, !=
    Rel = 5,  // relational: <, <=, >, >=
    Add = 6,  // additive: +, -
    Mult = 7, // multiplicative: *, /
    Call = 8, // function call (postfix)
}

impl Token {
    fn prec(&self) -> Precedence {
        match self {
            Token::Times | Token::Divide => Precedence::Mult,
            Token::Plus | Token::Minus => Precedence::Add,
            Token::LessThan | Token::LessThanOrEq | Token::GreaterThan | Token::GreaterThanOrEq => Precedence::Rel,
            Token::Eq | Token::NotEq => Precedence::Eq,
            Token::And => Precedence::And,
            Token::Or => Precedence::Or,
            Token::Pipe => Precedence::Pipe,
            Token::LParen => Precedence::Call,
            _ => Precedence::NotBinOp,
        }
    }
}

pub(super) fn parse_expr(parser: &mut Parser) -> ParseResult<Expr> {
    parse_expr_prec(parser, Precedence::NotBinOp)
}

fn parse_expr_prec(parser: &mut Parser, min_prec: Precedence) -> ParseResult<Expr> {
    let mut left = parse_null_denotation(parser)?;

    loop {
        let prec = parser.peek().prec();
        if prec <= min_prec {
            break;
        }

        // function call (postfix): left(arg, ...)
        if parser.peek().kind() == TokenKind::LParen {
            left = parse_call_expr(parser, left)?;
            continue;
        }

        // Binary operator
        let op = parser.advance();
        let right = parse_expr_prec(parser, prec)?;
        left = Expr::binop(op, left, right);
    }

    Ok(left)
}

// Null denotation: atoms and prefix operators.
fn parse_null_denotation(parser: &mut Parser) -> ParseResult<Expr> {
    match parser.peek().kind() {
        TokenKind::Int | TokenKind::Float | TokenKind::Bool | TokenKind::Id => parse_atom_expr(parser),
        TokenKind::Minus | TokenKind::Not => parse_unary_expr(parser),
        TokenKind::LParen => parse_paren_expr(parser),
        TokenKind::Fn => parse_closure_expr(parser),
        TokenKind::If => parse_if_expr(parser),
        TokenKind::Match => parse_match_expr(parser),
        _ => ParseError::many(
            &[
                TokenKind::Int,
                TokenKind::Float,
                TokenKind::Bool,
                TokenKind::Id,
                TokenKind::Minus,
                TokenKind::Not,
                TokenKind::LParen,
                TokenKind::Fn,
                TokenKind::If,
                TokenKind::Match,
            ],
            parser.peek().clone(),
        ),
    }
}

fn parse_atom_expr(parser: &mut Parser) -> ParseResult<Expr> {
    match parser.peek().kind() {
        TokenKind::Int => Ok(Expr::Int(parser.expect_int()?)),
        TokenKind::Float => Ok(Expr::Float(parser.expect_float()?)),
        TokenKind::Bool => Ok(Expr::Bool(parser.expect_bool()?)),
        TokenKind::Id => Ok(Expr::Id(parser.expect_id()?)),
        _ => unreachable!(),
    }
}

fn parse_unary_expr(parser: &mut Parser) -> ParseResult<Expr> {
    match parser.advance() {
        Token::Minus => Ok(Expr::Neg(Box::new(parse_expr_prec(parser, Precedence::Mult)?))),
        Token::Not => Ok(Expr::Bang(Box::new(parse_expr_prec(parser, Precedence::Mult)?))),
        _ => unreachable!(),
    }
}

fn parse_paren_expr(parser: &mut Parser) -> ParseResult<Expr> {
    parser.expect(TokenKind::LParen)?;
    let e = parse_expr(parser)?;
    parser.expect(TokenKind::RParen)?;
    Ok(e)
}

// fn ( <binding> ) -> <expr>
fn parse_closure_expr(parser: &mut Parser) -> ParseResult<Expr> {
    parser.expect(TokenKind::Fn)?;
    parser.expect(TokenKind::LParen)?;
    let binding = parse_binding(parser)?;
    parser.expect(TokenKind::RParen)?;
    parser.expect(TokenKind::Arrow)?;
    let body = parse_expr(parser)?;
    Ok(Expr::Fn(binding, Box::new(body)))
}

// if <cond> then <then_branch> else <else_branch>
fn parse_if_expr(parser: &mut Parser) -> ParseResult<Expr> {
    parser.expect(TokenKind::If)?;
    let cond = parse_expr(parser)?;
    parser.expect(TokenKind::Then)?;
    let then_branch = parse_expr(parser)?;
    parser.expect(TokenKind::Else)?;
    let else_branch = parse_expr(parser)?;
    Ok(Expr::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch)))
}

// match <expr> { <id>(<binding>, ...) -> <expr>, ... }
fn parse_match_expr(parser: &mut Parser) -> ParseResult<Expr> {
    parser.expect(TokenKind::Match)?;
    let scrutinee = parse_expr(parser)?;
    parser.expect(TokenKind::LBrace)?;
    let arms = parse_match_arms(parser)?;
    parser.expect(TokenKind::RBrace)?;
    Ok(Expr::Match(Box::new(scrutinee), arms))
}

// <left>( <expr>, ... )
fn parse_call_expr(parser: &mut Parser, left: Expr) -> ParseResult<Expr> {
    parser.expect(TokenKind::LParen)?;

    let mut args = Vec::new();

    if parser.peek().kind() != TokenKind::RParen {
        args.push(parse_expr(parser)?);
        while parser.peek().kind() == TokenKind::Comma {
            parser.advance();
            args.push(parse_expr(parser)?);
        }
    }

    parser.expect(TokenKind::RParen)?;
    Ok(Expr::Call(Box::new(left), args))
}

// parse match arms: <id>(<binding>, ...) -> <expr>, ...
// arms are comma-separated and end at '}'.
fn parse_match_arms(
    parser: &mut Parser,
) -> ParseResult<Vec<(String, Vec<crate::frontend::parser::ast::Binding>, Expr)>> {
    let mut arms = Vec::new();

    while parser.peek().kind() != TokenKind::RBrace {
        let constructor = parser.expect_id()?;
        let bindings = parse_binding_list(parser)?;
        parser.expect(TokenKind::Arrow)?;
        let body = parse_expr(parser)?;
        arms.push((constructor, bindings, body));

        if parser.peek().kind() == TokenKind::Comma {
            parser.advance();
        } else {
            break;
        }
    }

    Ok(arms)
}
