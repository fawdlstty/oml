use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "../oml.pest"]
pub struct OmlParser;

#[derive(Debug)]
pub enum OmlValue {
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<OmlValue>),
    Map(HashMap<String, OmlValue>),
}

impl OmlValue {
    pub fn new_map() -> Self {
        Self::Map(HashMap::new())
    }

    pub fn from_str(content: &str) -> Result<OmlValue, String> {
        match OmlParser::parse(Rule::oml, content) {
            Ok(mut root) => Self::parse_oml(root.next().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn as_key(&self) -> Result<Vec<String>, String> {
        if let Self::String(s) = self {
            // TODO
            panic!();
            Ok(vec![])
        } else {
            Err("".to_string())
        }
    }

    pub fn apply(&mut self, val: OmlValue) {
        // TODO
    }

    fn parse_oml(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        let mut ret = Self::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::group_block => {
                    let val = Self::parse_block(root_item)?;
                    ret.apply(val);
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        Ok(ret)
    }

    fn parse_block(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        let mut head = vec![];
        let mut is_array_head = false;
        let mut ret = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::group_head => head = Self::parse_ids(root_item),
                Rule::group_array_head => {
                    head = Self::parse_ids(root_item);
                    is_array_head = true;
                }
                Rule::assign_pair => (), //TODO
                _ => unreachable!(),
            }
        }
        Ok(Self::Map(ret))
    }

    fn parse_pair(root: pest::iterators::Pair<'_, Rule>) -> Result<(String, OmlValue), String> {
        let mut key = vec![];
        let mut value = OmlValue::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::string_literal => key = Self::parse_literal(root_item)?.as_key()?,
                Rule::ids => key = Self::parse_ids(root_item),
                Rule::expr => value = Self::parse_expr(root_item)?,
            }
        }
        Ok((key, value))
    }

    fn parse_expr(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        //
    }

    fn parse_ids(root: pest::iterators::Pair<'_, Rule>) -> Vec<String> {
        let mut ret = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::ids => return Self::parse_ids(root_item.into_inner().next().unwrap()),
                Rule::id => ret.push(root_item.as_str().to_string()),
                _ => unreachable!(),
            }
        }
        ret
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        //
    }
}
