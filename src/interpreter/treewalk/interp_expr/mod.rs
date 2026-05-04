use std::rc::Rc;

use crate::{
    frontend::parser::ast::Expr,
    interpreter::treewalk::{Env, types::Value},
};

enum ArithOp { Plus, Minus, Mult, Div }
enum CmpOp { Less, LessEq, Greater, GreaterEq, Eq, NotEq }
enum BoolOp { Or, And }

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
        Expr::Plus(left, right) => interp_arith_binop(*left, *right, env, ArithOp::Plus),
        Expr::Minus(left, right) => interp_arith_binop(*left, *right, env, ArithOp::Minus),
        Expr::Mult(left, right) => interp_arith_binop(*left, *right, env, ArithOp::Mult),
        Expr::Div(left, right) => interp_arith_binop(*left, *right, env, ArithOp::Div),
        Expr::Pipe(left, right) => todo!(),
        Expr::Less(left, right) => interp_cmp_binop(*left, *right, env, CmpOp::Less),
        Expr::LessEq(left, right) => interp_cmp_binop(*left, *right, env, CmpOp::LessEq),
        Expr::Greater(left, right) => interp_cmp_binop(*left, *right, env, CmpOp::Greater),
        Expr::GreaterEq(left, right) => interp_cmp_binop(*left, *right, env, CmpOp::GreaterEq),
        Expr::Eq(left, right) => interp_cmp_binop(*left, *right, env, CmpOp::Eq),
        Expr::NotEq(left, right) => interp_cmp_binop(*left, *right, env, CmpOp::NotEq),
        Expr::Or(left, right) => interp_bool_binop(*left, *right, env, BoolOp::Or),
        Expr::And(left, right) => interp_bool_binop(*left, *right, env, BoolOp::And),
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

fn interp_call(func: Expr, args: Vec<Expr>, env: Rc<Env>) -> Rc<Value> {
    todo!()
}

fn interp_arith_binop(left: Expr, right: Expr, env: Rc<Env>, op: ArithOp) -> Rc<Value> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Int(l), Value::Int(r)) => Value::Int(match op {
            ArithOp::Plus => l + r,
            ArithOp::Minus => l - r,
            ArithOp::Mult => l * r,
            ArithOp::Div => l / r,
        }).into(),
        (Value::Float(l), Value::Float(r)) => Value::Float(match op {
            ArithOp::Plus => l + r,
            ArithOp::Minus => l - r,
            ArithOp::Mult => l * r,
            ArithOp::Div => l / r,
        }).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}

fn interp_cmp_binop(left: Expr, right: Expr, env: Rc<Env>, op: CmpOp) -> Rc<Value> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Int(l), Value::Int(r)) => Value::Bool(match op {
            CmpOp::Less => l < r,
            CmpOp::LessEq => l <= r,
            CmpOp::Greater => l > r,
            CmpOp::GreaterEq => l >= r,
            CmpOp::Eq => l == r,
            CmpOp::NotEq => l != r,
        }).into(),
        (Value::Float(l), Value::Float(r)) => Value::Bool(match op {
            CmpOp::Less => l < r,
            CmpOp::LessEq => l <= r,
            CmpOp::Greater => l > r,
            CmpOp::GreaterEq => l >= r,
            CmpOp::Eq => l == r,
            CmpOp::NotEq => l != r,
        }).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}

fn interp_bool_binop(left: Expr, right: Expr, env: Rc<Env>, op: BoolOp) -> Rc<Value> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Bool(l), Value::Bool(r)) => Value::Bool(match op {
            BoolOp::Or => *l || *r,
            BoolOp::And => *l && *r,
        }).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}
