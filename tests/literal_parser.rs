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
    let result = AST::Stack([AST::Integer(1)].to_vec());
    assert_eq!(parse(TEXT),result)
}

#[test]
fn integer_0x() {
    const TEXT: &str = "0x0f";
    parse(TEXT);
    //panic!()
}

#[test]
fn integer_0o() {
    const TEXT: &str = "0x07";
    parse(TEXT);
    //panic!()
}

#[test]
fn integer_0b() {
    const TEXT: &str = "0x01";
    parse(TEXT);
    //panic!()
}

#[test]
fn float() {
    const TEXT: &str = "0.0";
    let result = AST::Stack([AST::Decimal(0.0)].to_vec());
    assert_eq!(parse(TEXT),result)
}

#[test]
fn float_bad_dot() {
    const TEXT: &str = ".0";
    let result = AST::Stack([AST::Decimal(0.0)].to_vec());
    assert_eq!(parse(TEXT),result)
}

#[test]
fn float_bad_integer() {
    const TEXT: &str = "0.";
    let result = AST::Stack([AST::Decimal(0.0)].to_vec());
    assert_eq!(parse(TEXT),result)
}

#[test]
fn string_single() {
    const TEXT: &str = "\"'\"";
    parse(TEXT);
    //panic!()
}

#[test]
fn string_double() {
    const TEXT: &str = "'\"'";
    parse(TEXT);
    //panic!()
}
