use super::Visitor;
use crate::ast::{Atom, BinaryArithmetic, BinaryOp, Expr, FunctionCall, UnaryArithmetic, UnaryOp};
use crate::symbols::{SymbolError, SymbolTable};
use std::f64::consts;

pub type Func<T> = fn(Vec<T>) -> T;

#[derive(Debug)]
pub struct Calculator {
    variables: SymbolTable<f64>,
    functions: SymbolTable<Func<f64>>,
    operand_stack: Vec<f64>,
}

#[derive(Debug)]
pub enum CalculatorError {
    StackEmpty,
    StackNotEmpty,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            variables: SymbolTable::new(),
            functions: SymbolTable::new(),
            operand_stack: Vec::new(),
        }
    }

    pub fn define_variable(&mut self, name: &str, value: f64) -> Result<(), SymbolError> {
        self.variables.define(name, value)
    }

    pub fn define_function(&mut self, name: &str, value: Func<f64>) -> Result<(), SymbolError> {
        self.functions.define(name, value)
    }

    pub fn preset(&mut self) -> Result<(), SymbolError> {
        self.define_variable("PI", consts::PI)?;
        self.define_variable("TAU", consts::TAU)?;
        self.define_variable("E", consts::E)?;

        self.define_function("log", |argv| f64::log(argv[1], argv[0]))?;
        self.define_function("ln", |argv| f64::ln(argv[0]))?;
        self.define_function("log_2", |argv| f64::log2(argv[0]))?;
        self.define_function("log_10", |argv| f64::log10(argv[0]))?;
        self.define_function("add", |argv| argv[0] + argv[1])?;
        self.define_function("sum", |argv| argv.iter().sum())?;
        self.define_function("pow", |argv| f64::powf(argv[0], argv[1]))?;
        self.define_function("sqrt", |argv| f64::sqrt(argv[0]))?;
        self.define_function("max", |argv| argv.iter().copied().fold(f64::NAN, f64::max))?;
        self.define_function("min", |argv| argv.iter().copied().fold(f64::NAN, f64::min))?;
        self.define_function("sin", |argv| f64::sin(argv[0]))?;
        self.define_function("cos", |argv| f64::cos(argv[0]))?;
        self.define_function("tan", |argv| f64::tan(argv[0]))?;
        self.define_function("floor", |argv| f64::floor(argv[0]))?;
        self.define_function("ceil", |argv| f64::ceil(argv[0]))?;
        self.define_function("abs", |argv| f64::abs(argv[0]))?;
        Ok(())
    }

    pub fn result(&mut self) -> Result<f64, CalculatorError> {
        let value = match self.operand_stack.pop() {
            Some(value) => Ok(value),
            None => Err(CalculatorError::StackEmpty),
        };
        if !self.operand_stack.is_empty() {
            return Err(CalculatorError::StackNotEmpty);
        }
        value
    }
}

impl Visitor<()> for Calculator {
    fn visit_expr(&mut self, e: &Expr) {
        match e {
            Expr::UnaryArithmetic(ref u) => self.visit_unary(u),
            Expr::BinaryArithmetic(ref b) => self.visit_binary(b),
            Expr::FunctionCall(ref f) => self.visit_function(f),
            Expr::Atom(ref a) => self.visit_atom(a),
        }
    }

    fn visit_unary(&mut self, u: &UnaryArithmetic) {
        self.visit_expr(&u.value);
        match u.op {
            UnaryOp::Pos => (),
            UnaryOp::Neg => {
                let value = self.operand_stack.pop();
                self.operand_stack.push(-value.unwrap());
            }
            UnaryOp::Fac => {
                let value = self.operand_stack.pop();
                self.operand_stack
                    .push(factorial(value.unwrap() as u64) as f64);
            }
        }
    }

    fn visit_binary(&mut self, b: &BinaryArithmetic) {
        self.visit_expr(&b.lhs);
        self.visit_expr(&b.rhs);

        let rhs = self.operand_stack.pop().unwrap();
        let lhs = self.operand_stack.pop().unwrap();
        match b.op {
            BinaryOp::Add => self.operand_stack.push(lhs + rhs),
            BinaryOp::Sub => self.operand_stack.push(lhs - rhs),
            BinaryOp::Mul => self.operand_stack.push(lhs * rhs),
            BinaryOp::Div => self.operand_stack.push(lhs / rhs),
        }
    }

    fn visit_function(&mut self, f: &FunctionCall) {
        let argc = f.args.len();
        for arg in &f.args {
            self.visit_expr(arg);
        }
        let func_name = &f.name;
        let func = self.functions.get(func_name).unwrap();
        let mut argv = Vec::new();
        for _ in 0..argc {
            argv.push(self.operand_stack.pop().unwrap());
        }
        argv.reverse();
        self.operand_stack.push(func(argv));
    }

    fn visit_atom(&mut self, a: &Atom) {
        match a {
            Atom::Ident(ref id) => self.operand_stack.push(self.variables.get(id).unwrap()),
            Atom::Number(ref n) => self.operand_stack.push(*n),
        }
    }
}

fn factorial(num: u64) -> u64 {
    match (1..=num).reduce(|accum, item| accum * item) {
        Some(x) => x,
        None => num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::calc_parser;
    use crate::utils::assert_close;

    #[test]
    fn calc_number() {
        let input = "2.333333333";
        let parsed_input = calc_parser::expr(input).unwrap();
        let mut calculator = Calculator::new();
        calculator.visit_expr(&parsed_input);
        assert_close(calculator.result().unwrap(), 2.333333333);
    }

    #[allow(clippy::approx_constant)]
    #[test]
    fn custom_variable() {
        let input = "var";
        let value = 3.141592653589;
        let parsed_input = calc_parser::expr(input).unwrap();
        let mut calculator = Calculator::new();
        assert_eq!(calculator.define_variable(input, value), Ok(()));
        calculator.visit_expr(&parsed_input);
        assert_close(calculator.result().unwrap(), value);
    }

    #[test]
    fn custom_function() {
        let a = 2.3333;
        let b = 3.2222;
        let input = format!("mul({a}, {b})");
        let parsed_input = calc_parser::expr(&input).unwrap();
        let mut calculator = Calculator::new();
        assert_eq!(
            calculator.define_function("mul", |args| args[0] * args[1]),
            Ok(())
        );
        calculator.visit_expr(&parsed_input);
        assert_close(calculator.result().unwrap(), a * b);
    }

    #[test]
    fn calc_preset() {
        let input = "sqrt(PI * E) - log(2, 3)";
        let value = f64::sqrt(consts::PI * consts::E) - f64::log(3.0, 2.0);
        let parsed_input = calc_parser::expr(input).unwrap();
        let mut calculator = Calculator::new();
        assert_eq!(calculator.preset(), Ok(()));
        calculator.visit_expr(&parsed_input);
        assert_close(calculator.result().unwrap(), value);
    }
}
