use std::str::FromStr;

use crate::ast;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PewterParser;

fn parse_def(def: Pair<Rule>) -> ast::Def {
    let mut pairs = def.into_inner();
    let id = pairs.next().unwrap().as_str().to_owned();
    let lit = pairs.next().unwrap().into_inner().next().unwrap();
    let radix = match lit.as_rule() {
        Rule::dec_literal => 10,
        Rule::hex_literal => 16,
        Rule::oct_literal => 8,
        Rule::bin_literal => 2,
        _ => unreachable!(),
    };
    let value = i32::from_str_radix(&lit.as_str(), radix).unwrap();
    ast::Def(id, value)
}

impl FromStr for ast::Compunit {
    type Err = pest::error::Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = PewterParser::parse(Rule::compunit, s)?;
        let module = pairs.next().unwrap();
        let mut defs = vec![];
        for pair in module.into_inner() {
            match pair.as_rule() {
                Rule::EOI => {}
                Rule::def => defs.push(parse_def(pair)),
                _ => unreachable!(),
            }
        }
        Ok(Self {
            name: "test".to_owned(),
            defs,
        })
    }
}
