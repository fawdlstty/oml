use super::eval::{Op1Evaluator, Op2Evaluator};
use super::oml_value::OmlValue;
use crate::string_utils::IntoBaseExt;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::sync::OnceLock;

static NULL_EXPR: OmlExpr = OmlExpr::None;

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
pub enum OmlExpr {
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
    IfAnno(OmlExprIfAnno),
}

#[derive(Debug, Clone)]
pub struct OmlExprIfAnno {
    pub exprs: Vec<(OmlExpr, OmlExpr)>,
    pub default: Option<Box<OmlExpr>>,
}

impl OmlExpr {
    pub fn new() -> Self {
        OmlExpr::None
    }

    pub fn make_if_anno(if_anno: OmlExpr, value: OmlExpr) -> Self {
        Self::IfAnno(OmlExprIfAnno {
            exprs: vec![(if_anno, value)],
            default: None,
        })
    }

    pub fn from_str(content: &str) -> Result<OmlExpr, String> {
        match OmlParser::parse(Rule::oml, content) {
            Ok(mut root) => Self::parse_oml(root.next().unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn apply(&mut self, val: OmlExpr) {
        match self {
            OmlExpr::None => *self = val,
            OmlExpr::Array(arr) => {
                if let OmlExpr::Array(arr2) = val {
                    arr.extend(arr2);
                }
            }
            OmlExpr::Map(map) => {
                if let OmlExpr::Map(map2) = val {
                    for (key, mut val) in map2.into_iter() {
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
            OmlExpr::IfAnno(if_anno) => {
                if let OmlExpr::IfAnno(if_anno2) = val {
                    if_anno.exprs.extend(if_anno2.exprs);
                } else if if_anno.default.is_none() {
                    if_anno.default = Some(Box::new(val));
                }
            }
            _ => {}
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
        let mut anno_if_expr = None;
        let mut head = "".to_string();
        let mut is_array_head = false;
        let mut ret = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::anno_if => {
                    anno_if_expr = Some(Self::parse_expr(root_item.into_inner().next().unwrap()))
                }
                Rule::group_head => head = Self::parse_ids(root_item),
                Rule::group_array_head => {
                    head = Self::parse_ids(root_item);
                    is_array_head = true;
                }
                Rule::assign_pair => {
                    let (key, mut value) = Self::parse_assign_pair(root_item);
                    let mut keys: Vec<_> = key.split('.').map(|key| key.to_string()).collect();
                    while keys.len() > 1 {
                        let mut tmp_map = HashMap::new();
                        tmp_map
                            .entry(keys.remove(keys.len() - 1))
                            .or_insert(OmlExpr::None)
                            .apply(value);
                        value = OmlExpr::Map(tmp_map);
                    }
                    ret.entry(keys.remove(0))
                        .or_insert(OmlExpr::None)
                        .apply(value);
                }
                _ => unreachable!(),
            }
        }
        let mut ret = OmlExpr::Map(ret);
        if is_array_head {
            ret = OmlExpr::Array(vec![ret]);
        }
        let mut keys: Vec<_> = head.split('.').map(|key| key.to_string()).collect();
        while !keys.is_empty() {
            let name = keys.remove(keys.len() - 1);
            ret = OmlExpr::Map(vec![(name, ret)].into_iter().collect());
        }
        if let Some(anno_if_expr) = anno_if_expr {
            ret = OmlExpr::IfAnno(OmlExprIfAnno {
                exprs: vec![(anno_if_expr, ret)],
                default: None,
            })
        }
        Ok(ret)
    }

    fn parse_assign_pair(root: pest::iterators::Pair<'_, Rule>) -> (String, OmlExpr) {
        let mut anno_if_expr = None;
        let mut keys = "".to_string();
        let mut value = OmlExpr::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::anno_if => {
                    anno_if_expr = Some(Self::parse_expr(root_item.into_inner().next().unwrap()))
                }
                Rule::ids => keys = Self::parse_ids(root_item),
                Rule::expr => value = Self::parse_expr(root_item),
                _ => unreachable!(),
            }
        }
        if let Some(anno_if_expr) = anno_if_expr {
            value = OmlExpr::IfAnno(OmlExprIfAnno {
                exprs: vec![(anno_if_expr, value)],
                default: None,
            })
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
            Rule::ids => OmlExpr::TempName(Self::parse_ids(root_item)),
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
        OmlExpr::Array(exprs)
    }

    fn parse_map_expr(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut map = HashMap::new();
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::map_assign_pair => {
                    let (key, value) = Self::parse_assign_pair(root_item);
                    map.insert(key, value);
                }
                _ => unreachable!(),
            }
        }
        OmlExpr::Map(map)
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
            expr = OmlExpr::Op1Prefix((prefix_op, Box::new(expr)));
        }
        while !suffix_ops.is_empty() {
            expr = match suffix_ops.remove(0) {
                SuffixOp::AccessVar(name) => OmlExpr::AccessVar((Box::new(expr), name)),
                SuffixOp::InvokeFunc((name, args)) => {
                    OmlExpr::InvokeFunc((Box::new(expr), name, args))
                }
                SuffixOp::Op(suffix_op) => OmlExpr::Op1Suffix((Box::new(expr), suffix_op)),
            };
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
                let expr = OmlExpr::Op2((Box::new(left), op, Box::new(right)));
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
        OmlExpr::Op3((expr1, expr2, expr3))
    }

    fn parse_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let root_item = root.into_inner().next().unwrap();
        OmlExpr::Value(match root_item.as_rule() {
            Rule::boolean_literal => OmlValue::Bool(root_item.as_str() == "true"),
            Rule::number_literal => match root_item.as_str().parse::<i64>() {
                Ok(n) => OmlValue::Int64(n),
                Err(_) => OmlValue::String(root_item.as_str().into_base()),
            },
            Rule::string_literal => OmlValue::String(root_item.as_str().into_base()),
            Rule::format_string_literal => return Self::parse_format_string_literal(root_item),
            _ => unreachable!(),
        })
    }

    fn parse_format_string_literal(root: pest::iterators::Pair<'_, Rule>) -> OmlExpr {
        let mut strs = vec![];
        let mut exprs = vec![];
        for root_item in root.into_inner() {
            match root_item.as_rule() {
                Rule::format_string => {
                    return OmlExpr::Value(OmlValue::String(root_item.as_str().into_base()));
                }
                Rule::format_string_part1 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part2 => strs.push(root_item.as_str().into_base()),
                Rule::format_string_part3 => strs.push(root_item.as_str().into_base()),
                Rule::expr => exprs.push(Self::parse_expr(root_item)),
                _ => unreachable!(),
            }
        }
        OmlExpr::FormatString((strs, exprs))
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
                Ok((result, success)) if success => return Ok(result),
                Ok((result, _)) => last_result = result,
                Err(err) => return Err(err),
            }
        }
        Err("evalute failed.".to_string())
    }

    fn evalute2(&self, path: &str, last_result: &OmlValue) -> Result<(OmlValue, bool), String> {
        let mut success = true;
        let value = match self {
            OmlExpr::None => OmlValue::None,
            OmlExpr::Value(val) => val.clone(),
            OmlExpr::Array(arr) => {
                let mut ret = vec![];
                for (index, item) in arr.iter().enumerate() {
                    let new_path = path.append_num(index);
                    let (val, tmp_success) = item.evalute2(&new_path, last_result)?;
                    ret.push(val);
                    success &= tmp_success;
                }
                OmlValue::Array(ret)
            }
            OmlExpr::Map(map) => {
                let mut ret = HashMap::new();
                for (key, value) in map.iter() {
                    let new_path = path.append_str(key);
                    let (val, tmp_success) = value.evalute2(&new_path, last_result)?;
                    ret.insert(key.clone(), val);
                    success &= tmp_success;
                }
                OmlValue::Map(ret)
            }
            OmlExpr::TempName(name) => {
                match last_result.get(&path.remove_once().append_str(name)) {
                    Some(val) => val.clone(),
                    None => {
                        success = false;
                        OmlValue::None
                    }
                }
            }
            OmlExpr::Op1Prefix((name, expr)) => {
                let (val, tmp_success) = expr.evalute2(path, last_result)?;
                success &= tmp_success;
                if tmp_success {
                    Op1Evaluator::eval_prefix(name, val)?
                } else {
                    OmlValue::None
                }
            }
            OmlExpr::Op1Suffix((expr, name)) => {
                let (val, tmp_success) = expr.evalute2(path, last_result)?;
                success &= tmp_success;
                if tmp_success {
                    Op1Evaluator::eval_suffix(name, val)?
                } else {
                    OmlValue::None
                }
            }
            OmlExpr::Op2((left, op, right)) => {
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
            OmlExpr::Op3((cond, left, right)) => {
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
            OmlExpr::FormatString((strs, exprs)) => {
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
            OmlExpr::AccessVar(_) => todo!(),
            OmlExpr::InvokeFunc(_) => todo!(),
            OmlExpr::IfAnno(if_anno) => {
                for (cond, value) in if_anno.exprs.iter() {
                    let (cond_val, tmp_success) = cond.evalute2(path, last_result)?;
                    if tmp_success {
                        if cond_val.as_bool().unwrap_or(false) {
                            return value.evalute2(path, last_result);
                        }
                    } else {
                        success = false;
                    }
                }
                match &if_anno.default {
                    Some(val) => {
                        return val.evalute2(path, last_result);
                    }
                    None => OmlValue::None,
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
        self.get_at(index).unwrap_or(&NULL_EXPR)
    }
}

impl IndexMut<usize> for OmlExpr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_at_mut(index)
    }
}

impl Index<&str> for OmlExpr {
    type Output = OmlExpr;
    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap_or(&NULL_EXPR)
    }
}

impl IndexMut<&str> for OmlExpr {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl OmlExpr {
    pub fn get_at(&self, index: usize) -> Option<&Self> {
        if let OmlExpr::Array(arr) = self {
            arr.get(index)
        } else {
            None
        }
    }

    pub fn get_at_mut(&mut self, index: usize) -> &mut Self {
        if let OmlExpr::Array(arr) = self {
            if (index + 1) > arr.len() {
                arr.extend(
                    (arr.len()..(index + 1))
                        .into_iter()
                        .map(|_| OmlExpr::new())
                        .collect::<Vec<_>>(),
                )
            }
        } else {
            *self = OmlExpr::Array(
                (0..(index + 1))
                    .into_iter()
                    .map(|_| OmlExpr::new())
                    .collect(),
            );
        }
        if let OmlExpr::Array(arr) = self {
            arr.get_mut(index).unwrap()
        } else {
            panic!()
        }
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        let path_items: Vec<_> = index.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.len() == 0 {
                continue;
            } else if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                let num: usize = num.parse().unwrap();
                if let Some(obj) = obj_ref.get_at(num) {
                    obj_ref = obj;
                } else {
                    return None;
                }
            } else {
                if let OmlExpr::Map(map) = obj_ref {
                    if let Some(obj) = map.get(path_item) {
                        obj_ref = obj;
                        continue;
                    }
                }
                return None;
            }
        }
        Some(obj_ref)
    }

    pub fn get_mut(&mut self, index: &str) -> &mut Self {
        let path_items: Vec<_> = index.split('.').collect();
        let mut obj_ref = self;
        for path_item in path_items.into_iter() {
            if path_item.len() == 0 {
                continue;
            } else if path_item.starts_with('[') {
                let num = &path_item[1..path_item.len() - 1];
                let num: usize = num.parse().unwrap();
                obj_ref = obj_ref.get_at_mut(num);
            } else {
                let map = match obj_ref {
                    OmlExpr::Map(map) => map,
                    _ => {
                        *obj_ref = OmlExpr::Map(HashMap::new());
                        match obj_ref {
                            OmlExpr::Map(map) => map,
                            _ => panic!(),
                        }
                    }
                };
                obj_ref = map.entry(path_item.to_string()).or_insert(OmlExpr::new());
            }
        }
        obj_ref
    }
}

impl OmlExpr {
    pub fn is_map(&self) -> bool {
        match self {
            OmlExpr::Map(_) => true,
            _ => false,
        }
    }
}

impl OmlExpr {
    pub fn set_null(&mut self) {
        *self = OmlExpr::None;
    }

    pub fn set_bool(&mut self, val: bool) {
        *self = OmlExpr::Value(OmlValue::Bool(val));
    }

    pub fn set_int(&mut self, val: i64) {
        *self = OmlExpr::Value(OmlValue::Int64(val));
    }

    pub fn set_float(&mut self, val: f64) {
        *self = OmlExpr::Value(OmlValue::Float64(val));
    }

    pub fn set_string(&mut self, val: impl Into<String>) {
        *self = OmlExpr::Value(OmlValue::String(val.into()));
    }
}
