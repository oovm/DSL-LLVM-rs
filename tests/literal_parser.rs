#![allow(unused_imports)]
extern crate core;
extern crate pest;
extern crate typescript;

use pest::Parser;
use typescript::utils::parse;
use typescript::{Rule, AST};

macro_rules! stack {
            ($name:expr) => {
                AST::Stack(vec![$name])
            };
            ($($x:expr,)*) => (AST::Stack(vec![$($x),*]))
        }

#[test]
fn symbol() {
    const TEXT: &str = "a";
    parse(TEXT);
    //panic!()
}

#[test]
fn symbol_underline() {
    const TEXT: &str = "_a";
    parse(TEXT);
    //panic!()
}

#[test]
fn symbol_dollar() {
    const TEXT: &str = "$a";
    parse(TEXT);
    //panic!()
}

#[test]
fn integer_zero() {
    const TEXT: &str = "0";
    parse(TEXT);
    //panic!()
}

#[test]
#[should_panic]
fn integer_nonzero() {
    const TEXT: &str = "18446744073709551616";
    parse(TEXT);
}

#[test]
fn integer_leading_zero() {
    const TEXT: &str = "01"; // TODO: This is wrong in js
    let result = stack![AST::from(1)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn integer_0x() {
    const TEXT: &str = "0xff";
    let result = stack![AST::from(255)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn integer_0o() {
    const TEXT: &str = "0o77";
    let result = stack![AST::from(63)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn integer_0b() {
    const TEXT: &str = "0b11";
    let result = stack![AST::from(3)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float() {
    const TEXT: &str = "0.0";
    let result = stack![AST::from(0.0)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float_bad_dot() {
    const TEXT: &str = ".0";
    let result = stack![AST::from(0.0)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float_bad_integer() {
    const TEXT: &str = "0.";
    let result = stack![AST::from(0.0)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float_exp() {
    const TEXT: &str = "2e5";
    let result = stack![AST::from(200000.0)];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn string_single() {
    const TEXT: &str = "\"'\"";
    let result = stack![AST::from("'")];
    assert_eq!(parse(TEXT), result)
}

#[test]
fn string_double() {
    const TEXT: &str = "'\"'";
    let result = stack![AST::from("\"")];
    assert_eq!(parse(TEXT), result)
}
