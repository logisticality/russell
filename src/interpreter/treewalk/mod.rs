use std::rc::Rc;

use crate::frontend::parser::ast::{Binding, Defn};
use crate::interpreter::treewalk::types::{Env, Fn};

mod interp_expr;
mod interp_fn;
mod types;

pub fn interp(defns: Vec<Defn>) {
    let (global_env, fn_defs) = process_global_env(defns);
    unimplemented!()
}

fn process_global_env(defns: Vec<Defn>) -> (Rc<Env>, Vec<Fn>) {
    let mut fn_defs = Vec::new();
    let mut env = Env::new().into();
    for defn in defns {
        match defn {
            Defn::Typedef(_, items) => env = add_typedef(env, items),
            Defn::Fn(id, bindings, _, stmts) => fn_defs.push(Fn::new(id, bindings, stmts)),
        }
    }

    (env, fn_defs)
}

fn add_typedef(env: Rc<Env>, bindings: Vec<(String, Vec<Binding>)>) -> Rc<Env> {
    unimplemented!()
}
