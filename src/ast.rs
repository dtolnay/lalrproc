use crate::span::Span;
use proc_macro::Literal;

pub use proc_macro::Ident;

#[derive(Debug)]
pub enum Type {
    Path(Path),
    Reference(Reference),
}

#[derive(Debug)]
pub struct Path {
    pub segments: Vec<Ident>,
}

#[derive(Debug)]
pub struct Reference {
    pub lifetime: Option<Lifetime>,
    pub mutable: bool,
    pub elem: Box<Type>,
}

#[derive(Debug)]
pub enum Expr {
    Path(Path),
    Lit(Literal),
    Binary(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Lifetime {
    pub ident: Ident,
    pub span: Span,
}
