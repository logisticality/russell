== Grammar
A Russell program is a list of definitions, and obeys the following grammar.
```
<defn> ::= typedef <id> { <id> ( <binding> , ... ); };
         | fn <id>( <binding> , ... ) -> <type> { <stmnt>; }

<stmnt> ::= let <id> = <exp>;
          | read <type> <id>;
          | echo <type> <exp>;
          | return <exp>;

<exp> ::= <integer>
        | <float>
        | <bool>
        | <id>
        | fn ( <binding> ) -> <exp>
        | - <exp>
        | ! <exp>
        | <id>(<exp>)
        | <exp> + <exp>
        | <exp> - <exp>
        | <exp> * <exp>
        | <exp> / <exp>
        | if <exp> then <exp> else <exp>
        | <exp> < <exp>
        | <exp> <= <exp>
        | <exp> > <exp>
        | <exp> >= <exp>
        | <exp> == <exp>
        | <exp> != <exp>
        | <exp> && <exp>
        | <exp> || <exp>
        | <exp> |> <exp>
        | ( <exp> )
        | match <exp> { <id>(<id, ...) -> <exp> }

<type> ::= u64
         | i64
         | f64
         | bool

binding ::= <id> : <type
```
