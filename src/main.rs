extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct NemetParser;

fn main() {
    let buf = fs::read("./test.nmt").unwrap();
    let source = String::from_utf8(buf).unwrap();
    let pairs = NemetParser::parse(Rule::program_file, source.as_str())
        .unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("{:?}",pair.as_rule());
    }
}

