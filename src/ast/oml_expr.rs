use super::eval::{Op1Evaluator, Op2Evaluator};
use super::oml_value::OmlValue;
use crate::string_utils::IntoBaseExt;
use pest::Parser;
use pest_derive::Parser;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::sync::OnceLock;

fn get_op2_level(op: &str) -> usize {
    static OP2_LEVELS: OnceLock<HashMap<&'static str, usize>> = OnceLock::new();
    *OP2_LEVELS
        .get_or_init(|| {
            [
                ("**", 0),
                ("*", 1),
                ("/", 1),
                ("%", 1),
                ("+", 2),
                ("-", 2),
                ("<<", 3),
                (">>", 3),
                ("^", 4),
                ("|", 4),
                ("&", 4),
                ("<", 5),
                ("<=", 5),
                (">", 5),
                (">=", 5),
                ("==", 6),
                ("!=", 6),
                ("&&", 7),
                ("||", 8),
            ]
            .into_iter()
            .collect()
        })
        .get(op)
        .unwrap_or(&9)
}

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
    Op1Prefix((String, Box<OmlExpr>)),
    Op1Suffix((Box<OmlExpr>, String)),
    Op2((Box<OmlExpr>, String, Box<OmlExpr>)),
    Op3((Box<OmlExpr>, Box<OmlExpr>, Box<OmlExpr>)),
    FormatString((Vec<String>, Vec<OmlExpr>)),
    AccessVar((Box<OmlExpr>, String)),
    InvokeFunc((Box<OmlExpr>, String, Vec<OmlExpr>)),
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
            Rule::weak_expr => Self::parse_weak_expr(root_item),
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

    fn parse_strong_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let root_item = root.into_inner().next().unwrap();
        match root_item.as_rule() {
            Rule::base_expr => Self::parse_base_expr(root_item),
            Rule::array_expr => Self::parse_array_expr(root_item),
            Rule::map_expr => Self::parse_map_expr(root_item),
            _ => unreachable!(),
        }
    }

    fn parse_middle_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        enum SuffixOp {
            AccessVar(String),
            InvokeFunc((String, Vec<OmlExpr>)),
            Op(String),
        }
        impl SuffixOp {
            pub fn parse(root: pest::iterators::Pair<'_, Rule>) -> Self {
                let root_str = root.as_str();
                let mut id = "".to_string();
                let mut args = None;
                for root_item in root.into_inner() {
                    match root_item.as_rule() {
                        Rule::id => id = root_item.as_str().to_string(),
                        Rule::_exprs => {
                            let mut exprs = vec![];
                            for root_item1 in root_item.into_inner() {
                                match root_item1.as_rule() {
                                    Rule::expr => exprs.push(OmlExpr::parse_expr(root_item1)),
                                    _ => unreachable!(),
                                }
                            }
                            args = Some(exprs)
                        }
                        _ => unreachable!(),
                    }
                }
                if id.is_empty() {
                    SuffixOp::Op(root_str.to_string())
                } else if let Some(args) = args {
                    SuffixOp::InvokeFunc((id, args))
                } else {
                    SuffixOp::AccessVar(id)
                }
            }
        }

        let mut expr = OmlExpr::new();
        let mut prefix_ops = vec![];
        let mut suffix_ops = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::strong_expr => expr = Self::parse_strong_expr(root_item),
                Rule::expr_prefix => prefix_ops.push(root_item.as_str().to_string()),
                Rule::expr_suffix => suffix_ops.push(SuffixOp::parse(root_item)),
                _ => unreachable!(),
            }
        }
        while !prefix_ops.is_empty() {
            let prefix_op = prefix_ops.remove(prefix_ops.len());
            expr = OmlExpr::make(vec![], OmlExprImpl::Op1Prefix((prefix_op, Box::new(expr))));
        }
        while !suffix_ops.is_empty() {
            expr = OmlExpr::make(
                vec![],
                match suffix_ops.remove(0) {
                    SuffixOp::AccessVar(name) => OmlExprImpl::AccessVar((Box::new(expr), name)),
                    SuffixOp::InvokeFunc((name, args)) => {
                        OmlExprImpl::InvokeFunc((Box::new(expr), name, args))
                    }
                    SuffixOp::Op(suffix_op) => OmlExprImpl::Op1Suffix((Box::new(expr), suffix_op)),
                },
            )
        }
        expr
    }

    fn parse_weak_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut exprs = vec![];
        let mut ops = vec![];
        //
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::middle_expr => exprs.push(Self::parse_middle_expr(root_item)),
                Rule::op2 => ops.push(root_item.as_str().to_string()),
                _ => unreachable!(),
            }
        }
        let mut ops: Vec<_> = ops
            .into_iter()
            .map(|op| {
                let level = get_op2_level(&op[..]);
                (op, level)
            })
            .collect();
        //
        for i in 0..10 {
            if exprs.len() == 1 {
                break;
            }
            if i == 5 {
                for j in 1..ops.len() {
                    if ops[j - i].1 == i && ops[j].1 == i {
                        exprs.insert(j, exprs[j].clone());
                        ops.insert(j, ("&&".to_string(), get_op2_level("&&")));
                    }
                }
            }
            for idx in 0..ops.len() {
                if let Some((_, level)) = ops.get(idx) {
                    if *level != i {
                        continue;
                    }
                }
                let left = exprs.remove(idx);
                let right = exprs.remove(idx);
                let op = ops.remove(idx).0;
                let expr = OmlExpr::make(
                    vec![],
                    OmlExprImpl::Op2((Box::new(left), op, Box::new(right))),
                );
                exprs.insert(idx, expr);
            }
        }
        exprs.remove(0)
    }

    fn parse_op3_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::middle_expr => exprs.push(Self::parse_middle_expr(root_item)),
                _ => unreachable!(),
            }
        }
        let expr1 = Box::new(exprs.remove(0));
        let expr2 = Box::new(exprs.remove(0));
        let expr3 = Box::new(exprs.remove(0));
        OmlExpr::make(vec![], OmlExprImpl::Op3((expr1, expr2, expr3)))
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let root_item = root.into_inner().next().unwrap();
        OmlExpr::make(
            vec![],
            OmlExprImpl::Value(match root_item.as_rule() {
                Rule::boolean_literal => OmlValue::Bool(root_item.as_str() == "true"),
                Rule::number_literal => match root_item.as_str().parse::<i64>() {
                    Ok(n) => OmlValue::Int64(n),
                    Err(_) => OmlValue::String(root_item.as_str().into_base()),
                },
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
            OmlExprImpl::Op1Prefix((name, expr)) => {
                let (val, tmp_success) = expr.evalute2(path, last_result)?;
                success &= tmp_success;
                if tmp_success {
                    Op1Evaluator::eval_prefix(name, val)?
                } else {
                    OmlValue::None
                }
            }
            OmlExprImpl::Op1Suffix((expr, name)) => {
                let (val, tmp_success) = expr.evalute2(path, last_result)?;
                success &= tmp_success;
                if tmp_success {
                    Op1Evaluator::eval_suffix(name, val)?
                } else {
                    OmlValue::None
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
            OmlExprImpl::AccessVar(_) => todo!(),
            OmlExprImpl::InvokeFunc(_) => todo!(),
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
        static NULL_EXPR: OmlExpr = OmlExpr {
            enable_if: Vec::new(),
            value: OmlExprImpl::Value(OmlValue::None),
        };
        if index == "" {
            return self;
        } else if let Some(p) = index.find('.') {
            let (a, b) = index.split_at(p);
            self.index(a).index(&b[1..])
        } else {
            match &self.value {
                OmlExprImpl::Map(map) => map.get(index).unwrap_or(&NULL_EXPR),
                _ => &NULL_EXPR,
            }
        }
    }
}

impl IndexMut<&str> for OmlExpr {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        // static NULL_EXPR: OmlExpr = OmlExpr {
        //     enable_if: Vec::new(),
        //     value: OmlExprImpl::Value(OmlValue::None),
        // };
        // let null_expr = unsafe {
        //     std::mem::transmute::<*const OmlExpr, *mut OmlExpr>(&NULL_EXPR as *const OmlExpr)
        // };
        // let null_expr = unsafe { &mut *null_expr };
        // if index == "" {
        //     return self;
        // } else if let Some(p) = index.find('.') {
        //     let (a, b) = index.split_at(p);
        //     self.index_mut(a).index_mut(&b[1..])
        // } else {
        //     match &mut self.value {
        //         OmlExprImpl::Map(map) => map.get_mut(index).unwrap_or(null_expr),
        //         _ => null_expr,
        //     }
        // }
        if index == "" {
            return self;
        } else if let Some(p) = index.find('.') {
            let (a, b) = index.split_at(p);
            self.index_mut(a).index_mut(&b[1..])
        } else {
            match &mut self.value {
                OmlExprImpl::Map(map) => map.get_mut(index).unwrap(),
                _ => panic!(),
            }
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

    pub fn get_with_path(&mut self, path: &str) -> Option<&mut Self> {
        let path_items: Vec<_> = path.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                let num: usize = num.parse().unwrap();
                if let Some(obj) = obj_ref.get_at_mut(num) {
                    obj_ref = obj;
                } else {
                    return None;
                }
            }
            if let Some(obj) = obj_ref.get_mut(path_item) {
                obj_ref = obj;
            } else {
                return None;
            }
        }
        Some(obj_ref)
    }
}
