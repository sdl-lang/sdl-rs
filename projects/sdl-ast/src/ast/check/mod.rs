use super::*;
use num::Zero;

impl ASTKind {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
    pub fn is_true(&self) -> bool {
        matches!(self, Self::Boolean(true))
    }
    pub fn is_false(&self) -> bool {
        matches!(self, Self::Boolean(false))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Self::HTMLText(_) | Self::String(_))
    }
    pub fn is_safe(&self) -> bool {
        unreachable!()
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Integer(_) | Self::Decimal(_))
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    pub fn is_decimal(&self) -> bool {
        matches!(self, Self::Decimal(_))
    }
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Integer(v) => v.is_zero(),
            Self::Decimal(v) => v.is_zero(),
            _ => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            // Self::String(_) => {
            //     unimplemented!()
            // }
            Self::List(v) => v.is_empty(),
            // Self::Dict(v) => {
            //     v.is_empty()
            // }
            _ => false,
        }
    }
    pub fn is_falsy(&self) -> bool {
        match self {
            Self::Null => true,
            Self::Boolean(false) => true,
            // Self::String(_) => {
            //     unimplemented!()
            // }
            Self::Integer(v) => v.is_zero(),
            Self::Decimal(v) => v.is_zero(),
            Self::List(v) => v.is_empty(),
            // Self::Dict(v) => {
            //     v.is_empty()
            // }
            _ => true,
        }
    }
}
