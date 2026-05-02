use std::rc::Rc;

use crate::{
    frontend::parser::ast::Expr,
    interpreter::treewalk::{Env, types::Value},
};

pub(super) fn interp_expr(expr: Expr, env: Rc<Env>) -> Rc<Value> {
    match expr {
        Expr::Int(num) => Value::Int(num).into(),
        Expr::Float(num) => Value::Float(num).into(),
        Expr::Bool(val) => Value::Bool(val).into(),
        Expr::Id(id) => interp_id(id, env),
        Expr::Fn(binding, expr) => Value::Closure(Rc::clone(&env), binding, expr).into(),
        Expr::Neg(expr) => interp_neg(*expr, env),
        Expr::Bang(expr) => interp_bang(*expr, env),
        Expr::Call(func, args) => todo!(),
        Expr::Plus(left, right) => todo!(),
        Expr::Minus(left, right) => todo!(),
        Expr::Mult(left, right) => todo!(),
        Expr::Div(left, right) => todo!(),
        Expr::Pipe(left, right) => todo!(),
        Expr::Less(left, right) => todo!(),
        Expr::LessEq(left, right) => todo!(),
        Expr::Greater(left, right) => todo!(),
        Expr::GreaterEq(left, right) => todo!(),
        Expr::Eq(left, right) => todo!(),
        Expr::NotEq(left, right) => todo!(),
        Expr::Or(left, right) => todo!(),
        Expr::And(left, right) => todo!(),
        Expr::If(cond, then_expr, else_expr) => todo!(),
        Expr::Match(expr, arms) => todo!(),
    }
}

fn interp_id(id: String, env: Rc<Env>) -> Rc<Value> {
    match env.lookup(&id) {
        Some(val) => Rc::clone(&val),
        None => panic!("FATAL ERROR: unbound identifier {id}"),
    }
}

fn interp_neg(expr: Expr, env: Rc<Env>) -> Rc<Value> {
    match &*interp_expr(expr, env) {
        Value::Int(num) => Value::Int(-num).into(),
        Value::Float(num) => Value::Float(-num).into(),
        val => panic!("FATAL ERROR: expected numeric value, found {val:?}"),
    }
}

fn interp_bang(expr: Expr, env: Rc<Env>) -> Rc<Value> {
    match &*interp_expr(expr, env) {
        Value::Bool(val) => Value::Bool(!val).into(),
        val => panic!("FATAL ERROR: expected boolean value, found {val:?}"),
    }
}
