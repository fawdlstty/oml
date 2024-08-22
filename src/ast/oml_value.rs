use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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
    pub fn is_none(&self) -> bool {
        match self {
            OmlValue::None => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            OmlValue::Bool(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            OmlValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            OmlValue::Int64(_) => true,
            _ => false,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            OmlValue::Int64(i) => Some(*i),
            _ => None,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            OmlValue::Float64(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            OmlValue::String(_) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            OmlValue::None => "none".to_string(),
            OmlValue::Bool(b) => b.to_string(),
            OmlValue::Int64(i) => i.to_string(),
            OmlValue::Float64(f) => f.to_string(),
            OmlValue::String(s) => s.clone(),
            OmlValue::Array(arr) => {
                let arr: Vec<_> = arr.iter().map(|item| item.as_str()).collect();
                format!("[{}]", arr.join(", "))
            }
            OmlValue::Map(map) => {
                let mut ret = "{ ".to_string();
                for (key, value) in map.iter() {
                    if !ret.is_empty() {
                        ret.push_str(", ");
                    }
                    ret.push_str(key);
                    ret.push_str(": ");
                    ret.push_str(&value.as_str());
                }
                ret.push_str(" }");
                ret
            }
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            OmlValue::Float64(f) => Some(*f),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            OmlValue::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_array(&self) -> Option<Vec<OmlValue>> {
        match self {
            OmlValue::Array(arr) => Some(arr.clone()),
            _ => None,
        }
    }

    pub fn is_map(&self) -> bool {
        match self {
            OmlValue::Map(_) => true,
            _ => false,
        }
    }

    pub fn as_map(&self) -> Option<HashMap<String, OmlValue>> {
        match self {
            OmlValue::Map(map) => Some(map.clone()),
            _ => None,
        }
    }

    fn apply(&mut self, val: OmlValue) {
        match self {
            OmlValue::Array(arr) => arr.push(val),
            OmlValue::Map(map) => {
                if let OmlValue::Map(map2) = val {
                    map.apply(map2);
                } else {
                    *self = val;
                }
            }
            _ => *self = val,
        }
    }
}

impl Index<usize> for OmlValue {
    type Output = OmlValue;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            OmlValue::Array(arr) => arr.get(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for OmlValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            OmlValue::Array(arr) => arr.get_mut(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl Index<&str> for OmlValue {
    type Output = OmlValue;
    fn index(&self, index: &str) -> &Self::Output {
        match self {
            OmlValue::Map(map) => map.get(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl IndexMut<&str> for OmlValue {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match self {
            OmlValue::Map(map) => map.get_mut(index).unwrap(),
            _ => panic!(),
        }
    }
}

impl OmlValue {
    pub fn get_at(&self, index: usize) -> Option<&Self> {
        if let OmlValue::Array(arr) = self {
            arr.get(index)
        } else {
            None
        }
    }

    pub fn get_at_mut(&mut self, index: usize) -> Option<&mut Self> {
        if let OmlValue::Array(arr) = self {
            arr.get_mut(index)
        } else {
            None
        }
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        match index.split_once('.') {
            Some((a, b)) => {
                let ret = match a.parse::<usize>() {
                    Ok(i) => self.get_at(i),
                    Err(_) => self.get(a),
                };
                match ret {
                    Some(val) => val.get(b),
                    None => None,
                }
            }
            None => {
                if let OmlValue::Map(map) = self {
                    map.get(index)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_mut(&mut self, index: &str) -> Option<&mut Self> {
        if let OmlValue::Map(map) = self {
            map.get_mut(index)
        } else {
            None
        }
    }
}

pub trait OmlValueGetExt {
    fn get(&self, index: &str) -> Option<&OmlValue>;
    // fn get_mut(&mut self, index: &str) -> Option<&mut OmlValue>;
}

impl OmlValueGetExt for Option<&OmlValue> {
    fn get(&self, index: &str) -> Option<&OmlValue> {
        match *self {
            Some(value) => value.get(index),
            None => None,
        }
    }
}

pub trait ApplyExt {
    fn apply(&mut self, val: Self);
}

impl ApplyExt for HashMap<String, OmlValue> {
    fn apply(&mut self, val: Self) {
        for (key, mut val) in val.into_iter() {
            if let Some(self_k) = self.get_mut(&key) {
                self_k.apply(val);
            } else {
                self.insert(key, val);
            }
        }
    }
}
