use std::{collections::HashMap, rc::Rc};

use crate::frontend::parser::ast::{Binding, Expr, Stmt};

#[derive(Debug)]
pub(super) struct Env {
    pub next: Option<Rc<Env>>,
    pub binding: (String, Rc<Value>),
}

pub(super) struct Fn {
    pub name: String,
    pub bindings: Vec<Binding>,
    pub statements: Vec<Stmt>,
}

impl Fn {
    pub(super) fn new(name: String, bindings: Vec<Binding>, statements: Vec<Stmt>) -> Self {
        Self {
            name,
            bindings,
            statements,
        }
    }
}

impl Env {
    pub(super) fn new() -> Env {
        unimplemented!()
    }

    pub(super) fn extend(&self, id: String, val: Rc<Value>) -> Rc<Env> {
        unimplemented!()
    }

    pub(super) fn lookup(&self, key: &str) -> Option<Rc<Value>> {
        if self.binding.0 == key {
            return Some(Rc::clone(&self.binding.1));
        }

        match &self.next {
            Some(env) => env.lookup(key),
            None => None,
        }
    }
}

#[derive(Debug)]
pub(super) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Closure(Rc<Env>, Binding, Box<Expr>),
    Adt(String, HashMap<String, Rc<Value>>),
}
