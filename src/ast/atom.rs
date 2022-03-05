use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Ident(String),
    Number(f64),
}

#[allow(clippy::from_over_into)]
impl Into<Expr> for Atom {
    fn into(self) -> Expr {
        Expr::Atom(self)
    }
}
