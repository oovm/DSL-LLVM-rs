#![feature(custom_attribute)]
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[cfg(feature = "colored")]
extern crate colored;

mod ast;
mod parser;
#[allow(unused_imports)]
pub mod utils;

pub use ast::AST;
pub use parser::{Parser, Rule};
