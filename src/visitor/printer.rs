use super::Visitor;
use crate::ast::{Atom, BinaryArithmetic, BinaryOp, Expr, FunctionCall, UnaryArithmetic, UnaryOp};

pub struct PrettyPrinter {
    indent_level: u32,
    indent: u32,
}

impl PrettyPrinter {
    pub fn new(indent: u32) -> Self {
        PrettyPrinter {
            indent_level: 0,
            indent,
        }
    }

    fn get_indent(&self) -> usize {
        (self.indent_level * self.indent) as usize
    }
}

impl Visitor<()> for PrettyPrinter {
    fn visit_expr(&mut self, e: &Expr) {
        let indent = " ".repeat(self.get_indent());
        println!("{indent}Expr");
        self.indent_level += 1;
        match e {
            Expr::UnaryArithmetic(ref u) => self.visit_unary(u),
            Expr::BinaryArithmetic(ref b) => self.visit_binary(b),
            Expr::FunctionCall(ref f) => self.visit_function(f),
            Expr::Atom(ref a) => self.visit_atom(a),
        }
        self.indent_level -= 1;
    }

    fn visit_unary(&mut self, u: &UnaryArithmetic) {
        let indent = " ".repeat(self.get_indent());
        match u.op {
            UnaryOp::Pos => println!("{indent}Pos"),
            UnaryOp::Neg => println!("{indent}Neg"),
            UnaryOp::Fac => println!("{indent}Fac"),
        }
        println!("{indent}Unary");
        self.indent_level += 1;
        self.visit_expr(&u.value);
        self.indent_level -= 1;
    }

    fn visit_binary(&mut self, b: &BinaryArithmetic) {
        let indent = " ".repeat(self.get_indent());
        match b.op {
            BinaryOp::Add => println!("{indent}Add"),
            BinaryOp::Sub => println!("{indent}Sub"),
            BinaryOp::Mul => println!("{indent}Mul"),
            BinaryOp::Div => println!("{indent}Div"),
        }
        self.indent_level += 1;
        self.visit_expr(&b.left);
        self.visit_expr(&b.right);
        self.indent_level -= 1;
    }

    fn visit_function(&mut self, f: &FunctionCall) {
        let indent = " ".repeat(self.get_indent());
        let func_name = &f.name;
        println!("{indent}Function {func_name}");
        self.indent_level += 1;
        for arg in &f.args {
            self.visit_expr(arg);
        }
        self.indent_level -= 1;
    }

    fn visit_atom(&mut self, a: &Atom) {
        let indent = " ".repeat(self.get_indent());
        match a {
            Atom::Ident(ref id) => println!("{indent}Identifier {id}"),
            Atom::Number(ref n) => println!("{indent}Number {n}"),
        }
    }
}
