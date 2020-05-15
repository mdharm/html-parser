use anyhow::Result;
use pest::{iterators::Pair, iterators::Pairs, Parser};
use pest_derive::Parser;

use crate::error::Error;

mod fmt;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "parser/html.pest"]
pub struct PestRules;

pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse(input: &str, debug: bool) -> Result<()> {
        let mut pairs = match PestRules::parse(Rule::root, input) {
            Ok(pairs) => pairs,
            Err(error) => return fmt::error_msg(error),
        };
        if debug {
            dbg!(pairs);
        }
        Ok(())
    }
}
