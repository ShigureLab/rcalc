use crate::ast::{Atom, BinaryArithmetic, Expr, FunctionCall, UnaryArithmetic};

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_unary(&mut self, u: &UnaryArithmetic) -> T;
    fn visit_binary(&mut self, b: &BinaryArithmetic) -> T;
    fn visit_function(&mut self, f: &FunctionCall) -> T;
    fn visit_atom(&mut self, a: &Atom) -> T;
}
