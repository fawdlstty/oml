use super::eval::Op2Evaluator;
use super::oml_value::OmlValue;
use crate::string_utils::IntoBaseExt;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Parser)]
#[grammar = "../oml.pest"]
pub struct OmlParser;

#[derive(Debug, Clone)]
pub struct OmlExpr {
    pub enable_if: Vec<OmlExprImpl>,
    pub value: OmlExprImpl,
}

#[derive(Debug, Clone)]
pub enum OmlExprImpl {
    None,
    Value(OmlValue),
    Array(Vec<OmlExpr>),
    Map(HashMap<String, OmlExpr>),
    TempName(String),
    Op2((Box<OmlExpr>, String, Box<OmlExpr>)),
    Op3((Box<OmlExpr>, Box<OmlExpr>, Box<OmlExpr>)),
    FormatString((Vec<String>, Vec<OmlExpr>)),
}

impl OmlExpr {
    pub fn new() -> Self {
        Self {
            enable_if: vec![],
            value: OmlExprImpl::None,
        }
    }

    pub fn make(enable_if: Vec<OmlExprImpl>, value: OmlExprImpl) -> Self {
        Self { enable_if, value }
    }

    pub fn from_str(content: &str) -> Result<OmlExpr, String> {
        match OmlParser::parse(Rule::oml, content) {
            Ok(mut root) => Self::parse_oml(root.next().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn apply(&mut self, val: OmlExpr) {
        match &mut self.value {
            OmlExprImpl::Array(arr) => arr.push(val),
            OmlExprImpl::Map(map) => {
                if let OmlExprImpl::Map(map2) = val.value {
                    for (key, mut val) in map2.into_iter() {
                        // TODO process enable_if with not same
                        //val.enable_if.extend(val.enable_if.clone());
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

    fn parse_oml(root: pest::iterators::Pair<'_, Rule>) -> Result<OmlExpr, String> {
        let mut ret = Self::new();
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
                        value = OmlExpr::make(vec![], OmlExprImpl::Map(tmp_map));
                    }
                    ret.insert(keys.remove(0), value);
                }
                _ => unreachable!(),
            }
        }
        let mut ret = OmlExprImpl::Map(ret);
        if is_array_head {
            ret = OmlExprImpl::Array(vec![OmlExpr::make(vec![], ret)]);
        }
        let mut keys: Vec<_> = head.split('.').map(|key| key.to_string()).collect();
        while !keys.is_empty() {
            let name = keys.remove(keys.len() - 1);
            ret = OmlExprImpl::Map(
                vec![(name, OmlExpr::make(vec![], ret))]
                    .into_iter()
                    .collect(),
            );
        }
        Ok(OmlExpr::make(vec![], ret))
    }

    fn parse_pair(root: pest::iterators::Pair<'_, Rule>) -> (String, OmlExpr) {
        let mut keys = "".to_string();
        let mut value = OmlExpr::new();
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
            Rule::ids => OmlExpr::make(vec![], OmlExprImpl::TempName(Self::parse_ids(root_item))),
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
        OmlExpr::make(vec![], OmlExprImpl::Array(exprs))
    }

    fn parse_map_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut map = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::map_assign_pair => {
                    let (key, value) = Self::parse_pair(root_item);
                    map.insert(key, value);
                }
                _ => unreachable!(),
            }
        }
        OmlExpr::make(vec![], OmlExprImpl::Map(map))
    }

    fn parse_op2_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut expr1 = OmlExpr::new();
        let mut op = "".to_string();
        let mut expr2 = OmlExpr::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::base_expr => expr1 = Self::parse_base_expr(root_item),
                Rule::op2 => op = root_item.as_str().to_string(),
                Rule::expr => expr2 = Self::parse_expr(root_item),
                _ => unreachable!(),
            }
        }
        OmlExpr::make(
            vec![],
            OmlExprImpl::Op2((Box::new(expr1), op, Box::new(expr2))),
        )
    }

    fn parse_op3_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut expr1 = OmlExpr::new();
        let mut expr2 = OmlExpr::new();
        let mut expr3 = OmlExpr::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::base_expr => expr1 = Self::parse_base_expr(root_item),
                Rule::expr => expr2 = Self::parse_expr(root_item),
                Rule::expr1 => expr3 = Self::parse_expr(root_item.into_inner().next().unwrap()),
                _ => unreachable!(),
            }
        }
        OmlExpr::make(
            vec![],
            OmlExprImpl::Op3((Box::new(expr1), Box::new(expr2), Box::new(expr3))),
        )
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let root_item = root.into_inner().next().unwrap();
        OmlExpr::make(
            vec![],
            OmlExprImpl::Value(match root_item.as_rule() {
                Rule::boolean_literal => OmlValue::Bool(root_item.as_str() == "true"),
                Rule::number_literal => OmlValue::Int64(root_item.as_str().parse().unwrap_or(0)),
                Rule::string_literal => OmlValue::String(root_item.as_str().into_base()),
                Rule::format_string_literal => return Self::parse_format_string_literal(root_item),
                _ => unreachable!(),
            }),
        )
    }

    fn parse_format_string_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut strs = vec![];
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::format_string => {
                    return OmlExpr::make(
                        vec![],
                        OmlExprImpl::Value(OmlValue::String(root_item.as_str().into_base())),
                    )
                }
                Rule::format_string_part1 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part2 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part3 => strs.push(root_item.as_str().into_base()),
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        OmlExpr::make(vec![], OmlExprImpl::FormatString((strs, exprs)))
    }

    fn parse_ids(root: pest::iterators::Pair<'_, Rule>) -> String {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::ids => root_item.as_str().to_string(),
            Rule::id => root_item.as_str().to_string(),
            _ => unreachable!(),
        }
    }

    pub fn evalute(&self) -> Result<OmlValue, String> {
        let mut last_result = OmlValue::None;
        let mut count = 3;
        while count >= 0 {
            count -= 1;
            match self.evalute2("", &last_result) {
                Ok((result, success)) => {
                    if success {
                        return Ok(result);
                    }
                    last_result = result;
                }
                Err(err) => return Err(err),
            }
        }
        Err("evalute failed.".to_string())
    }

    fn evalute2(&self, path: &str, last_result: &OmlValue) -> Result<(OmlValue, bool), String> {
        let mut success = true;
        let value = match &self.value {
            OmlExprImpl::None => OmlValue::None,
            OmlExprImpl::Value(val) => val.clone(),
            OmlExprImpl::Array(arr) => {
                let mut ret = vec![];
                for (index, item) in arr.iter().enumerate() {
                    let (val, tmp_success) = item.evalute2(&path.append_num(index), last_result)?;
                    ret.push(val);
                    success &= tmp_success;
                }
                OmlValue::Array(ret)
            }
            OmlExprImpl::Map(map) => {
                let mut ret = HashMap::new();
                for (key, value) in map.iter() {
                    let (val, tmp_success) = value.evalute2(&path.append_str(key), last_result)?;
                    ret.insert(key.clone(), val);
                    success &= tmp_success;
                }
                OmlValue::Map(ret)
            }
            OmlExprImpl::TempName(name) => {
                match last_result.get(&path.remove_once().append_str(name)) {
                    Some(val) => val.clone(),
                    None => {
                        success = false;
                        OmlValue::None
                    }
                }
            }
            OmlExprImpl::Op2((left, op, right)) => {
                let (left, tmp_success1) = left.evalute2(path, last_result)?;
                success &= tmp_success1;
                let (right, tmp_success2) = right.evalute2(path, last_result)?;
                success &= tmp_success2;
                if tmp_success1 && tmp_success2 {
                    Op2Evaluator::eval(left, op, right)?
                } else {
                    OmlValue::None
                }
            }
            OmlExprImpl::Op3((cond, left, right)) => {
                let (cond, tmp_success) = cond.evalute2(path, last_result)?;
                if tmp_success {
                    let (value, tmp_success) = match cond.as_bool() {
                        Some(true) => left.evalute2(path, last_result)?,
                        Some(false) => right.evalute2(path, last_result)?,
                        None => return Err("condition must be boolean.".to_string()),
                    };
                    success &= tmp_success;
                    value
                } else {
                    OmlValue::None
                }
            }
            OmlExprImpl::FormatString((strs, exprs)) => {
                let mut exprs1 = vec![];
                let mut tmp_success = true;
                for item in exprs.iter() {
                    let (val, tmp_success1) = item.evalute2(path, last_result)?;
                    exprs1.push(val);
                    tmp_success &= tmp_success1;
                }
                if tmp_success {
                    exprs1.push(OmlValue::String("".to_string()));
                    let mut ret = "".to_string();
                    for (a, b) in strs.iter().zip(exprs1.iter()) {
                        ret.push_str(a);
                        ret.push_str(&b.as_str());
                    }
                    OmlValue::String(ret)
                } else {
                    success = false;
                    OmlValue::None
                }
            }
        };
        Ok((value, success))
    }
}

pub(crate) trait PathAppendExt {
    fn append_str(&self, path: &str) -> String;
    fn append_num(&self, num: usize) -> String;
    fn remove_once(&self) -> &str;
}

impl PathAppendExt for str {
    fn append_str(&self, name: &str) -> String {
        match self.is_empty() {
            true => name.to_string(),
            false => format!("{}.{}", self, name),
        }
    }

    fn append_num(&self, num: usize) -> String {
        match self.is_empty() {
            true => num.to_string(),
            false => format!("{}.{}", self, num),
        }
    }

    fn remove_once(&self) -> &str {
        match self.rfind('.') {
            Some(pos) => &self[0..pos],
            None => "",
        }
    }
}

impl Index<usize> for OmlExpr {
    type Output = OmlExpr;
    fn index(&self, index: usize) -> &Self::Output {
        match &self.value {
            OmlExprImpl::Array(arr) => arr.get(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for OmlExpr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match &mut self.value {
            OmlExprImpl::Array(arr) => arr.get_mut(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl Index<&str> for OmlExpr {
    type Output = OmlExpr;
    fn index(&self, index: &str) -> &Self::Output {
        match &self.value {
            OmlExprImpl::Map(map) => map.get(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl IndexMut<&str> for OmlExpr {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match &mut self.value {
            OmlExprImpl::Map(map) => map.get_mut(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl OmlExpr {
    pub fn get_at(&self, index: usize) -> Option<&Self> {
        if let OmlExprImpl::Array(arr) = &self.value {
            arr.get(index)
        } else {
            None
        }
    }

    pub fn get_at_mut(&mut self, index: usize) -> Option<&mut Self> {
        if let OmlExprImpl::Array(arr) = &mut self.value {
            arr.get_mut(index)
        } else {
            None
        }
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        if let OmlExprImpl::Map(map) = &self.value {
            map.get(index)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: &str) -> Option<&mut Self> {
        if let OmlExprImpl::Map(map) = &mut self.value {
            map.get_mut(index)
        } else {
            None
        }
    }
}
