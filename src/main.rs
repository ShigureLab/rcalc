mod ast;
mod parser;
mod symbols;
mod visitor;

use inkwell::context::Context;
use parser::calc_parser;
use std::error::Error;
use visitor::{Calculator, CalculatorJIT, PrettyPrinter, Visitor};

fn main() -> Result<(), Box<dyn Error>> {
    let input = "mul(PI, E)";

    match calc_parser::expr(&input) {
        Ok(parsed_input) => {
            println!("{:?}", parsed_input);

            // pretty printer
            let mut printer = PrettyPrinter::new(2);
            printer.visit_expr(&parsed_input);

            // calc
            let mut calculator = Calculator::new();
            calculator.preset().unwrap();
            // User defined variables and functions
            calculator.define_variable(&"a".into(), 222.0).unwrap();
            calculator
                .define_function(&"mul".into(), |args| args[0] * args[1])
                .unwrap();
            calculator.visit_expr(&parsed_input);
            let result = calculator.result().unwrap();

            println!("Calculator Interpret result: {}", result);

            // JIT calc
            let context = Context::create();
            let mut calculator_jit = CalculatorJIT::new(&context);
            calculator_jit.preset().unwrap();
            // User defined variables and functions
            calculator_jit.define_variable(&"a".into(), 222.0).unwrap();
            calculator_jit
                .define_function(&"mul".into(), 2, |args, builder| {
                    let a = args[0];
                    let b = args[1];
                    builder.build_float_mul(a, b, "mul")
                })
                .unwrap();
            let calc_main = calculator_jit.compile(&parsed_input).unwrap();
            let result = unsafe { calc_main.call() };
            println!("JIT compile result: {}", result);
            Ok(())
        }
        Err(e) => {
            let (err_line, err_col) = (e.location.line, e.location.column);
            let error_line = input.split("\n").collect::<Vec<_>>()[err_line - 1];
            println!(
                "Unexpected char `{}` at line {}, column {}:",
                error_line.chars().nth(err_col - 1).unwrap(),
                err_line,
                err_col
            );
            println!("{}", error_line);
            println!("{}{}", " ".repeat(err_col - 1), "^");
            println!("Excepct chars: {:?}", e.expected);
            Ok(())
        }
    }
}
