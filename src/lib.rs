#![feature(proc_macro_diagnostic)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::empty_enum,
    clippy::indexing_slicing,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::use_self
)]

extern crate lalrpop_util;
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

use cursor::Cursor;
use error::NoUserError;
use lalrpop_util::ParseError;
use parse::{ExprParser, TypeParser};
use proc_macro::{Delimiter, Group, Literal, TokenStream, TokenTree};
use span::Span;
use std::iter::{self, FromIterator};
use token::Token;

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
    TokenStream::from_iter(iter::once(TokenTree::Literal(lit)))
}

fn parse_error(err: ParseError<Span, Token, NoUserError>) -> TokenStream {
    error::emit(err);
    let group = Group::new(Delimiter::Brace, TokenStream::new());
    TokenStream::from_iter(iter::once(TokenTree::Group(group)))
}
