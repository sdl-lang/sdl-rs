pub use crate::compile::value::{string::StringValue, template::HTMLElement};
use crate::Result;
use bigdecimal::BigDecimal;
use num::BigInt;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Debug, Formatter},
};
mod check;
mod convert;
mod ops;
mod string;
mod template;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Block(Vec<Value>),
    Null,
    Boolean(bool),
    String(Box<StringValue>),
    Integer(Box<BigInt>),
    Decimal(Box<BigDecimal>),
    List(Vec<Value>),
    Dict(BTreeMap<String, Value>),
    HTMLElement(Box<HTMLElement>),
}
