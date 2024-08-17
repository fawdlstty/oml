use super::oml_value::{ApplyExt, OmlValue};

pub(crate) struct Op2Evaluator {}

impl Op2Evaluator {
    pub fn eval(mut left: OmlValue, op: &str, mut right: OmlValue) -> Result<OmlValue, String> {
        match (left, op, right) {
            (OmlValue::Bool(left), _, OmlValue::Bool(right)) => {
                Ok(OmlValue::Bool(Self::eval_bool(left, op, right)?))
            }
            (OmlValue::Int64(left), _, OmlValue::Int64(right)) => Self::eval_int64(left, op, right),
            (OmlValue::Float64(left), _, OmlValue::Float64(right)) => {
                Self::eval_float64(left, op, right)
            }
            (OmlValue::Int64(left), _, OmlValue::Float64(right)) => {
                Self::eval_float64(left as f64, op, right)
            }
            (OmlValue::Float64(left), _, OmlValue::Int64(right)) => {
                Self::eval_float64(left, op, right as f64)
            }
            (OmlValue::String(left), _, OmlValue::String(right)) => {
                Self::eval_string(&left, op, &right)
            }
            (OmlValue::String(left), "*", OmlValue::Int64(right)) if right >= 0 => {
                Ok(OmlValue::String(left.repeat(right as usize)))
            }
            (OmlValue::Array(left), "+", OmlValue::Array(right)) => {
                let mut left = left.clone();
                left.extend(right.clone());
                Ok(OmlValue::Array(left))
            }
            (OmlValue::Map(left), "+", OmlValue::Map(right)) => {
                let mut left = left.clone();
                left.apply(right.clone());
                Ok(OmlValue::Map(left))
            }
            _ => Err(format!("illegal operator: {}", op)),
        }
    }

    fn eval_bool(left: bool, op: &str, right: bool) -> Result<bool, String> {
        Ok(match op {
            "&&" => left && right,
            "||" => left || right,
            "==" => left == right,
            "!=" => left != right,
            _ => return Err(format!("illegal operator: {}", op)),
        })
    }

    fn eval_int64(left: i64, op: &str, right: i64) -> Result<OmlValue, String> {
        Ok(OmlValue::Int64(match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "**" if right < 0 => return Ok(OmlValue::Float64((left as f64).powf(right as f64))),
            "**" => left.pow(right as u32),
            "%" => left % right,
            "|" => left | right,
            "&" => left & right,
            "<<" => left << right,
            ">>" => left >> right,
            "^" => left ^ right,
            "<" => return Ok(OmlValue::Bool(left < right)),
            "<=" => return Ok(OmlValue::Bool(left <= right)),
            ">" => return Ok(OmlValue::Bool(left > right)),
            ">=" => return Ok(OmlValue::Bool(left >= right)),
            "==" => return Ok(OmlValue::Bool(left == right)),
            "!=" => return Ok(OmlValue::Bool(left != right)),
            _ => return Err(format!("illegal operator: {}", op)),
        }))
    }

    fn eval_float64(left: f64, op: &str, right: f64) -> Result<OmlValue, String> {
        Ok(OmlValue::Float64(match op {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            "**" => left.powf(right),
            "%" => left % right,
            "<" => return Ok(OmlValue::Bool(left < right)),
            "<=" => return Ok(OmlValue::Bool(left <= right)),
            ">" => return Ok(OmlValue::Bool(left > right)),
            ">=" => return Ok(OmlValue::Bool(left >= right)),
            "==" => return Ok(OmlValue::Bool(left == right)),
            "!=" => return Ok(OmlValue::Bool(left != right)),
            _ => return Err(format!("illegal operator: {}", op)),
        }))
    }

    fn eval_string(left: &str, op: &str, right: &str) -> Result<OmlValue, String> {
        match op {
            "+" => Ok(OmlValue::String(format!("{}{}", left, right))),
            "==" => Ok(OmlValue::Bool(left == right)),
            "!=" => Ok(OmlValue::Bool(left != right)),
            _ => Err(format!("illegal operator: {}", op)),
        }
    }
}
