pub use crate::compile::value::template::HTMLElement;
use crate::Result;
use bigdecimal::BigDecimal;
use num::BigInt;
use std::collections::{BTreeMap, BTreeSet};

mod convert;
mod ops;
mod template;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Block(Vec<Value>),
    Null,
    Boolean(bool),
    String(String),
    UnsafeString(String),
    Integer(Box<BigInt>),
    Decimal(Box<BigDecimal>),
    List(Vec<Value>),
    Dict(BTreeMap<String, Value>),
    HTMLElement(Box<HTMLElement>),
}

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }
}
