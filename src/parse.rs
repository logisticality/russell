use crate::lex::Token;
pub enum Expression {
    Int(i64),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut expressions = Vec::new();
    expressions.push(Expression::Int(10));
    return expressions;
}
