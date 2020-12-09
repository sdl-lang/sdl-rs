pub use crate::compile::value::template::HTMLElement;
use std::collections::{BTreeMap, BTreeSet};

mod convert;
mod template;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Block(Vec<Value>),

    Null,
    Boolean(bool),
    String(String),
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
