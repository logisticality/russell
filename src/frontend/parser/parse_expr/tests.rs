use crate::frontend::lexer::lex;
use crate::frontend::parser::ast::{Binding, Expr, Type};
use crate::frontend::parser::Parser;

fn parser_from(input: &str) -> Parser {
    Parser::new(lex(input))
}

fn parse(input: &str) -> Expr {
    let mut p = parser_from(input);
    super::parse_expr(&mut p).unwrap()
}

/// Shorthand for Box::new
fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

// ─── atoms ──────────────────────────────────────────────────────────

#[test]
fn int_literal() {
    assert_eq!(parse("42"), Expr::Int(42));
}

#[test]
fn zero() {
    assert_eq!(parse("0"), Expr::Int(0));
}

#[test]
fn float_literal() {
    assert_eq!(parse("3.14"), Expr::Float(3.14));
}

#[test]
fn true_literal() {
    assert_eq!(parse("true"), Expr::Bool(true));
}

#[test]
fn false_literal() {
    assert_eq!(parse("false"), Expr::Bool(false));
}

#[test]
fn identifier() {
    assert_eq!(parse("foo"), Expr::Id("foo".into()));
}

// ─── unary operators ────────────────────────────────────────────────

#[test]
fn negate_int() {
    assert_eq!(parse("-1"), Expr::Neg(b(Expr::Int(1))));
}

#[test]
fn negate_identifier() {
    assert_eq!(parse("-x"), Expr::Neg(b(Expr::Id("x".into()))));
}

#[test]
fn bang_bool() {
    assert_eq!(parse("!true"), Expr::Bang(b(Expr::Bool(true))));
}

#[test]
fn bang_identifier() {
    assert_eq!(parse("!x"), Expr::Bang(b(Expr::Id("x".into()))));
}

#[test]
fn double_negate() {
    assert_eq!(parse("--1"), Expr::Neg(b(Expr::Neg(b(Expr::Int(1))))));
}

#[test]
fn double_bang() {
    assert_eq!(
        parse("!!true"),
        Expr::Bang(b(Expr::Bang(b(Expr::Bool(true)))))
    );
}

// ─── binary operators ───────────────────────────────────────────────

#[test]
fn addition() {
    assert_eq!(
        parse("1 + 2"),
        Expr::Plus(b(Expr::Int(1)), b(Expr::Int(2)))
    );
}

#[test]
fn subtraction() {
    assert_eq!(
        parse("3 - 1"),
        Expr::Minus(b(Expr::Int(3)), b(Expr::Int(1)))
    );
}

#[test]
fn multiplication() {
    assert_eq!(
        parse("2 * 3"),
        Expr::Mult(b(Expr::Int(2)), b(Expr::Int(3)))
    );
}

#[test]
fn division() {
    assert_eq!(
        parse("6 / 2"),
        Expr::Div(b(Expr::Int(6)), b(Expr::Int(2)))
    );
}

#[test]
fn less_than() {
    assert_eq!(
        parse("a < b"),
        Expr::Less(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn less_than_or_eq() {
    assert_eq!(
        parse("a <= b"),
        Expr::LessEq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn greater_than() {
    assert_eq!(
        parse("a > b"),
        Expr::Greater(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn greater_than_or_eq() {
    assert_eq!(
        parse("a >= b"),
        Expr::GreaterEq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn equality() {
    assert_eq!(
        parse("a == b"),
        Expr::Eq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn not_equal() {
    assert_eq!(
        parse("a != b"),
        Expr::NotEq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn logical_or() {
    assert_eq!(
        parse("a || b"),
        Expr::Or(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn logical_and() {
    assert_eq!(
        parse("a && b"),
        Expr::And(b(Expr::Id("a".into())), b(Expr::Id("b".into())))
    );
}

#[test]
fn pipe() {
    assert_eq!(
        parse("a |> f"),
        Expr::Pipe(b(Expr::Id("a".into())), b(Expr::Id("f".into())))
    );
}

// ─── precedence ─────────────────────────────────────────────────────

#[test]
fn mult_before_add() {
    // 1 + 2 * 3 = 1 + (2 * 3)
    assert_eq!(
        parse("1 + 2 * 3"),
        Expr::Plus(b(Expr::Int(1)), b(Expr::Mult(b(Expr::Int(2)), b(Expr::Int(3)))))
    );
}

#[test]
fn mult_before_sub() {
    // 1 - 2 * 3 = 1 - (2 * 3)
    assert_eq!(
        parse("1 - 2 * 3"),
        Expr::Minus(b(Expr::Int(1)), b(Expr::Mult(b(Expr::Int(2)), b(Expr::Int(3)))))
    );
}

#[test]
fn add_before_relational() {
    // a + b < c = (a + b) < c
    assert_eq!(
        parse("a + b < c"),
        Expr::Less(
            b(Expr::Plus(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("c".into()))
        )
    );
}

#[test]
fn relational_before_equality() {
    // a < b == c = (a < b) == c
    assert_eq!(
        parse("a < b == c"),
        Expr::Eq(
            b(Expr::Less(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("c".into()))
        )
    );
}

#[test]
fn equality_before_and() {
    // a == b && c = (a == b) && c
    assert_eq!(
        parse("a == b && c"),
        Expr::And(
            b(Expr::Eq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("c".into()))
        )
    );
}

#[test]
fn and_before_or() {
    // a && b || c = (a && b) || c
    assert_eq!(
        parse("a && b || c"),
        Expr::Or(
            b(Expr::And(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("c".into()))
        )
    );
}

#[test]
fn or_before_pipe() {
    // a || b |> f = (a || b) |> f
    assert_eq!(
        parse("a || b |> f"),
        Expr::Pipe(
            b(Expr::Or(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("f".into()))
        )
    );
}

#[test]
fn unary_binds_tighter_than_mult() {
    // -1 * 2 = (-1) * 2
    assert_eq!(
        parse("-1 * 2"),
        Expr::Mult(b(Expr::Neg(b(Expr::Int(1)))), b(Expr::Int(2)))
    );
}

#[test]
fn unary_binds_tighter_than_add() {
    // -1 + 2 = (-1) + 2
    assert_eq!(
        parse("-1 + 2"),
        Expr::Plus(b(Expr::Neg(b(Expr::Int(1)))), b(Expr::Int(2)))
    );
}

#[test]
fn bang_binds_tighter_than_and() {
    // !a && b = (!a) && b
    assert_eq!(
        parse("!a && b"),
        Expr::And(
            b(Expr::Bang(b(Expr::Id("a".into())))),
            b(Expr::Id("b".into()))
        )
    );
}

// ─── associativity (left-to-right) ──────────────────────────────────

#[test]
fn addition_left_assoc() {
    // 1 + 2 + 3 = (1 + 2) + 3
    assert_eq!(
        parse("1 + 2 + 3"),
        Expr::Plus(
            b(Expr::Plus(b(Expr::Int(1)), b(Expr::Int(2)))),
            b(Expr::Int(3))
        )
    );
}

#[test]
fn subtraction_left_assoc() {
    // 5 - 3 - 1 = (5 - 3) - 1
    assert_eq!(
        parse("5 - 3 - 1"),
        Expr::Minus(
            b(Expr::Minus(b(Expr::Int(5)), b(Expr::Int(3)))),
            b(Expr::Int(1))
        )
    );
}

#[test]
fn multiplication_left_assoc() {
    // 2 * 3 * 4 = (2 * 3) * 4
    assert_eq!(
        parse("2 * 3 * 4"),
        Expr::Mult(
            b(Expr::Mult(b(Expr::Int(2)), b(Expr::Int(3)))),
            b(Expr::Int(4))
        )
    );
}

#[test]
fn pipe_left_assoc() {
    // x |> f |> g = (x |> f) |> g
    assert_eq!(
        parse("x |> f |> g"),
        Expr::Pipe(
            b(Expr::Pipe(b(Expr::Id("x".into())), b(Expr::Id("f".into())))),
            b(Expr::Id("g".into()))
        )
    );
}

#[test]
fn mixed_add_sub_left_assoc() {
    // 1 + 2 - 3 = (1 + 2) - 3
    assert_eq!(
        parse("1 + 2 - 3"),
        Expr::Minus(
            b(Expr::Plus(b(Expr::Int(1)), b(Expr::Int(2)))),
            b(Expr::Int(3))
        )
    );
}

// ─── parenthesized expressions ──────────────────────────────────────

#[test]
fn parens_identity() {
    assert_eq!(parse("(42)"), Expr::Int(42));
}

#[test]
fn parens_override_precedence() {
    // (1 + 2) * 3
    assert_eq!(
        parse("(1 + 2) * 3"),
        Expr::Mult(
            b(Expr::Plus(b(Expr::Int(1)), b(Expr::Int(2)))),
            b(Expr::Int(3))
        )
    );
}

#[test]
fn nested_parens() {
    assert_eq!(parse("((1))"), Expr::Int(1));
}

#[test]
fn parens_in_right_operand() {
    // 2 * (3 + 4)
    assert_eq!(
        parse("2 * (3 + 4)"),
        Expr::Mult(
            b(Expr::Int(2)),
            b(Expr::Plus(b(Expr::Int(3)), b(Expr::Int(4))))
        )
    );
}

// ─── function calls ─────────────────────────────────────────────────

#[test]
fn call_no_args() {
    assert_eq!(
        parse("f()"),
        Expr::Call(b(Expr::Id("f".into())), vec![])
    );
}

#[test]
fn call_one_arg() {
    assert_eq!(
        parse("f(1)"),
        Expr::Call(b(Expr::Id("f".into())), vec![Expr::Int(1)])
    );
}

#[test]
fn call_multiple_args() {
    assert_eq!(
        parse("f(1, 2, 3)"),
        Expr::Call(
            b(Expr::Id("f".into())),
            vec![Expr::Int(1), Expr::Int(2), Expr::Int(3)]
        )
    );
}

#[test]
fn call_with_expr_arg() {
    assert_eq!(
        parse("f(1 + 2)"),
        Expr::Call(
            b(Expr::Id("f".into())),
            vec![Expr::Plus(b(Expr::Int(1)), b(Expr::Int(2)))]
        )
    );
}

#[test]
fn chained_calls() {
    // f(x)(y) = Call(Call(f, [x]), [y])
    assert_eq!(
        parse("f(x)(y)"),
        Expr::Call(
            b(Expr::Call(
                b(Expr::Id("f".into())),
                vec![Expr::Id("x".into())]
            )),
            vec![Expr::Id("y".into())]
        )
    );
}

#[test]
fn call_in_binary_expr() {
    // f(x) + 1
    assert_eq!(
        parse("f(x) + 1"),
        Expr::Plus(
            b(Expr::Call(
                b(Expr::Id("f".into())),
                vec![Expr::Id("x".into())]
            )),
            b(Expr::Int(1))
        )
    );
}

#[test]
fn binary_expr_then_call() {
    // 1 + f(x)
    assert_eq!(
        parse("1 + f(x)"),
        Expr::Plus(
            b(Expr::Int(1)),
            b(Expr::Call(
                b(Expr::Id("f".into())),
                vec![Expr::Id("x".into())]
            ))
        )
    );
}

#[test]
fn negate_call() {
    // -f(x) = Neg(Call(f, [x]))
    assert_eq!(
        parse("-f(x)"),
        Expr::Neg(b(Expr::Call(
            b(Expr::Id("f".into())),
            vec![Expr::Id("x".into())]
        )))
    );
}

// ─── if-then-else ───────────────────────────────────────────────────

#[test]
fn simple_if() {
    assert_eq!(
        parse("if true then 1 else 2"),
        Expr::If(b(Expr::Bool(true)), b(Expr::Int(1)), b(Expr::Int(2)))
    );
}

#[test]
fn if_with_condition_expr() {
    assert_eq!(
        parse("if a == b then a else b"),
        Expr::If(
            b(Expr::Eq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("a".into())),
            b(Expr::Id("b".into()))
        )
    );
}

#[test]
fn if_with_complex_branches() {
    assert_eq!(
        parse("if x then 1 + 2 else 3 * 4"),
        Expr::If(
            b(Expr::Id("x".into())),
            b(Expr::Plus(b(Expr::Int(1)), b(Expr::Int(2)))),
            b(Expr::Mult(b(Expr::Int(3)), b(Expr::Int(4))))
        )
    );
}

#[test]
fn nested_if_in_else() {
    assert_eq!(
        parse("if a then 1 else if b then 2 else 3"),
        Expr::If(
            b(Expr::Id("a".into())),
            b(Expr::Int(1)),
            b(Expr::If(
                b(Expr::Id("b".into())),
                b(Expr::Int(2)),
                b(Expr::Int(3))
            ))
        )
    );
}

// ─── match ──────────────────────────────────────────────────────────

#[test]
fn match_single_arm() {
    assert_eq!(
        parse("match x { a() -> 1 }"),
        Expr::Match(
            b(Expr::Id("x".into())),
            vec![("a".into(), vec![], Expr::Int(1))]
        )
    );
}

#[test]
fn match_multiple_arms() {
    assert_eq!(
        parse("match x { a() -> 1, b() -> 2 }"),
        Expr::Match(
            b(Expr::Id("x".into())),
            vec![
                ("a".into(), vec![], Expr::Int(1)),
                ("b".into(), vec![], Expr::Int(2)),
            ]
        )
    );
}

#[test]
fn match_with_bindings() {
    assert_eq!(
        parse("match x { some(val: Int) -> val, none() -> 0 }"),
        Expr::Match(
            b(Expr::Id("x".into())),
            vec![
                (
                    "some".into(),
                    vec![Binding::new("val".into(), Type::Int)],
                    Expr::Id("val".into())
                ),
                ("none".into(), vec![], Expr::Int(0)),
            ]
        )
    );
}

#[test]
fn match_arm_with_expr_body() {
    assert_eq!(
        parse("match x { a(n: Int) -> n + 1 }"),
        Expr::Match(
            b(Expr::Id("x".into())),
            vec![(
                "a".into(),
                vec![Binding::new("n".into(), Type::Int)],
                Expr::Plus(b(Expr::Id("n".into())), b(Expr::Int(1)))
            )]
        )
    );
}

#[test]
fn match_trailing_comma() {
    // trailing comma should be accepted
    assert_eq!(
        parse("match x { a() -> 1, b() -> 2, }"),
        Expr::Match(
            b(Expr::Id("x".into())),
            vec![
                ("a".into(), vec![], Expr::Int(1)),
                ("b".into(), vec![], Expr::Int(2)),
            ]
        )
    );
}

#[test]
fn match_with_multiple_bindings() {
    assert_eq!(
        parse("match p { point(x: Int, y: Int) -> x + y }"),
        Expr::Match(
            b(Expr::Id("p".into())),
            vec![(
                "point".into(),
                vec![
                    Binding::new("x".into(), Type::Int),
                    Binding::new("y".into(), Type::Int),
                ],
                Expr::Plus(b(Expr::Id("x".into())), b(Expr::Id("y".into())))
            )]
        )
    );
}

// ─── closures ───────────────────────────────────────────────────────

#[test]
fn simple_closure() {
    assert_eq!(
        parse("fn (x: Int) -> x"),
        Expr::Fn(Binding::new("x".into(), Type::Int), b(Expr::Id("x".into())))
    );
}

#[test]
fn closure_with_body_expr() {
    assert_eq!(
        parse("fn (x: Int) -> x + 1"),
        Expr::Fn(
            Binding::new("x".into(), Type::Int),
            b(Expr::Plus(b(Expr::Id("x".into())), b(Expr::Int(1))))
        )
    );
}

#[test]
fn nested_closures() {
    assert_eq!(
        parse("fn (x: Int) -> fn (y: Int) -> x + y"),
        Expr::Fn(
            Binding::new("x".into(), Type::Int),
            b(Expr::Fn(
                Binding::new("y".into(), Type::Int),
                b(Expr::Plus(b(Expr::Id("x".into())), b(Expr::Id("y".into()))))
            ))
        )
    );
}

#[test]
fn closure_with_bool_param() {
    assert_eq!(
        parse("fn (b: Bool) -> !b"),
        Expr::Fn(
            Binding::new("b".into(), Type::Bool),
            b(Expr::Bang(b(Expr::Id("b".into()))))
        )
    );
}

// ─── complex expressions ────────────────────────────────────────────

#[test]
fn mixed_arithmetic() {
    // 1 + 2 * 3 - 4 = ((1 + (2 * 3)) - 4)
    assert_eq!(
        parse("1 + 2 * 3 - 4"),
        Expr::Minus(
            b(Expr::Plus(
                b(Expr::Int(1)),
                b(Expr::Mult(b(Expr::Int(2)), b(Expr::Int(3))))
            )),
            b(Expr::Int(4))
        )
    );
}

#[test]
fn comparison_chain() {
    // a == b != c = (a == b) != c
    assert_eq!(
        parse("a == b != c"),
        Expr::NotEq(
            b(Expr::Eq(b(Expr::Id("a".into())), b(Expr::Id("b".into())))),
            b(Expr::Id("c".into()))
        )
    );
}

#[test]
fn full_precedence_chain() {
    // a |> b || c && d == e < f + g * h
    // = a |> (b || (c && (d == (e < (f + (g * h))))))
    assert_eq!(
        parse("a |> b || c && d == e < f + g * h"),
        Expr::Pipe(
            b(Expr::Id("a".into())),
            b(Expr::Or(
                b(Expr::Id("b".into())),
                b(Expr::And(
                    b(Expr::Id("c".into())),
                    b(Expr::Eq(
                        b(Expr::Id("d".into())),
                        b(Expr::Less(
                            b(Expr::Id("e".into())),
                            b(Expr::Plus(
                                b(Expr::Id("f".into())),
                                b(Expr::Mult(b(Expr::Id("g".into())), b(Expr::Id("h".into()))))
                            ))
                        ))
                    ))
                ))
            ))
        )
    );
}

#[test]
fn call_in_pipe() {
    // x |> f(y) — note: pipe binds looser than call, so f(y) is parsed first
    // Actually: the pipe operator gets x and then parses f with pipe precedence.
    // f(y) has higher precedence (Call > Pipe), so it becomes Call(f, [y]).
    // Result: Pipe(x, Call(f, [y]))
    assert_eq!(
        parse("x |> f(y)"),
        Expr::Pipe(
            b(Expr::Id("x".into())),
            b(Expr::Call(
                b(Expr::Id("f".into())),
                vec![Expr::Id("y".into())]
            ))
        )
    );
}

// ─── error cases ────────────────────────────────────────────────────

#[test]
fn error_on_semicolon() {
    let mut p = parser_from(";");
    assert!(super::parse_expr(&mut p).is_err());
}

#[test]
fn error_on_rbrace() {
    let mut p = parser_from("}");
    assert!(super::parse_expr(&mut p).is_err());
}

#[test]
fn error_on_arrow() {
    let mut p = parser_from("->");
    assert!(super::parse_expr(&mut p).is_err());
}

#[test]
fn error_on_eof() {
    let mut p = parser_from("");
    assert!(super::parse_expr(&mut p).is_err());
}
