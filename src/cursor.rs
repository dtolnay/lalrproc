use proc_macro::token_stream::IntoIter as TokenIter;
use proc_macro::{self, Delimiter, Ident, Spacing, TokenStream, TokenTree};
use span::Span;
use token::{Keyword, Token};

pub struct Cursor {
    stack: Vec<Frame>,
    joint: Option<Span>,
}

struct Frame {
    iter: TokenIter,
    span: Span,
    delimiter: Delimiter,
}

impl Cursor {
    pub fn new(input: TokenStream) -> Self {
        Cursor {
            stack: vec![Frame {
                iter: input.into_iter(),
                span: Span(proc_macro::Span::call_site()),
                delimiter: Delimiter::None,
            }],
            joint: None,
        }
    }
}

impl Iterator for Cursor {
    type Item = (Span, Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(span) = self.joint.take() {
            return Some((span, Token::Joint, span));
        }

        let mut top = self.stack.pop()?;
        match top.iter.next() {
            Some(tt) => {
                let span = Span(tt.span());
                self.stack.push(top);
                let token = match tt {
                    TokenTree::Group(tt) => {
                        let delimiter = tt.delimiter();
                        let iter = tt.stream().into_iter();
                        self.stack.push(Frame {
                            iter,
                            span,
                            delimiter,
                        });
                        Token::Open(delimiter)
                    }
                    TokenTree::Punct(tt) => {
                        if let Spacing::Joint = tt.spacing() {
                            self.joint = Some(span);
                        }
                        Token::Punct(tt.as_char())
                    }
                    TokenTree::Ident(ident) => ident_to_token(ident),
                    TokenTree::Literal(lit) => Token::Literal(lit),
                };
                Some((span, token, span))
            }
            None => if self.stack.is_empty() {
                None
            } else {
                Some((top.span, Token::Close(top.delimiter), top.span))
            },
        }
    }
}

fn ident_to_token(ident: Ident) -> Token {
    match &*ident.to_string() {
        "abstract" => Token::Keyword(Keyword::Abstract),
        "alignof" => Token::Keyword(Keyword::Alignof),
        "as" => Token::Keyword(Keyword::As),
        "become" => Token::Keyword(Keyword::Become),
        "box" => Token::Keyword(Keyword::Box),
        "break" => Token::Keyword(Keyword::Break),
        "const" => Token::Keyword(Keyword::Const),
        "continue" => Token::Keyword(Keyword::Continue),
        "crate" => Token::Keyword(Keyword::Crate),
        "do" => Token::Keyword(Keyword::Do),
        "else" => Token::Keyword(Keyword::Else),
        "enum" => Token::Keyword(Keyword::Enum),
        "extern" => Token::Keyword(Keyword::Extern),
        "false" => Token::Keyword(Keyword::False),
        "final" => Token::Keyword(Keyword::Final),
        "fn" => Token::Keyword(Keyword::Fn),
        "for" => Token::Keyword(Keyword::For),
        "if" => Token::Keyword(Keyword::If),
        "impl" => Token::Keyword(Keyword::Impl),
        "in" => Token::Keyword(Keyword::In),
        "let" => Token::Keyword(Keyword::Let),
        "loop" => Token::Keyword(Keyword::Loop),
        "macro" => Token::Keyword(Keyword::Macro),
        "match" => Token::Keyword(Keyword::Match),
        "mod" => Token::Keyword(Keyword::Mod),
        "move" => Token::Keyword(Keyword::Move),
        "mut" => Token::Keyword(Keyword::Mut),
        "offsetof" => Token::Keyword(Keyword::Offsetof),
        "override" => Token::Keyword(Keyword::Override),
        "priv" => Token::Keyword(Keyword::Priv),
        "proc" => Token::Keyword(Keyword::Proc),
        "pub" => Token::Keyword(Keyword::Pub),
        "pure" => Token::Keyword(Keyword::Pure),
        "ref" => Token::Keyword(Keyword::Ref),
        "return" => Token::Keyword(Keyword::Return),
        "Self" => Token::Keyword(Keyword::UpperSelf),
        "self" => Token::Keyword(Keyword::LowerSelf),
        "sizeof" => Token::Keyword(Keyword::Sizeof),
        "static" => Token::Keyword(Keyword::Static),
        "struct" => Token::Keyword(Keyword::Struct),
        "super" => Token::Keyword(Keyword::Super),
        "trait" => Token::Keyword(Keyword::Trait),
        "true" => Token::Keyword(Keyword::True),
        "type" => Token::Keyword(Keyword::Type),
        "typeof" => Token::Keyword(Keyword::Typeof),
        "unsafe" => Token::Keyword(Keyword::Unsafe),
        "unsized" => Token::Keyword(Keyword::Unsized),
        "use" => Token::Keyword(Keyword::Use),
        "virtual" => Token::Keyword(Keyword::Virtual),
        "where" => Token::Keyword(Keyword::Where),
        "while" => Token::Keyword(Keyword::While),
        "yield" => Token::Keyword(Keyword::Yield),
        _other => Token::Ident(ident),
    }
}
