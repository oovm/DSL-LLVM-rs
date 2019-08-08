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

#[test]
fn integer_zero() {
    const TEXT: &str = "0";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn integer_nonzero() {
    const TEXT: &str = "18446744073709551616"; // will not panic in lexer
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn integer_leading_zero() {
    const TEXT: &str = "01"; // TODO: This is wrong in js
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn integer_0x() {
    const TEXT: &str = "0x0f";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn integer_0o() {
    const TEXT: &str = "0x07";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn integer_0b() {
    const TEXT: &str = "0x01";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn float() {
    const TEXT: &str = "0.0";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn float_bad_dot() {
    const TEXT: &str = ".0";
    token_print(TEXT, Rule::program);
    //panic!()
}

#[test]
fn float_bad_integer() {
    const TEXT: &str = "0.";
    token_print(TEXT, Rule::program);
    //panic!()
}