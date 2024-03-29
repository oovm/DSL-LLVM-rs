#[allow(unused_imports)]
use crate::{Parser, Rule, AST};
use colored::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser as Pest;
use std::convert::TryFrom;
use std::str::FromStr;

pub fn token_print(s: &str, rule: Rule) {
    let pairs = Parser::parse(rule, s).unwrap_or_else(|e| panic!("{}", e));
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => continue,
            _ => (),
        }
        println!("{} {:?}", "Rule:".green(), pair.as_rule());
        println!("{} {:?}", "Span:".green(), pair.as_span());
        println!("{} {}", "Text:".green(), pair.as_str());
        for inner_pair in pair.into_inner() {
            println!("    {} {:?}", "Inner:".cyan(), inner_pair.as_rule());
            println!("           {:?}", inner_pair.as_span());
        }
    }
}

pub fn parse(s: &str) -> AST {
    let pairs = Parser::parse(Rule::program, s).unwrap_or_else(|e| panic!("{}", e));
    let mut stack: Vec<AST> = vec![];
    for pair in pairs {
        macro_rules! push {
            ($handler:ident) => {
                stack.push($handler(pair))
            };
        }
        match pair.as_rule() {
            Rule::EOI => (),
            Rule::Integer => push!(parse_integer),
            Rule::Byte_OCT | Rule::Byte_BIN | Rule::Byte_HEX => push!(parse_byte),
            Rule::Decimal | Rule::DecimalBad | Rule::Exponent => push!(parse_float),
            Rule::SingleEscape | Rule::DoubleEscape => push!(parse_string),
            Rule::SYMBOL => push!(parse_symbol),
            _ => println!("unimplemented: {:?}", pair.as_rule()),
        }
    }
    AST::Stack(stack)
}

fn parse_integer(pair: Pair<Rule>) -> AST {
    let s = pair.as_str();
    match i64::from_str(s) {
        Ok(a) => AST::Integer(a),
        Err(e) => {
            println!(
                "ParseError: {} is {:?} at {:?}",
                s,
                e.kind(),
                pair.as_span().start_pos()
            );
            panic!(e)
        }
    }
}

fn parse_byte(pair: Pair<Rule>) -> AST {
    let s = pair.as_str();
    let base = s.chars().nth(1).unwrap();
    let num = &s[2..s.len()];
    let r = match base {
        'x' | 'X' => i64::from_str_radix(num, 16),
        'o' | 'O' => i64::from_str_radix(num, 8),
        'b' | 'B' => i64::from_str_radix(num, 2),
        _ => unreachable!(),
    };
    match r {
        Ok(a) => AST::Integer(a),
        Err(e) => {
            println!(
                "ParseError: {} is {:?} at {:?}",
                s,
                e.kind(),
                pair.as_span().start_pos()
            );
            panic!(e)
        }
    }
}

fn parse_float(pair: Pair<Rule>) -> AST {
    let s = pair.as_str();
    match f64::from_str(s) {
        Ok(a) => AST::Decimal(a),
        Err(e) => {
            println!(
                "ParseError: {} is {:?} at {:?}",
                s,
                e,
                pair.as_span().start_pos()
            );
            panic!(e)
        }
    }
}

fn parse_string(pair: Pair<Rule>) -> AST {
    let i = pair.as_str();
    let s = &i[1..i.len() - 1];
    AST::String(s.to_string())
}

fn parse_symbol(pair: Pair<Rule>) -> AST {
    AST::Symbol(pair.to_string())
}
