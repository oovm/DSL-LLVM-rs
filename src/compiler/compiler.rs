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
    pub is_anonymous: bool,
}

impl Default for Prototype {
    fn default() -> Self {
        Prototype {
            name: String::new(),
            args: vec![],
            is_anonymous: true,
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

impl<'a> Default for Compiler<'a> {
    fn default() -> Self {
        unimplemented!();
        /*
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("ts");
        // Create FPM
        let fpm = PassManager::create(&module);
        let f = Function::default();
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
            function: &f,
            fpm: &fpm,
            variables: HashMap::new(),
            fn_value: None,
        }
        */
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
        builder.build_alloca(self.context.i64_type(), name)
    }

    fn create_prototype(&self, f: &Prototype) -> Result<FunctionValue, &'static str> {
        //ret_type: BasicTypeEnum
        let ret_type = self.context.i64_type();
        let args_types: Vec<BasicTypeEnum> = std::iter::repeat(ret_type)
            .take(f.args.len())
            .map(|f| f.into())
            .collect();
        let fn_type = self
            .context
            .i64_type()
            .fn_type(args_types.as_slice(), false);
        let fn_value = self.module.add_function(f.name.as_str(), fn_type, None);
        // Args
        for (i, arg) in fn_value.get_param_iter().enumerate() {
            arg.into_float_value().set_name(f.args[i].as_str());
        }
        Ok(fn_value)
    }

    fn create_fn(&mut self) -> Result<FunctionValue, &'static str> {
        let proto = &self.function.prototype;
        let function = self.create_prototype(proto)?;
        // pass if empty
        if self.function.body.is_none() {
            return Ok(function);
        }
        // add entry
        let entry = self.context.append_basic_block(&function, "entry");
        self.builder.position_at_end(&entry);
        self.fn_value = Some(function);
        self.variables.reserve(proto.args.len());
        // add args
        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = proto.args[i].as_str();
            let alloc = self.create_allocation(arg_name);
            self.builder.build_store(alloc, arg);
            self.variables.insert(proto.args[i].clone(), alloc);
        }
        // compile body
        let body = self.compile_ast(self.function.body.as_ref().unwrap())?;
        self.builder.build_return(Some(&body));
        // delete if verify failed
        if function.verify(true) {
            self.fpm.run_on(&function);
            Ok(function)
        } else {
            unsafe {
                function.delete();
            }
            Err("Invalid generated function.")
        }
    }

    fn compile_ast(&mut self, expr: &AST) -> Result<BasicValueEnum, &'static str> {
        match *expr {
            AST::Decimal(x) => {
                let v = self.context.f64_type().const_float(x);
                Ok(BasicValueEnum::FloatValue(v))
            }
            AST::Integer(i) => {
                let v = self.context.i64_type().const_int(i as u64, false);
                Ok(BasicValueEnum::IntValue(v))
            }
            AST::Boolean(b) => {
                let v = self.context.bool_type().const_int(b as u64, false);
                Ok(BasicValueEnum::IntValue(v))
            }
            AST::String(ref s) => {
                let v = self.context.const_string(s, false);
                Ok(BasicValueEnum::VectorValue(v))
            }
            AST::Symbol(ref name) => {
                let v = self.variables.get(name.as_str());
                match v {
                    Some(var) => Ok(self.builder.build_load(*var, name.as_str())),
                    None => Err("Variable not found"),
                }
            }
            AST::Infix(ref infix, ref left, ref right) => {
                let op = infix.as_str();
                let lhs = self.compile_ast(left.as_ref())?;
                let rhs = self.compile_ast(right.as_ref())?;
                // may int
                match op {
                    "+" => Ok(self.builder.build_int_add(lhs, rhs, "i_add")),
                    "-" => Ok(self.builder.build_int_sub(lhs, rhs, "i_sub")),
                    "=" => {
                        // handle assignement
                        let var = self
                            .variables
                            .get(lhs.as_str())
                            .ok_or("Undefined variable.")?;
                        self.builder.build_store(*var, rhs);
                        Ok(rhs)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
