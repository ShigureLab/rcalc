use super::Visitor;
use crate::ast::{Atom, BinaryArithmetic, BinaryOp, Expr, FunctionCall, UnaryArithmetic, UnaryOp};
use crate::symbols::{SymbolError, SymbolTable};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, FloatType};
use inkwell::values::{BasicMetadataValueEnum, FloatValue, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::OptimizationLevel;
use std::f64::consts;

pub type CalcMain = unsafe extern "C" fn() -> f64;
pub const CALC_ENTRYPOINT: &str = "calc_main";
pub type FuncLLVM<'a, T> = fn(Vec<T>, &Builder<'a>) -> T;

#[derive(Debug)]
pub struct CalculatorJIT<'ctx> {
    variables: SymbolTable<PointerValue<'ctx>>,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CalculatorJIT<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("calc");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        CalculatorJIT {
            variables: SymbolTable::new(),
            context,
            module,
            builder: context.create_builder(),
            execution_engine,
        }
    }

    #[inline]
    fn double(&self) -> FloatType<'ctx> {
        self.context.f64_type()
    }

    pub fn define_variable(&mut self, name: &str, value: f64) -> Result<(), SymbolError> {
        let var = self
            .module
            .add_global(self.double(), Some(AddressSpace::default()), name);

        let initial_value = self.double().const_float(value);
        var.set_initializer(&initial_value);

        let alloca = var.as_pointer_value();
        self.variables.define(name, alloca)?;
        Ok(())
    }

    fn get_variable(&mut self, name: &str) -> Result<FloatValue<'ctx>, SymbolError> {
        let alloca = self.variables.get(name)?;
        let var = self
            .builder
            .build_load(self.double(), alloca, name)
            .expect("Failed to load variable")
            .into_float_value();

        Ok(var)
    }

    pub fn define_function(
        &mut self,
        name: &str,
        argc: usize,
        func: FuncLLVM<'ctx, FloatValue<'ctx>>,
    ) -> Result<(), SymbolError> {
        let ret_type = self.double();
        let args_types = std::iter::repeat(ret_type)
            .take(argc)
            .map(|f| f.into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = self.double().fn_type(args_types, false);
        let fn_val = self.module.add_function(name, fn_type, None);
        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);

        let mut args = Vec::with_capacity(argc);
        for i in 0..argc as u32 {
            args.push(fn_val.get_nth_param(i).unwrap().into_float_value())
        }

        let ret_val = func(args, &self.builder);

        self.builder
            .build_return(Some(&ret_val))
            .expect("Failed to build return");

        Ok(())
    }

    pub fn get_function(&mut self, name: &str) -> Result<FunctionValue<'ctx>, SymbolError> {
        match self.module.get_function(name) {
            Some(func) => Ok(func),
            None => Err(SymbolError::UnDefinition),
        }
    }

    pub fn preset(&mut self) -> Result<(), SymbolError> {
        self.define_variable("PI", consts::PI)?;
        self.define_variable("TAU", consts::TAU)?;
        self.define_variable("E", consts::E)?;

        self.define_function("add", 2, |args, builder| {
            let a = args[0];
            let b = args[1];
            builder
                .build_float_add(a, b, "add")
                .expect("Failed to build add")
        })?;
        // Other ops is too complex to hand-write LLVM code...
        Ok(())
    }

    pub fn compile(&mut self, ast: &Expr) -> Option<JitFunction<CalcMain>> {
        let sig = self.double().fn_type(&[], false);
        let func = self.module.add_function(CALC_ENTRYPOINT, sig, None);
        let basic_block = self.context.append_basic_block(func, "entry");

        self.builder.position_at_end(basic_block);

        let ret = self.visit_expr(ast);
        self.builder
            .build_return(Some(&ret))
            .expect("Failed to build return");

        unsafe { self.execution_engine.get_function(CALC_ENTRYPOINT).ok() }
    }
}

impl<'ctx> Visitor<FloatValue<'ctx>> for CalculatorJIT<'ctx> {
    fn visit_expr(&mut self, e: &Expr) -> FloatValue<'ctx> {
        match e {
            Expr::UnaryArithmetic(ref u) => self.visit_unary(u),
            Expr::BinaryArithmetic(ref b) => self.visit_binary(b),
            Expr::FunctionCall(ref f) => self.visit_function(f),
            Expr::Atom(ref a) => self.visit_atom(a),
        }
    }
    fn visit_unary(&mut self, u: &UnaryArithmetic) -> FloatValue<'ctx> {
        let value = self.visit_expr(&u.value);

        match u.op {
            UnaryOp::Pos => value,
            UnaryOp::Neg => self
                .builder
                .build_float_neg(value, "neg")
                .expect("Failed to build neg"),
            UnaryOp::Fac => unimplemented!(),
        }
    }

    fn visit_binary(&mut self, b: &BinaryArithmetic) -> FloatValue<'ctx> {
        let lhs = self.visit_expr(&b.lhs);
        let rhs = self.visit_expr(&b.rhs);
        match b.op {
            BinaryOp::Add => self
                .builder
                .build_float_add(lhs, rhs, "add")
                .expect("Failed to build add"),
            BinaryOp::Sub => self
                .builder
                .build_float_sub(lhs, rhs, "sub")
                .expect("Failed to build sub"),
            BinaryOp::Mul => self
                .builder
                .build_float_mul(lhs, rhs, "mul")
                .expect("Failed to build mul"),
            BinaryOp::Div => self
                .builder
                .build_float_div(lhs, rhs, "div")
                .expect("Failed to build div"),
        }
    }

    fn visit_function(&mut self, f: &FunctionCall) -> FloatValue<'ctx> {
        let argc = f.args.len();
        let func = self.get_function(&f.name).unwrap();
        let mut argv = Vec::with_capacity(argc);

        for i in 0..argc {
            argv.push(self.visit_expr(&f.args[i]))
        }

        let argsv: Vec<BasicMetadataValueEnum> =
            argv.iter().by_ref().map(|&val| val.into()).collect();

        let ret_val = self
            .builder
            .build_call(func, argsv.as_slice(), "tmp")
            .expect("Unable to call function")
            .try_as_basic_value()
            .left()
            .unwrap();

        ret_val.into_float_value()
    }

    fn visit_atom(&mut self, a: &Atom) -> FloatValue<'ctx> {
        match a {
            Atom::Ident(ref id) => self.get_variable(id).unwrap(),
            Atom::Number(ref n) => self.double().const_float(*n),
        }
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
        let context = Context::create();
        let mut calculator_jit = CalculatorJIT::new(&context);
        let calc_main = calculator_jit.compile(&parsed_input).unwrap();
        let result = unsafe { calc_main.call() };
        assert_close(result, 2.333333333);
    }

    #[allow(clippy::approx_constant)]
    #[test]
    fn custom_variable() {
        let input = "var";
        let value = 3.141592653589;
        let parsed_input = calc_parser::expr(input).unwrap();
        let context = Context::create();
        let mut calculator_jit = CalculatorJIT::new(&context);
        assert_eq!(calculator_jit.define_variable(input, value), Ok(()));
        let calc_main = calculator_jit.compile(&parsed_input).unwrap();
        let result = unsafe { calc_main.call() };
        assert_close(result, value);
    }

    #[test]
    fn custom_function() {
        let a = 2.3333;
        let b = 3.2222;
        let input = format!("mul({a}, {b})");
        let parsed_input = calc_parser::expr(&input).unwrap();
        let context = Context::create();
        let mut calculator_jit = CalculatorJIT::new(&context);
        assert_eq!(
            calculator_jit.define_function("mul", 2, |args, builder| {
                let a = args[0];
                let b = args[1];
                builder
                    .build_float_mul(a, b, "mul")
                    .expect("Failed to build mul")
            }),
            Ok(())
        );
        let calc_main = calculator_jit.compile(&parsed_input).unwrap();
        let result = unsafe { calc_main.call() };
        assert_close(result, a * b);
    }

    #[test]
    fn calc_preset() {
        let input = "1 - add(PI * E, TAU)";
        let value = 1. - (consts::PI * consts::E + consts::TAU);
        let parsed_input = calc_parser::expr(input).unwrap();
        let context = Context::create();
        let mut calculator_jit = CalculatorJIT::new(&context);
        assert_eq!(calculator_jit.preset(), Ok(()));
        let calc_main = calculator_jit.compile(&parsed_input).unwrap();
        let result = unsafe { calc_main.call() };
        assert_close(result, value);
    }
}
