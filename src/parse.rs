use std::str::FromStr;

use crate::ast;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PewterParser;

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

fn parse_def(def: Pair<Rule>) -> ast::Def {
    let mut pairs = def.into_inner();
    let id = pairs.next().unwrap().as_str().to_owned();
    let lit = parse_literal(pairs.next().unwrap());
    ast::Def(id, lit)
}

fn parse_literal(lit: Pair<Rule>) -> ast::Literal {
    let inner = lit.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::int_literal => parse_int_literal(inner),
        _ => unreachable!(),
    }
}

fn parse_int_literal(lit: Pair<Rule>) -> ast::Literal {
    let inner = lit.into_inner().next().unwrap();
    let radix = match inner.as_rule() {
        Rule::dec_literal => 10,
        Rule::hex_literal => 16,
        Rule::oct_literal => 8,
        Rule::bin_literal => 2,
        _ => unreachable!(),
    };
    let to_parse = inner.as_str().replace("_", "");
    let value = i32::from_str_radix(&to_parse, radix).unwrap();
    ast::Literal::Int(value)
}
