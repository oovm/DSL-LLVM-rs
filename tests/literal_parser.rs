#![allow(unused_imports)]
extern crate core;
extern crate pest;
extern crate typescript;

use pest::Parser;
use typescript::utils::parse;
use typescript::{Rule, AST};

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
    let result = AST::Stack([AST::from(1)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn integer_0x() {
    const TEXT: &str = "0xff";
    let result = AST::Stack([AST::from(255)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn integer_0o() {
    const TEXT: &str = "0o77";
    let result = AST::Stack([AST::from(63)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn integer_0b() {
    const TEXT: &str = "0b11";
    let result = AST::Stack([AST::from(3)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float() {
    const TEXT: &str = "0.0";
    let result = AST::Stack([AST::from(0.0)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float_bad_dot() {
    const TEXT: &str = ".0";
    let result = AST::Stack([AST::from(0.0)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float_bad_integer() {
    const TEXT: &str = "0.";
    let result = AST::Stack([AST::from(0.0)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn float_exp() {
    const TEXT: &str = "2e5";
    let result = AST::Stack([AST::from(200000.0)].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn string_single() {
    const TEXT: &str = "\"'\"";
    let result = AST::Stack([AST::from("'")].to_vec());
    assert_eq!(parse(TEXT), result)
}

#[test]
fn string_double() {
    const TEXT: &str = "'\"'";
    let result = AST::Stack([AST::from("\"")].to_vec());
    assert_eq!(parse(TEXT), result)
}
