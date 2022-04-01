use super::*;
use num::Zero;

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false
        }
    }
    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false
        }
    }
    pub fn is_safe(&self) -> bool {
        unreachable!()
    }
    pub fn is_number(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            Value::Decimal(_) => true,
            _ => false
        }
    }
    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false
        }
    }
    pub fn is_decimal(&self) -> bool {
        match self {
            Value::Decimal(_) => true,
            _ => false
        }
    }
    pub fn is_zero(&self) -> bool {
        match self {
            Value::Decimal(v) => {
                v.is_zero()
            }
            _ => false
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Value::String(_) => {
                unimplemented!()
            }
            Value::List(v) => {
                v.is_empty()
            }
            Value::Dict(v) => {
                v.is_empty()
            }
            _ => false
        }
    }
    pub fn is_falsy(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Boolean(false) => true,
            Value::String(_) => {
                unimplemented!()
            }
            Value::Integer(v) => {
                v.is_zero()
            }
            Value::Decimal(v) => {
                v.is_zero()
            }
            Value::List(v) => {
                v.is_empty()
            }
            Value::Dict(v) => {
                v.is_empty()
            }
            _ => true
        }
    }
}
