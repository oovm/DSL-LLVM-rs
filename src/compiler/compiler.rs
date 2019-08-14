use crate::AST;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
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

pub enum IR {
    Void,
    Decimal(FloatValue),
    Integer(IntValue),
}

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

    fn compiler(&mut self, expr: &AST) -> Result<BasicValueEnum, &'static str> {
        match *expr {
            AST::Decimal(ref x) => {
                let v = self.context.f64_type().const_float(*x);
                Ok(BasicValueEnum::FloatValue(v))
            }
            AST::Integer(ref i) => {
                let v = self.context.i64_type().const_int(*i as u64, false);
                OK(BasicValueEnum::IntValue(v))
            }
            AST::Symbol(ref name) => match self.variables.get(name.as_str()) {
                Some(var) => {
                    let s = self.builder.build_load(*var, name.as_str());
                    match s.get_type() {
                        BasicTypeEnum::IntType => Ok(BasicValueEnum::IntValue(s.into_int_value())),
                        BasicTypeEnum::FloatType => Ok(BasicValueEnum::FloatValue(s.into_float_value())),
                        _ => unimplemented!(),
                    }
                }
                None => Err("Could not find a matching variable."),
            },
            _ => unimplemented!(),
        }
    }
}
