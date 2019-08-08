#[allow(unused_imports)]
use crate::{Parser, Rule, AST};
#[cfg(feature = "colored")]
use colored::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser as Pest;

#[cfg(feature = "colored")]
pub fn token_print(s: &str, rule: Rule) {
    let pairs = Parser::parse(rule, s).unwrap_or_else(|e| panic!("{}", e));
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        match pair.as_rule() {
            Rule::NEWLINE | Rule::EOI => {
                continue;
            }
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
