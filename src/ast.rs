use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

use crate::string_utils::IntoBaseExt;

#[derive(Parser)]
#[grammar = "../oml.pest"]
pub struct OmlParser;

#[derive(Debug, Clone)]
pub enum OmlValue {
    None,
    Bool(bool),
    Int64(i64),
    Float64(f64),
    String(String),
    Array(Vec<OmlValue>),
    Map(HashMap<String, OmlValue>),
    TempName(String),
    Op2((Box<OmlValue>, String, Box<OmlValue>)),
    Op3((Box<OmlValue>, Box<OmlValue>, Box<OmlValue>)),
    FormatString((Vec<String>, Vec<OmlValue>)),
}

impl OmlValue {
    pub fn new() -> Self {
        Self::None
    }

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
        match self {
            OmlValue::Array(arr) => arr.push(val),
            OmlValue::Map(map) => {
                if let OmlValue::Map(map2) = val {
                    for (key, val) in map2.into_iter() {
                        if let Some(self_k) = map.get_mut(&key) {
                            self_k.apply(val);
                        } else {
                            map.insert(key, val);
                        }
                    }
                } else {
                    *self = val;
                }
            }
            _ => *self = val,
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

    fn parse_block(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlValue, String> {
        let mut head = "".to_string();
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
                    let (key, mut value) = Self::parse_pair(root_item);
                    let mut keys: Vec<_> = key.split('.').map(|key| key.to_string()).collect();
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
        let mut ret = OmlValue::Map(ret);
        if is_array_head {
            ret = OmlValue::Array(vec![ret]);
        }
        let mut keys: Vec<_> = head.split('.').map(|key| key.to_string()).collect();
        while !keys.is_empty() {
            let name = keys.remove(keys.len() - 1);
            ret = OmlValue::Map(vec![(name, ret)].into_iter().collect());
        }
        Ok(ret)
    }

    fn parse_pair(root: pest::iterators::Pair<'_, Rule>) -> (String, OmlValue) {
        let mut keys = "".to_string();
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
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::base_expr => Self::parse_base_expr(root_item),
            Rule::array_expr => Self::parse_array_expr(root_item),
            Rule::map_expr => Self::parse_map_expr(root_item),
            Rule::op2_expr => Self::parse_op2_expr(root_item),
            Rule::op3_expr => Self::parse_op3_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_base_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::literal => Self::parse_literal(root_item),
            Rule::ids => OmlValue::TempName(Self::parse_ids(root_item)),
            Rule::expr => Self::parse_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_array_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        OmlValue::Array(exprs)
    }

    fn parse_map_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let mut map = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::map_assign_pair => {
                    let (key, mut value) = Self::parse_pair(root_item);
                    map.insert(key, value);
                }
                _ => unreachable!(),
            }
        }
        OmlValue::Map(map)
    }

    fn parse_op2_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let mut expr1 = OmlValue::new_map();
        let mut op = "".to_string();
        let mut expr2 = OmlValue::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::base_expr => expr1 = Self::parse_base_expr(root_item),
                Rule::op2 => op = root_item.as_str().to_string(),
                Rule::expr => expr2 = Self::parse_expr(root_item),
                _ => unreachable!(),
            }
        }
        OmlValue::Op2((Box::new(expr1), op, Box::new(expr2)))
    }

    fn parse_op3_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let mut expr1 = OmlValue::new_map();
        let mut expr2 = OmlValue::new_map();
        let mut expr3 = OmlValue::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::base_expr => expr1 = Self::parse_base_expr(root_item),
                Rule::expr => expr2 = Self::parse_expr(root_item),
                Rule::expr1 => expr3 = Self::parse_expr(root_item.into_inner().next().unwrap()),
                _ => unreachable!(),
            }
        }
        OmlValue::Op3((Box::new(expr1), Box::new(expr2), Box::new(expr3)))
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::boolean_literal => OmlValue::Bool(root_item.as_str() == "true"),
            Rule::number_literal => OmlValue::Int64(root_item.as_str().parse().unwrap_or(0)),
            Rule::string_literal => OmlValue::String(root_item.as_str().into_base()),
            Rule::format_string_literal => Self::parse_format_string_literal(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_format_string_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlValue {
        let mut strs = vec![];
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::format_string => return OmlValue::String(root_item.as_str().into_base()),
                Rule::format_string_part1 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part2 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part3 => strs.push(root_item.as_str().into_base()),
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        OmlValue::FormatString((strs, exprs))
    }

    fn parse_ids(root: pest::iterators::Pair<'_, Rule>) -> String {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::ids => root_item.as_str().to_string(),
            Rule::id => root_item.as_str().to_string(),
            _ => unreachable!(),
        }
    }
}
