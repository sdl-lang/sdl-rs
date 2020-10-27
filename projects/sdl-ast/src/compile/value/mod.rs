use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    List(Vec<Value>),
    Dict(BTreeMap<String, Value>),
    HTMLElement,

}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _=> false
        }
    }
}