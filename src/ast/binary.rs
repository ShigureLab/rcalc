use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryArithmetic {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

impl BinaryArithmetic {
    pub fn new(op: BinaryOp, lhs: Expr, rhs: Expr) -> Self {
        BinaryArithmetic { op, lhs, rhs }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Expr> for BinaryArithmetic {
    fn into(self) -> Expr {
        Expr::BinaryArithmetic(Box::new(self))
    }
}
