#![allow(unused_imports)]
extern crate core;
extern crate pest;
extern crate typescript;

use pest::Parser;
use typescript::utils::token_print;
use typescript::{Rule, AST};

#[test]
fn symbol() {
    const TEXT: &str = "a";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn symbol_underline() {
    const TEXT: &str = "_a";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn symbol_dollar() {
    const TEXT: &str = "$a";
    token_print(TEXT, Rule::program);
    //panic!()
}
