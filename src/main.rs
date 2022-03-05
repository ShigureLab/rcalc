mod ast;
mod cli;
mod parser;
mod symbols;
mod utils;
mod visitor;

use clap::Parser;
use cli::{get_variables, Cli};
use inkwell::context::Context;
use parser::calc_parser;
use std::error::Error;
use visitor::{Calculator, CalculatorJIT, PrettyPrinter, Visitor};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let variables = get_variables(&cli);

    let input = cli.expr;
    // parse to AST
    match calc_parser::expr(&input) {
        Ok(parsed_input) => {
            if cli.verbose && !cli.pure {
                // pretty printer
                let mut printer = PrettyPrinter::new(2);
                let separator = "~".repeat(20);
                println!("{separator}");
                println!("AST:");
                printer.visit_expr(&parsed_input);
                println!("{separator}");
            }

            let result;
            if !cli.jit {
                // calc
                let mut calculator = Calculator::new();
                calculator.preset().unwrap();
                // User defined variables and functions
                // calculator.define_variable(&"a".into(), 222.0).unwrap();
                // calculator
                //     .define_function(&"mul".into(), |args| args[0] * args[1])
                //     .unwrap();
                for (name, value) in variables {
                    calculator.define_variable(&name, value).unwrap();
                }
                calculator.visit_expr(&parsed_input);
                result = calculator.result().unwrap();

                if !cli.pure {
                    print!("Calculator Interpret result: ");
                };
            } else {
                // JIT calc
                let context = Context::create();
                let mut calculator_jit = CalculatorJIT::new(&context);
                calculator_jit.preset().unwrap();
                // User defined variables and functions
                // calculator_jit.define_variable(&"a".into(), 222.0).unwrap();
                // calculator_jit
                //     .define_function(&"mul".into(), 2, |args, builder| {
                //         let a = args[0];
                //         let b = args[1];
                //         builder.build_float_mul(a, b, "mul")
                //     })
                //     .unwrap();
                for (name, value) in variables {
                    calculator_jit.define_variable(&name, value).unwrap();
                }
                let calc_main = calculator_jit.compile(&parsed_input).unwrap();
                result = unsafe { calc_main.call() };

                if !cli.pure {
                    print!("JIT compile result: ");
                };
            }
            match cli.precision {
                Some(precision) => {
                    print!("{result:.precision$}")
                }
                None => print!("{result}"),
            }

            if !cli.pure {
                println!();
            }
            Ok(())
        }
        Err(e) => {
            let (err_line, err_col) = (e.location.line, e.location.column);
            let error_line = input.split('\n').collect::<Vec<_>>()[err_line - 1];
            println!(
                "Unexpected char `{}` at line {}, column {}:",
                error_line.chars().nth(err_col - 1).unwrap(),
                err_line,
                err_col
            );
            println!("{}", error_line);
            println!("{}^", " ".repeat(err_col - 1));
            println!("Excepct chars: {:?}", e.expected);
            Ok(())
        }
    }
}
