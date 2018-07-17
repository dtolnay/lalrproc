#![feature(proc_macro_non_items, use_extern_macros)]

extern crate lalrproc;
use lalrproc::{s_expr, s_type};

#[test]
fn test_s_type() {
    assert_eq!(
        "(Reference 'a mut (Path module T))",
        s_type!(&'a mut module::T)
    );
}

#[test]
fn test_s_expr() {
    assert_eq!("(+ (* 1 2) (* 3 4))", s_expr!(1 * 2 + 3 * 4));
}
