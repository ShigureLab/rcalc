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
    pub left: Expr,
    pub right: Expr,
}

impl BinaryArithmetic {
    pub fn new(op: BinaryOp, left: Expr, right: Expr) -> Self {
        BinaryArithmetic { op, left, right }
    }
}

impl Into<Expr> for BinaryArithmetic {
    fn into(self) -> Expr {
        Expr::BinaryArithmetic(Box::new(self))
    }
}
