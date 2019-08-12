#![feature(custom_attribute)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]
#![feature(int_error_matching)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[cfg(feature = "colored")]
extern crate colored;
extern crate inkwell;

mod ast;
mod compiler;
mod parser;
#[allow(unused_imports)]
pub mod utils;
mod value;
mod vm;

pub use ast::AST;
pub use parser::{Parser, Rule};
