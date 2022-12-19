//! [![github]](https://github.com/dtolnay/lalrproc)&ensp;[![crates-io]](https://crates.io/crates/lalrproc)&ensp;[![docs-rs]](https://docs.rs/lalrproc)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

#![feature(proc_macro_diagnostic)]
#![allow(
    clippy::empty_enum,
    clippy::from_iter_instead_of_collect,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::use_self,
    clippy::wildcard_imports
)]

extern crate proc_macro;

mod ast;
mod cursor;
mod error;
mod sexpr;
mod span;
mod token;

mod parse {
    #![allow(clippy::all, clippy::pedantic)]

    include!(concat!(env!("OUT_DIR"), "/parse.rs"));
}

use crate::cursor::Cursor;
use crate::error::NoUserError;
use crate::parse::{ExprParser, TypeParser};
use crate::span::Span;
use crate::token::Token;
use lalrpop_util::ParseError;
use proc_macro::{Delimiter, Group, Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn s_type(input: TokenStream) -> TokenStream {
    match TypeParser::new().parse(Cursor::new(input)) {
        Ok(t) => string_literal(&t.to_string()),
        Err(err) => parse_error(err),
    }
}

#[proc_macro]
pub fn s_expr(input: TokenStream) -> TokenStream {
    match ExprParser::new().parse(Cursor::new(input)) {
        Ok(e) => string_literal(&e.to_string()),
        Err(err) => parse_error(err),
    }
}

fn string_literal(s: &str) -> TokenStream {
    let lit = Literal::string(s);
    TokenStream::from(TokenTree::Literal(lit))
}

fn parse_error(err: ParseError<Span, Token, NoUserError>) -> TokenStream {
    error::emit(err);
    let group = Group::new(Delimiter::Brace, TokenStream::new());
    TokenStream::from(TokenTree::Group(group))
}
