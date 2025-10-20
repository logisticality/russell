use std::env;
use std::fs;

mod lex;
mod parse;
mod typecheck;

fn main() {
    parse::parse(lex::lex());
}
