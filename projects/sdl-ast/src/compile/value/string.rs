use std::ops::{Add};
use super::*;
use std::str::Chars;


#[derive(Clone, Eq, PartialEq)]
pub enum StringValue {
    Escaped(String),
    Unsafe(String),
}

impl Add<StringValue> for StringValue {
    type Output = Self;

    fn add(self, rhs: StringValue) -> Self::Output {
        match (self,rhs) {
            (Self::Unsafe(lhs), Self::Unsafe(rhs)) => {
                Self::Unsafe(lhs+&rhs)
            }
            (Self::Escaped(lhs), Self::Escaped(rhs)) => {
                Self::Escaped(lhs+&rhs)
            }
            (Self::Escaped(lhs), Self::Unsafe(rhs))|(Self::Unsafe(rhs), Self::Escaped(lhs)) => {
                Self::Escaped(lhs + &string_escape(&rhs))
            }
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
            StringValue::Escaped(lhs) => {
                match rhs {
                    StringValue::Escaped(rhs) => {Self::Escaped(lhs+rhs)}
                    StringValue::Unsafe(rhs) => {Self::Escaped(lhs + &string_escape(rhs))}
                }
            }
            StringValue::Unsafe(lhs) => {
                match rhs {
                    StringValue::Escaped(rhs) => {Self::Escaped(string_escape(&lhs) + rhs)}
                    StringValue::Unsafe(rhs) => {Self::Unsafe(lhs+rhs)}
                }
            }
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
        Value::String(Box::new(Self::Escaped(s.into())))
    }
    pub fn non_escaped(s: impl Into<String>) -> Value {
        Value::String(Box::new(Self::Unsafe(s.into())))
    }
    pub fn chars(&self) -> Chars<'_> {
        match self {
            StringValue::Escaped(s) => {
                s.chars()
            }
            StringValue::Unsafe(s) => {
                s.chars()
            }
        }
    }
}

impl Debug for StringValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Escaped(s) => {
                write!(f, "{:?}", s)
            }
            Self::Unsafe(s) => {
                write!(f, "{:?}", string_escape(s))
            }
        }
    }
}


fn string_escape(input: &str) -> String {
    input.to_owned()
}

fn string_unescape(input: &str) -> String {
    input.to_owned()
}