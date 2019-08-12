use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, PointerValue};
use inkwell::{FloatPredicate, OptimizationLevel};
use std::collections::HashMap;
use crate::AST;

pub struct Compiler<'a> {
    pub context: &'a Context,
    pub builder: &'a Builder,
    pub module: &'a Module,
    pub function: &'a Function,
    variables: HashMap<String, PointerValue>,
    fn_value_opt: Option<FunctionValue>,
}

impl<'a> Compiler<'a> {
    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue> {
        self.module.get_function(name)
    }
    #[inline]
    fn fn_value(&self) -> FunctionValue {
        self.fn_value_opt.unwrap()
    }
    fn compile_expr(&mut self, expr: &AST) -> Result<FloatValue, &'static str> {
        let exec = match *expr {
            AST::Decimal(ref x) => self.context.f64_type().const_float(*x),
            _ => println!("unimplemented {:?}", expr),
        };
        Ok(exec)
    }
}
