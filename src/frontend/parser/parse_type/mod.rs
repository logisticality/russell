use crate::frontend::lexer::token::{Token, TokenKind};
use crate::frontend::parser::ast::{Binding, Type};
use crate::frontend::parser::{ParseError, ParseResult, Parser};

pub(super) fn parse_type(parser: &mut Parser) -> ParseResult<Type> {
    // parse a type
    let l_type = match parser.peek() {
        Token::IntType => Type::Int,
        Token::FloatType => Type::Float,
        Token::BoolType => Type::Bool,
        Token::TypeId(id) => Type::TypeId(id.clone()),
        _ => {
            return ParseError::new(TokenKind::IntType, parser.peek().clone());
            // TODO error handling here also sucks right now
        }
    };

    // if we see an arrow, parse the right-hand side of the function type
    if matches!(parser.peek(), Token::Arrow) {
        parser.expect(TokenKind::Arrow)?;
        let r_type = parse_type(parser)?;
        return Ok(Type::Fn(Box::from(l_type), Box::from(r_type)));
    }

    // otherwise, there's no right-hand side
    Ok(l_type)
}

pub(super) fn parse_binding(parser: &mut Parser) -> ParseResult<Binding> {
    let id = match parser.expect(TokenKind::Id)? {
        Token::Id(str) => str,
        _ => unreachable!(),
    };
    parser.expect(TokenKind::Colon)?;
    let id_type = parse_type(parser)?;
    Ok(Binding::new(id, id_type))
}

pub(super) fn parse_binding_list(parser: &mut Parser) -> ParseResult<Vec<Binding>> {
    unimplemented!()
}
