use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Pos,
    Neg,
    Fac,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryArithmetic {
    pub op: UnaryOp,
    pub value: Expr,
}

impl UnaryArithmetic {
    pub fn new(op: UnaryOp, value: Expr) -> Self {
        UnaryArithmetic { op, value }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Expr> for UnaryArithmetic {
    fn into(self) -> Expr {
        Expr::UnaryArithmetic(Box::new(self))
    }
}
