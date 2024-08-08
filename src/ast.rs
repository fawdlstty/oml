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

    pub fn apply(&mut self, val: OmlValue) {
        if let OmlValue::Map(self_m) = self {
            if let OmlValue::Map(m) = val {
                for (key, value) in m {
                    self_m.insert(key, value);
                }
            }
        }
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

    fn parse_block(
        root: pest::iterators::Pair<'_, Rule>,
    ) -> Result<(Vec<String>, bool, OmlValue), String> {
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
                Rule::assign_pair => {
                    let (mut keys, mut value) = Self::parse_pair(root_item);
                    while keys.len() > 1 {
                        let mut tmp_map = HashMap::new();
                        tmp_map.insert(keys.remove(keys.len() - 1), value);
                        value = OmlValue::Map(tmp_map);
                    }
                    ret.insert(keys.remove(0), value);
                }
                _ => unreachable!(),
            }
        }
        Ok((head, is_array_head, OmlValue::Map(ret)))
    }

    fn parse_pair(root: pest::iterators::Pair<'_, Rule>) -> (Vec<String>, OmlValue) {
        let mut keys = vec![];
        let mut value = OmlValue::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::ids => keys = Self::parse_ids(root_item),
                Rule::expr => value = Self::parse_expr(root_item),
                _ => unreachable!(),
            }
        }
        (keys, value)
    }

    fn parse_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        todo!()
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
        todo!()
    }
}
