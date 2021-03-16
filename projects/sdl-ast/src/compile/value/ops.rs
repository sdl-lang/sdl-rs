use super::*;
use crate::utils::get_variant_name;
use num::{ToPrimitive, Zero};
use std::ops::{Add, Div, Mul, Sub, Neg};

impl Add<Value> for Value {
    type Output = Result<Value>;

    fn add(self, rhs: Value) -> Self::Output {
        let error = format!("(Value::{}(lhs), Value::{}(rhs)) => {{}}", get_variant_name(&self), get_variant_name(&rhs));
        let out = match (self, rhs) {
            (Value::String(lhs), Value::String(rhs)) => Value::String(Box::new(lhs + rhs)),
            (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() + rhs.as_ref())),
            (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() + rhs.as_ref())),
            (Value::Decimal(lhs), Value::Integer(rhs)) | (Value::Integer(rhs), Value::Decimal(lhs)) => {
                Value::Decimal(Box::new(lhs.as_ref() + BigDecimal::from(rhs.as_ref().clone())))
            }
            _ => {
                println!("{}", error);
                unreachable!()
            }
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
            (Value::Decimal(lhs), Value::Integer(rhs)) | (Value::Integer(rhs), Value::Decimal(lhs)) => {
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
        match n {
            n if n > &BigInt::zero()  => {
                // TODO: Invalid Index Error
                let n = n.to_usize().unwrap_or_default() - 1 ;
                let out = match self {
                    Value::List(list) => list.get(n).cloned().unwrap_or_default(),
                    Value::String(string) => {
                        match string.chars().nth(n) {
                            Some(s) => StringValue::non_escaped(s),
                            None => Value::Null,
                        }
                    },
                    Value::Null => Value::Null,

                    _ => unimplemented!("{:?}", self),
                };
                Ok(out)
            }
            n if n < &BigInt::zero() => {
                // TODO: Invalid Index Error
                let n = n.neg().to_usize().unwrap_or_default();
                let out = match self {
                    Value::List(list) => {
                        let l = match list.len().checked_sub(n) {
                            Some(u) => {u},
                            None => {return Ok(Value::Null)}
                        };
                        list.get(l).cloned().unwrap_or_default()
                    },
                    Value::String(string) => {
                        let l = match string.length().checked_sub(n) {
                            Some(u) => {u},
                            None => {return Ok(Value::Null)}
                        };
                        match string.chars().nth(l) {
                            Some(s) => StringValue::non_escaped(s),
                            None => Value::Null,
                        }
                    },
                    Value::Null => Value::Null,

                    _ => unimplemented!("{:?}", self),
                };
                Ok(out)
            }
            // n is zero
            _ => {
                Ok(Value::Null)
            }
        }



    }
}
