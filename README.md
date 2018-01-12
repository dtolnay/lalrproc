LALRPOP parser for procedural macro input
=========================================

[![Build Status](https://api.travis-ci.org/dtolnay/lalrproc.svg?branch=master)](https://travis-ci.org/dtolnay/lalrproc)

This crate demostrates a proof of concept of using the [LALRPOP] parser
generator framework to parse input tokens in a Rust procedural macro.

[LALRPOP]: https://github.com/nikomatsakis/lalrpop

## Macros

The procedural macros in this example are `s_expr!` and `s_type!` which expand
to a string literal S-expression representation of the Rust expression or type
given in the input.

```rust
extern crate lalrproc;
use lalrproc::{s_expr, s_type};

fn main() {
    // Expands to "(+ (* 1 2) (* 3 4))"
    let e = s_expr!(1 * 2 + 3 * 4);

    // Expands to "(Reference 'a mut (Path module T))"
    let t = s_type!(&'a mut module::T);
}
```

## Parser

The input parsing is handled entirely by LALRPOP. In particular, [Syn] and
[proc-macro2] are not involved.

[Syn]: https://github.com/dtolnay/syn
[proc-macro2]: https://github.com/alexcrichton/proc-macro2

All of the features of LALRPOP are available to the LALRPOP grammar. For example
here is a subset of the expression grammar illustrating the parsing of something
like `1 * 2 + 3 * 4` into the syntax tree `(+ (* 1 2) (* 3 4))`.

```rust
pub Expr: Box<Expr> = {
    Expr ExprOp Factor => Box::new(Expr::Binary(<>)),
    Factor,
};

ExprOp: BinOp = {
    "+" => BinOp::Add,
    "-" => BinOp::Sub,
};

Factor: Box<Expr> = {
    Factor FactorOp Component => Box::new(Expr::Binary(<>)),
    Component,
};

FactorOp: BinOp = {
    "*" => BinOp::Mul,
    "/" => BinOp::Div,
};

Component: Box<Expr> = {
    Literal => Box::new(Expr::Lit(<>)),
    Path => Box::new(Expr::Path(<>)),
    "(" <Expr> ")",
};
```

## Error reporting

If the input does not conform to the grammar expected by the procedural macro,
the error from LALRPOP is rendered in a way that indicates the problematic token
and gives the tokens that would have been accepted by the grammar in that
position.

```
error: failed to parse macro input
  --> tests/test.rs:17:25
   |
17 |     let e = s_expr!(1 + : / 2);
   |                         ^
   |
   = note: unrecognized token `:`
   = note: expected one of `(`, identifier, or literal
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
