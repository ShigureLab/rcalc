use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Ident(String),
    Number(f64),
}

impl Into<Expr> for Atom {
    fn into(self) -> Expr {
        Expr::Atom(self)
    }
}
