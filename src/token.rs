use proc_macro::{Term, Literal, Delimiter};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Token {
    Open(Delimiter),
    Close(Delimiter),
    Op(char),
    Joint,
    Keyword(Keyword),
    Ident(Term),
    Lifetime(Term),
    Literal(Literal),
}

// From https://doc.rust-lang.org/grammar.html#keywords
#[derive(Debug)]
pub enum Keyword {
    Abstract, Alignof, As, Become, Box, Break, Const, Continue, Crate, Do, Else,
    Enum, Extern, False, Final, Fn, For, If, Impl, In, Let, Loop, Macro, Match,
    Mod, Move, Mut, Offsetof, Override, Priv, Proc, Pub, Pure, Ref, Return,
    UpperSelf, LowerSelf, Sizeof, Static, Struct, Super, Trait, True, Type,
    Typeof, Unsafe, Unsized, Use, Virtual, Where, While, Yield,
}

impl Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Open(Delimiter::Parenthesis) => formatter.write_str("`(`"),
            Token::Open(Delimiter::Brace) => formatter.write_str("`{`"),
            Token::Open(Delimiter::Bracket) => formatter.write_str("`[`"),
            Token::Open(Delimiter::None) => formatter.write_str("None-delimiter"),
            Token::Close(Delimiter::Parenthesis) => formatter.write_str("`)`"),
            Token::Close(Delimiter::Brace) => formatter.write_str("`}`"),
            Token::Close(Delimiter::Bracket) => formatter.write_str("`]`"),
            Token::Close(Delimiter::None) => formatter.write_str("None-delimiter"),
            Token::Op(op) => write!(formatter, "`{}`", op),
            Token::Joint => formatter.write_str("joint-op"),
            Token::Keyword(ref keyword) => keyword.fmt(formatter),
            Token::Ident(term) | Token::Lifetime(term) => write!(formatter, "`{}`", term.as_str()),
            Token::Literal(ref lit) => write!(formatter, "`{}`", lit),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Keyword::Abstract => formatter.write_str("`abstract`"),
            Keyword::Alignof => formatter.write_str("`alignof`"),
            Keyword::As => formatter.write_str("`as`"),
            Keyword::Become => formatter.write_str("`become`"),
            Keyword::Box => formatter.write_str("`box`"),
            Keyword::Break => formatter.write_str("`break`"),
            Keyword::Const => formatter.write_str("`const`"),
            Keyword::Continue => formatter.write_str("`continue`"),
            Keyword::Crate => formatter.write_str("`crate`"),
            Keyword::Do => formatter.write_str("`do`"),
            Keyword::Else => formatter.write_str("`else`"),
            Keyword::Enum => formatter.write_str("`enum`"),
            Keyword::Extern => formatter.write_str("`extern`"),
            Keyword::False => formatter.write_str("`false`"),
            Keyword::Final => formatter.write_str("`final`"),
            Keyword::Fn => formatter.write_str("`fn`"),
            Keyword::For => formatter.write_str("`for`"),
            Keyword::If => formatter.write_str("`if`"),
            Keyword::Impl => formatter.write_str("`impl`"),
            Keyword::In => formatter.write_str("`in`"),
            Keyword::Let => formatter.write_str("`let`"),
            Keyword::Loop => formatter.write_str("`loop`"),
            Keyword::Macro => formatter.write_str("`macro`"),
            Keyword::Match => formatter.write_str("`match`"),
            Keyword::Mod => formatter.write_str("`mod`"),
            Keyword::Move => formatter.write_str("`move`"),
            Keyword::Mut => formatter.write_str("`mut`"),
            Keyword::Offsetof => formatter.write_str("`offsetof`"),
            Keyword::Override => formatter.write_str("`override`"),
            Keyword::Priv => formatter.write_str("`priv`"),
            Keyword::Proc => formatter.write_str("`proc`"),
            Keyword::Pub => formatter.write_str("`pub`"),
            Keyword::Pure => formatter.write_str("`pure`"),
            Keyword::Ref => formatter.write_str("`ref`"),
            Keyword::Return => formatter.write_str("`return`"),
            Keyword::UpperSelf => formatter.write_str("`Self`"),
            Keyword::LowerSelf => formatter.write_str("`self`"),
            Keyword::Sizeof => formatter.write_str("`sizeof`"),
            Keyword::Static => formatter.write_str("`static`"),
            Keyword::Struct => formatter.write_str("`struct`"),
            Keyword::Super => formatter.write_str("`super`"),
            Keyword::Trait => formatter.write_str("`trait`"),
            Keyword::True => formatter.write_str("`true`"),
            Keyword::Type => formatter.write_str("`type`"),
            Keyword::Typeof => formatter.write_str("`typeof`"),
            Keyword::Unsafe => formatter.write_str("`unsafe`"),
            Keyword::Unsized => formatter.write_str("`unsized`"),
            Keyword::Use => formatter.write_str("`use`"),
            Keyword::Virtual => formatter.write_str("`virtual`"),
            Keyword::Where => formatter.write_str("`where`"),
            Keyword::While => formatter.write_str("`while`"),
            Keyword::Yield => formatter.write_str("`yield`"),
        }
    }
}
