use super::*;
use crate::utils::get_variant_name;
use num::{ToPrimitive, Zero, Integer};
use std::ops::{Add, Div, Mul, Sub, Neg};
use crate::ASTNode;

pub trait Concat<Rhs = Self> {
    type Output;
    fn concat(self, rhs: Rhs) -> Self::Output;
}

impl Add<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn add(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(Value::{}(lhs), Value::{}(rhs)) => {{}}", get_variant_name(&self), get_variant_name(&rhs));
        unimplemented!("{}", error)
        // let out = match (self.kind, rhs.kind) {
        //     (Value::String(lhs), Value::String(rhs)) => Value::String(Box::new(lhs + rhs)),
        //     (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() + rhs.as_ref())),
        //     (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() + rhs.as_ref())),
        //     (Value::Decimal(lhs), Value::Integer(rhs)) | (Value::Integer(rhs), Value::Decimal(lhs)) => {
        //         Value::Decimal(Box::new(lhs.as_ref() + BigDecimal::from(rhs.as_ref().clone())))
        //     }
        //     _ => {
        //         println!("{}", error);
        //         unreachable!()
        //     }
        // };
        // Ok(out)
    }
}

impl Sub<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn sub(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        unimplemented!("{}", error)
        // let out = match (self, rhs) {
        //     (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() - rhs.as_ref())),
        //     (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() - rhs.as_ref())),
        //     (Value::Decimal(lhs), Value::Integer(rhs)) | (Value::Integer(rhs), Value::Decimal(lhs)) => {
        //         Value::Decimal(Box::new(lhs.as_ref() - BigDecimal::from(rhs.as_ref().clone())))
        //     }
        //     _ => unimplemented!("{}", error),
        // };
        // Ok(out)
    }
}

impl Mul<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn mul(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        unimplemented!("{}", error)
        // let out = match (self, rhs) {
        //     (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() * rhs.as_ref())),
        //     (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() * rhs.as_ref())),
        //     _ => unimplemented!("{}", error),
        // };
        // Ok(out)
    }
}

impl Div<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn div(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        unimplemented!("{}", error)
        // let out = match (self, rhs) {
        //     (Value::Integer(lhs), Value::Integer(rhs)) => Value::Integer(Box::new(lhs.as_ref() / rhs.as_ref())),
        //     (Value::Decimal(lhs), Value::Decimal(rhs)) => Value::Decimal(Box::new(lhs.as_ref() / rhs.as_ref())),
        //     _ => unimplemented!("{}", error),
        // };
        // Ok(out)
    }
}

impl Concat<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn concat(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(Value::{:?}, Value::{:?}) => {{}}", &self, &rhs);
        unimplemented!("{}", error)
        // let out = match (self, rhs) {
        //     (Value::Integer(lhs), Value::Integer(rhs)) => {
        //       let new = lhs.mul(rhs.to_string().len()) + rhs.as_ref();
        //         Value::Integer(Box::new(new))
        //     },
        //     (Value::Integer(lhs), Value::String(rhs)) => {
        //         StringValue::non_escaped(lhs.to_string() + rhs.as_str())
        //     }
        //     _ => unimplemented!("{}", error),
        // };
        // Ok(out)
    }
}

impl ASTNode {
    pub fn get_index(&self, n: &BigInt) -> Result<ASTNode> {
        unimplemented!("{:?}", self);
        // match n {
        //     n if n > &BigInt::zero()  => {
        //         // TODO: Invalid Index Error
        //         let n = n.to_usize().unwrap_or_default() - 1 ;
        //         let out = match self {
        //             Value::List(list) => list.get(n).cloned().unwrap_or_default(),
        //             Value::String(string) => {
        //                 match string.chars().nth(n) {
        //                     Some(s) => StringValue::non_escaped(s),
        //                     None => Value::Null,
        //                 }
        //             },
        //             Value::Null => Value::Null,
        //
        //             _ => unimplemented!("{:?}", self),
        //         };
        //         Ok(out)
        //     }
        //     n if n < &BigInt::zero() => {
        //         // TODO: Invalid Index Error
        //         let n = n.neg().to_usize().unwrap_or_default();
        //         let out = match self {
        //             Value::List(list) => {
        //                 let l = match list.len().checked_sub(n) {
        //                     Some(u) => {u},
        //                     None => {return Ok(Value::Null)}
        //                 };
        //                 list.get(l).cloned().unwrap_or_default()
        //             },
        //             Value::String(string) => {
        //                 let l = match string.length().checked_sub(n) {
        //                     Some(u) => {u},
        //                     None => {return Ok(Value::Null)}
        //                 };
        //                 match string.chars().nth(l) {
        //                     Some(s) => StringValue::non_escaped(s),
        //                     None => Value::Null,
        //                 }
        //             },
        //             Value::Null => Value::Null,
        //
        //             _ => unimplemented!("{:?}", self),
        //         };
        //         Ok(out)
        //     }
        //     // n is zero
        //     _ => {
        //         Ok(Value::Null)
        //     }
        // }
    }
}
