use super::{Atom, BinaryArithmetic, FunctionCall, UnaryArithmetic};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BinaryArithmetic(Box<BinaryArithmetic>),
    UnaryArithmetic(Box<UnaryArithmetic>),
    Atom(Atom),
    FunctionCall(FunctionCall),
}
