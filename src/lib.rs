#![feature(proc_macro)]

extern crate proc_macro;
extern crate lalrpop_util;

mod ast;
mod cursor;
mod error;
mod parse;
mod sexpr;
mod span;
mod token;

use proc_macro::{TokenStream, Literal, TokenTree, TokenNode, Delimiter};
use cursor::Cursor;
use std::iter::{self, FromIterator};
use lalrpop_util::ParseError;
use span::Span;
use token::Token;
use error::NoUserError;

#[proc_macro]
pub fn s_type(input: TokenStream) -> TokenStream {
    match parse::parse_Type(Cursor::new(input)) {
        Ok(t) => string_literal(&t.to_string()),
        Err(err) => parse_error(err),
    }
}

#[proc_macro]
pub fn s_expr(input: TokenStream) -> TokenStream {
    match parse::parse_Expr(Cursor::new(input)) {
        Ok(e) => string_literal(&e.to_string()),
        Err(err) => parse_error(err),
    }
}

fn string_literal(s: &str) -> TokenStream {
    TokenStream::from_iter(iter::once(TokenTree {
        span: proc_macro::Span::call_site(),
        kind: TokenNode::Literal(Literal::string(s)),
    }))
}

fn parse_error(err: ParseError<Span, Token, NoUserError>) -> TokenStream {
    error::emit(err);
    TokenStream::from_iter(iter::once(TokenTree {
        span: proc_macro::Span::call_site(),
        kind: TokenNode::Group(Delimiter::Brace, TokenStream::empty()),
    }))
}
