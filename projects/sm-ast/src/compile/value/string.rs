use super::*;
use std::{ops::Add, str::Chars};

#[derive(Clone, Eq, PartialEq)]
pub enum StringValue {
    Normal(String),
    HTMLEscaped(String),
}

impl Add<StringValue> for StringValue {
    type Output = Self;

    fn add(self, rhs: StringValue) -> Self::Output {
        match (self, rhs) {
            (Self::Normal(lhs), Self::Normal(rhs)) => Self::Normal(lhs + &rhs),
            (Self::HTMLEscaped(lhs), Self::HTMLEscaped(rhs)) => Self::HTMLEscaped(lhs + &rhs),
            (Self::HTMLEscaped(lhs), Self::Normal(rhs)) | (Self::Normal(rhs), Self::HTMLEscaped(lhs)) => Self::HTMLEscaped(lhs + &string_escape(&rhs)),
        }
    }
}

impl Add<&StringValue> for StringValue {
    type Output = Self;

    fn add(self, rhs: &StringValue) -> Self::Output {
        // FIXME: #![feature(move_ref_pattern)]
        // match (self,rhs) {
        //     (Self::Unsafe(lhs), Self::Unsafe(rhs)) => {
        //         Self::Unsafe(lhs+rhs)
        //     }
        //     (Self::Escaped(lhs), Self::Escaped(rhs)) => {
        //         Self::Escaped(lhs+rhs)
        //     }
        //     (Self::Escaped(lhs), Self::Unsafe(rhs)) => {
        //         Self::Escaped(lhs + &string_escape(rhs))
        //     }
        //     (Self::Unsafe(lhs), Self::Escaped(rhs)) => {
        //         Self::Escaped(string_escape(&lhs) + rhs)
        //     }
        // }
        match self {
            StringValue::HTMLEscaped(lhs) => match rhs {
                StringValue::HTMLEscaped(rhs) => Self::HTMLEscaped(lhs + rhs),
                StringValue::Normal(rhs) => Self::HTMLEscaped(lhs + &string_escape(rhs)),
            },
            StringValue::Normal(lhs) => match rhs {
                StringValue::HTMLEscaped(rhs) => Self::HTMLEscaped(string_escape(&lhs) + rhs),
                StringValue::Normal(rhs) => Self::Normal(lhs + rhs),
            },
        }
    }
}

impl Add<Box<StringValue>> for Box<StringValue> {
    type Output = StringValue;

    fn add(self, rhs: Box<StringValue>) -> Self::Output {
        self.as_ref().clone() + rhs.as_ref()
    }
}

impl StringValue {
    pub fn escaped(s: impl Into<String>) -> Value {
        Value::String(Box::new(Self::HTMLEscaped(s.into())))
    }
    pub fn non_escaped(s: impl Into<String>) -> Value {
        Value::String(Box::new(Self::Normal(s.into())))
    }
    pub fn chars(&self) -> Chars<'_> {
        match self {
            StringValue::HTMLEscaped(s) => s.chars(),
            StringValue::Normal(s) => s.chars(),
        }
    }
}

impl Debug for StringValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::HTMLEscaped(s) => write!(f, "{:?}", s),
            Self::Normal(s) => write!(f, "{:?}", string_escape(s)),
        }
    }
}

fn string_escape(input: &str) -> String {
    input.to_owned()
}

fn string_unescape(input: &str) -> String {
    input.to_owned()
}

impl StringValue {
    pub fn length(&self) -> usize {
        match self {
            StringValue::HTMLEscaped(s) => {s.len()}
            StringValue::Normal(s) => {s.len()}
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            StringValue::HTMLEscaped(s) => {s.as_str()}
            StringValue::Normal(s) => {s.as_str()}
        }
    }
}