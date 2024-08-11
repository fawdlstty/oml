use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

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
}

impl OmlValue {
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
}

#[derive(Debug, Clone)]
pub enum OmlExpr {
    None,
    Value(OmlValue),
    TempName(String),
    Op2((Box<OmlExpr>, String, Box<OmlExpr>)),
    Op3((Box<OmlExpr>, Box<OmlExpr>, Box<OmlExpr>)),
}

impl OmlExpr {
    pub fn new() -> Self {
        Self::None
    }

    pub fn from_str(content: &str) -> Result<OmlExpr, String> {
        match OmlParser::parse(Rule::oml, content) {
            Ok(mut root) => Self::parse_oml(root.next().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn eval(&self, mut path: Vec<String>) -> OmlValue {
        match self {
            OmlExpr::None => OmlValue::None,
            OmlExpr::Value(val) => val.clone(),
            OmlExpr::TempName(name) => {
                let names: Vec<_> = name.split('.').map(|s| s.to_string()).collect();
                for name in names {
                    if name == "super" {
                        //
                    }
                }
            }
            OmlExpr::Op2(_) => todo!(),
            OmlExpr::Op3(_) => todo!(),
        }
    }

    pub fn apply(&mut self, val: OmlExpr) {
        // match self {
        //     OmlValue::Array(arr) => arr.push(val),
        //     OmlValue::Map(map) => {
        //         if let OmlValue::Map(map2) = val {
        //             for (key, val) in map2.into_iter() {
        //                 if let Some(self_k) = map.get_mut(&key) {
        //                     self_k.apply(val);
        //                 } else {
        //                     map.insert(key, val);
        //                 }
        //             }
        //         } else {
        //             *self = val;
        //         }
        //     }
        //     _ => *self = val,
        // }
    }

    fn parse_oml(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlExpr, String> {
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

    fn parse_block(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlExpr, String> {
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

    fn parse_pair(root: pest::iterators::Pair<'_, Rule>) -> (String, OmlExpr) {
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

    fn parse_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
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

    fn parse_base_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::literal => Self::parse_literal(root_item),
            Rule::ids => OmlValue::TempName(Self::parse_ids(root_item)),
            Rule::expr => Self::parse_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_array_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        OmlValue::Array(exprs)
    }

    fn parse_map_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
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

    fn parse_op2_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut expr1 = OmlValue::new_map();
        let mut op = "".to_string();
        let mut expr2 = OmlValue::new_map();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::base_expr => expr1 = Self::parse_base_expr(root_item),
                _ => unreachable!(),
            }
        }
    }

    fn parse_op3_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        //
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        //
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
