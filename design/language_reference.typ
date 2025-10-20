#show heading: set align(center)

= About
*Russell* is a high-level, simple functional language with math-like syntax, inspired by Gleam, OCaml, and Zig. The name "Russell" was adapted from mathematician and philosopher Bertrand Russell, as well as paying homage to the implementation of the language in Rust. Some key features of Russell are
1. First-Class Functions and First-Class Types
2. Closures
3. Sum Data Types and Pattern Matching
4. Garbage Collection
5. Parametric Polymorphism
6. Partial Application and Argument Labels

There is _no shadowing_ and _no mutability_ in Russell. Once you have defined a variable, you cannot redefine it. The goal is to have as much _referential transparency_ as possible - each value may be replaced directly with its literals without changing the meaning of the program.

A simple "Hello, World!" program in Russell can be written as below:
```
```
// todo

= Statements
Each line of Russell code is a *statement*. // todo

Comments can be made with conventional `//` syntax. There are no multi-line comments in Russell.

= Variables
Variables are declared with the following syntax.
```
let a : Z = 10;
```
...where `a` is the name of the variable, `Z` is the _type_ of the variable, and `10` is the _value_ of the variable. By convention, variable declarations (other than type definitions, as seen below) should be written in `snake_case`.

Variable names must begin with an alphabetical character, and may contain any combination of alphabetical characters, numbers, and underscores. More specifically, variables are captured with the regular expression `[a-zA-Z][a-zA-Z0-9_]*`.

= Types
There are six built-in types in Russell.

Mathematical types include $NN$, $ZZ$, and $RR$, which are backed by unsigned 64-bit integers, signed 64-bit integers, and 64-bit floats, respectively. Their associated types are `N`, `Z`, and `R`.

The `Type` type represents a type, as types are first-class citizens in Russell.

The `Any` type represents a sum-data type over all possible types. Any type of data may enter this.

Composite types are possible using an ML-like type system. _Only_ sum-data types are possible in Russell. See the following example.
```
let Number : Type = typedef {
  int(num : Z);
  real(num : R);
};
```

This will create an `int` function, which takes a `Z` and returns a `Number`, and a `real` function, which takes an `R` and returns a `Number`. By convention, type definitions should be written in `PascalCase`.

Product-types (like structs) can be produced by creating a sum-type with only one variant.
```
let Coordinates : Type = typedef {
  coordinates(x : Z, y : Z);
}
```

You can also choose not to add data to your sum-type to create enums.
```
let Animal : Type = typedef {
  dog();
  cat();
}
```

Here is an example of an Int Option type, for example.
```
let IntOption : Type = typedef {
  some(data : Z);
  none();
};
```

Here is an example of a linked-list.
```
let IntList : Type = typedef {
  empty();
  cons(data : Z, rest : IntList);
};
```

Parametric polymorphism can be leveraged with the `Type` type; simply pass in the type as a value and use it as normal. Here is an example of a generic Option.
```
let Option : Type = typedef {
  some(T : Type, data : T);
  none();
}
```

= Functions
Functions are first-class citizens in Russell. As such, Russell makes no distinction between the instantiation of a function, and the instantiation of other variables. Functions are _not_ closures; they cannot capture their environment. As an example, here is how one would declare the `add` function to add two numbers.
```
let add : Function =
  function (x : Z, y : Z) returns Z {
    return x + y;
  };
```

Note that since Russell is executed top-down in the file, functions must be declared before they are used, the same way variables must be declared before they are used.

= Standard Library
Built-in functions begin with the `_` character.

`_cast(d : Any, t : Type) returns t` will try to cast the given data to the given Type. If this is impossible, it returns an error.

`_type(d : Any) returns Type` will try to get the type of a given piece of data.
