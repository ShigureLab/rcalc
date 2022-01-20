use super::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}

impl FunctionCall {
    pub fn new(name: String, args: Vec<Expr>) -> Self {
        FunctionCall { name, args }
    }
}

impl Into<Expr> for FunctionCall {
    fn into(self) -> Expr {
        Expr::FunctionCall(self)
    }
}
