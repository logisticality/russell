pub enum Defn {
    // typedef <typeId> { <id> ( <binding> , ... ) , ... };
    Typedef(String, Vec<(String, Vec<Binding>)>),

    // fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }
    Fn(String, Vec<Binding>, Type, Vec<Stmnt>),
}

pub enum Stmnt {
    Let(String, Expr),  // let <id> = <expr>;
    Read(Type, String), // read <type> <id>;
    Echo(Type, Expr),   // echo <type> <expr>;
    Return(Expr),       // return <expr>;
}

pub enum Expr {
    // atomic expressions
    Int(i64),
    Float(f64),
    Bool(bool),

    // idents
    Id(String),

    // closures
    Fn(Binding, Box<Expr>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<Expr>),  // - <exp>
    Bang(Box<Expr>), // ! <exp>

    // function calls
    Call(Box<Expr>, Box<Expr>), // <left>(<right>)

    // binary operators
    Plus(Box<Expr>, Box<Expr>),    // <left> + <right>
    Minus(Box<Expr>, Box<Expr>),   // <left> - <right>
    Mult(Box<Expr>, Box<Expr>),    // <left> * <right>
    Div(Box<Expr>, Box<Expr>),     // <left> / <right>
    Pipe(Box<Expr>, Box<Expr>),    // <left> |> <right>
    Less(Box<Expr>, Box<Expr>),    // <left> < <right>
    LessEq(Box<Expr>, Box<Expr>),  // <left> <= <right>
    Greater(Box<Expr>, Box<Expr>), // <left> > <right>
    GreaterEq(Box<Expr>, Box<Expr>), // <left> >= <right>
    Eq(Box<Expr>, Box<Expr>),      // <left> == <right>
    NotEq(Box<Expr>, Box<Expr>),   // <left> != <right>
    Or(Box<Expr>, Box<Expr>),      // <left> || <right>
    And(Box<Expr>, Box<Expr>),     // <left> && <right>

    // if <1> then <2> else <3>
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    // match <expr> { <id>(<bindings>) -> <expr>, ... }
    Match(Box<Expr>, Vec<(String, Vec<Binding>, Expr)>),
}

pub enum Type {
    Int,
    Float,
    Bool,
    TypeId(String),
    Fn(Box<Type>, Box<Type>),
}

pub struct Binding {
    pub id: String,
    pub typ: Type,
}

impl Binding {
    pub fn new(id: String, typ: Type) -> Binding {
        Binding { id, typ }
    }
}
