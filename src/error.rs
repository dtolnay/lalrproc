use crate::span::Span;
use crate::token::Token;
use lalrpop_util::ParseError;
use std::fmt::{self, Display};

pub enum NoUserError {}

impl Display for NoUserError {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

pub fn emit(err: ParseError<Span, Token, NoUserError>) {
    match err {
        ParseError::InvalidToken { location: span } => {
            span.0
                .error("failed to parse macro input")
                .note("invalid token")
                .emit();
        }
        ParseError::UnrecognizedToken {
            token: (span, token, _),
            expected,
        } => {
            let mut diagnostic = span
                .0
                .error("failed to parse macro input")
                .note(format!("unrecognized token {}", token));
            if !expected.is_empty() {
                diagnostic = diagnostic.note(Expected(&expected).to_string());
            }
            diagnostic.emit();
        }
        ParseError::UnrecognizedEOF {
            location: span,
            expected,
        } => {
            let mut diagnostic = span
                .0
                .error("failed to parse macro input")
                .note("unexpected EOF");
            if !expected.is_empty() {
                diagnostic = diagnostic.note(Expected(&expected).to_string());
            }
            diagnostic.emit();
        }
        ParseError::ExtraToken {
            token: (span, token, _),
        } => {
            span.0
                .error("failed to parse macro input")
                .note(format!("extra token {}", token))
                .emit();
        }
        ParseError::User { error } => match error {},
    }
}

struct Expected<'a>(&'a [String]);

impl<'a> Display for Expected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        assert!(!self.0.is_empty());

        if self.0.len() == 1 {
            write!(formatter, "expected {}", GrammarRule(&self.0[0]))
        } else {
            formatter.write_str("expected ")?;
            if self.0.len() > 2 {
                formatter.write_str("one of ")?;
            }
            for (i, e) in self.0.iter().enumerate() {
                if i == self.0.len() - 1 {
                    formatter.write_str(", or ")?;
                } else if i > 0 {
                    formatter.write_str(", ")?;
                }
                GrammarRule(e).fmt(formatter)?;
            }
            Ok(())
        }
    }
}

struct GrammarRule<'a>(&'a str);

impl<'a> Display for GrammarRule<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            "BasicIdent" => formatter.write_str("identifier"),
            "BasicLifetime" => formatter.write_str("lifetime"),
            "Literal" => formatter.write_str("literal"),
            s => {
                if s.len() > 2
                    && s.starts_with('"')
                    && s.ends_with('"')
                    && !s[1..s.len() - 1].contains('"')
                {
                    write!(formatter, "`{}`", &s[1..s.len() - 1])
                } else {
                    s.fmt(formatter)
                }
            }
        }
    }
}
