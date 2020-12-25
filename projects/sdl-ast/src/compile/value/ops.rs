use super::*;
use num::ToPrimitive;
use std::ops::{Add, Div, Mul, Sub};
use crate::utils::get_variant_name;

impl Add<Value> for Value {
    type Output = Result<Value>;

    fn add(self, rhs: Value) -> Self::Output {
        let error = format!("(Value::{}(lhs), Value::{}(rhs)) => {{}}", get_variant_name(&self), get_variant_name(&rhs));
        let out = match (self, rhs) {
            (Value::String(lhs), Value::String(rhs)) => Value::String(lhs + &rhs),
            (Value::UnsafeString(lhs), Value::UnsafeString(rhs)) => Value::String(lhs + &rhs),
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() + rhs.as_ref())),
            (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() + rhs.as_ref())),
            (Value::Decimal(lhs), Value::Integer(rhs))|(Value::Integer(rhs), Value::Decimal(lhs)) => {
                Value::Decimal(Box::new(lhs.as_ref() + BigDecimal::from(rhs.as_ref().clone())))
            }
            _ => {
                println!("{}", error);
                unreachable!()
            },
        };
        Ok(out)
    }
}

impl Sub<Value> for Value {
    type Output = Result<Value>;

    fn sub(self, rhs: Value) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        let out = match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() - rhs.as_ref())),
            (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() - rhs.as_ref())),
            (Value::Decimal(lhs), Value::Integer(rhs))|(Value::Integer(rhs), Value::Decimal(lhs)) => {
                Value::Decimal(Box::new(lhs.as_ref() - BigDecimal::from(rhs.as_ref().clone())))
            }
            _ => unimplemented!("{}", error),
        };
        Ok(out)
    }
}

impl Mul<Value> for Value {
    type Output = Result<Value>;

    fn mul(self, rhs: Value) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        let out = match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() * rhs.as_ref())),
            (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() * rhs.as_ref())),
            _ => unimplemented!("{}", error),
        };
        Ok(out)
    }
}

impl Div<Value> for Value {
    type Output = Result<Value>;

    fn div(self, rhs: Value) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        let out = match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() / rhs.as_ref())),
            (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() / rhs.as_ref())),
            _ => unimplemented!("{}", error),
        };
        Ok(out)
    }
}

impl Value {
    pub fn get_index(&self, n: &BigInt) -> Result<Value> {
        // TODO: Invalid Index Error
        let n = n.to_usize().unwrap_or_default();
        let out = match self {
            Value::List(list) => match list.get(n) {
                Some(s) => s.to_owned(),
                None => Value::Null,
            },
            Value::String(string) => match string.chars().nth(n) {
                Some(s) => Value::from(s),
                None => Value::Null,
            },
            Value::Null => Value::Null,

            _ => unimplemented!("{:?}", self),
        };
        Ok(out)
    }
}
