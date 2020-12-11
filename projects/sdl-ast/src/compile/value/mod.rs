pub use crate::compile::value::template::HTMLElement;
use std::collections::{BTreeMap, BTreeSet};
use crate::Result;
use num::BigInt;

mod convert;
mod template;
mod ops;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Block(Vec<Value>),
    Null,
    Boolean(bool),
    String(String),
    Integer(Box<BigInt>),
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
