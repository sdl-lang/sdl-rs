use super::*;

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl From<char> for Value {
    fn from(v: char) -> Self {
        StringValue::non_escaped(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        StringValue::non_escaped(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        StringValue::non_escaped(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}
