use crate::ast::*;
use std::fmt::{self, Display};

impl Display for Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Path(ref path) => path.fmt(formatter),
            Type::Reference(ref reference) => reference.fmt(formatter),
        }
    }
}

impl Display for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("(Path")?;
        for segment in &self.segments {
            write!(formatter, " {segment}")?;
        }
        formatter.write_str(")")
    }
}

impl Display for Reference {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("(Reference")?;
        if let Some(ref lifetime) = self.lifetime {
            write!(formatter, " '{}", lifetime.ident)?;
        }
        if self.mutable {
            formatter.write_str(" mut")?;
        }
        write!(formatter, " {})", self.elem)
    }
}

impl Display for Expr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Path(ref path) => path.fmt(formatter),
            Expr::Lit(ref lit) => lit.fmt(formatter),
            Expr::Binary(ref left, ref op, ref right) => {
                write!(formatter, "({op} {left} {right})")
            }
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinOp::Add => formatter.write_str("+"),
            BinOp::Sub => formatter.write_str("-"),
            BinOp::Mul => formatter.write_str("*"),
            BinOp::Div => formatter.write_str("/"),
        }
    }
}
