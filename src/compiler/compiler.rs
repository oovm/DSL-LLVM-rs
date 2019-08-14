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
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
    pub is_anon: bool,
}

impl Default for Prototype {
    fn default() -> Self {
        Prototype {
            name: String::new(),
            args: vec![],
            is_anon: false,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Option<AST>,
}

impl Default for Function {
    fn default() -> Self {
        Function {
            prototype: Prototype::default(),
            body: None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Compiler<'a> {
    pub context: &'a Context,
    pub builder: &'a Builder,
    pub module: &'a Module,
    pub function: &'a Function,
    pub fpm: &'a PassManager<FunctionValue>,
    variables: HashMap<String, PointerValue>,
    fn_value: Option<FunctionValue>,
}

impl Default for Compiler {
    fn default() -> Self {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("ts");
        // Create FPM
        let fpm = PassManager::create(&module);
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.initialize();
        Compiler {
            context: &context,
            builder: &builder,
            module: &module,
            function: &Function::default(),
            fpm: &fpm,
            variables: HashMap::new(),
            fn_value: None,
        }
    }
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

    fn compile_ast(&mut self, expr: &AST) -> Result<BasicValueEnum, &'static str> {
        match *expr {
            AST::Decimal(ref x) => {
                let v = self.context.f64_type().const_float(*x);
                Ok(BasicValueEnum::FloatValue(v))
            }
            AST::Integer(ref i) => {
                let v = self.context.i64_type().const_int(*i as u64, false);
                Ok(BasicValueEnum::IntValue(v))
            }
            AST::Symbol(ref name) => {
                let v = self.variables.get(name.as_str());
                match v {
                    Some(var) => Ok(self.builder.build_load(*var, name.as_str())),
                    None => Err("Could not find a matching variable."),
                }
            }
            _ => unimplemented!(),
        }
    }
}
