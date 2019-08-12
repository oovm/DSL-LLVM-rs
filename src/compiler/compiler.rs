use crate::AST;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, PointerValue};
use inkwell::{FloatPredicate, OptimizationLevel};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub is_op: bool,
    pub prec: usize,
    pub is_anon: bool,
}

#[derive(Debug)]
pub struct Compiler<'a> {
    pub context: &'a Context,
    pub builder: &'a Builder,
    pub module: &'a Module,
    pub function: &'a Function,
    pub fpm: &'a PassManager<FunctionValue>,
    variables: HashMap<String, PointerValue>,
    fn_value: Option<FunctionValue>,
}

type ReturnValueType<T> = Result<T, &'static str>;

impl<'a> Compiler<'a> {
    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue> {
        self.module.get_function(name)
    }
    #[inline]
    fn fn_value(&self) -> FunctionValue {
        self.fn_value.unwrap()
    }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_allocation(&self, name: &str) -> PointerValue {
        let builder = self.context.create_builder();
        let entry = self.fn_value().get_first_basic_block().unwrap();
        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(&entry),
        }
        builder.build_alloca(self.context.f64_type(), name)
    }

    fn register_value(&mut self, expr: &AST) {
        match *expr {
            AST::Decimal(ref x) => self.context.f64_type().const_float(*x),
            _ => {
                println!("unimplemented {:?}", expr);
                self.context.f64_type().const_zero()
            }
        };
    }
}
