use crate::utils::get_variant_name;
use num::{ToPrimitive, Zero, Integer, BigInt};
use std::ops::{Add, Div, Mul, Sub, Neg};
use crate::{ASTNode, ASTKind};
use crate::Result;
use crate::traits::Concat;
use bigdecimal::BigDecimal;

impl Add<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn add(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(ASTKind::{}(lhs), ASTKind::{}(rhs)) => {{}}", get_variant_name(&self.kind), get_variant_name(&rhs.kind));
        let out = match (self.kind, rhs.kind) {
            (ASTKind::String(lhs), ASTKind::String(rhs)) => ASTKind::String(lhs + rhs.as_ref()),
            (ASTKind::Integer(lhs), ASTKind::Integer(rhs)) => ASTKind::Integer(Box::new(lhs.as_ref() + rhs.as_ref())),
            (ASTKind::Decimal(lhs), ASTKind::Decimal(rhs)) => ASTKind::Decimal(Box::new(lhs.as_ref() + rhs.as_ref())),
            (ASTKind::Decimal(lhs), ASTKind::Integer(rhs)) | (ASTKind::Integer(rhs), ASTKind::Decimal(lhs)) => {
                ASTKind::Decimal(Box::new(lhs.as_ref() + BigDecimal::from(rhs.as_ref().clone())))
            }
            _ => {
                println!("{}", error);
                unreachable!()
            }
        };
        Ok(ASTNode {
            kind: out,
            range: self.range
        })
    }
}

impl Sub<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn sub(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(ASTKind::{}(lhs), ASTKind::{}(rhs)) => {{}}", get_variant_name(&self.kind), get_variant_name(&rhs.kind));
        let out = match (self.kind, rhs.kind) {
            (ASTKind::Integer(lhs), ASTKind::Integer(rhs)) => ASTKind::Integer(Box::new(lhs.as_ref() - rhs.as_ref())),
            (ASTKind::Decimal(lhs), ASTKind::Decimal(rhs)) => ASTKind::Decimal(Box::new(lhs.as_ref() - rhs.as_ref())),
            (ASTKind::Decimal(lhs), ASTKind::Integer(rhs)) | (ASTKind::Integer(rhs), ASTKind::Decimal(lhs)) => {
                ASTKind::Decimal(Box::new(lhs.as_ref() - BigDecimal::from(rhs.as_ref().clone())))
            }
            _ => unimplemented!("{}", error),
        };
        Ok(ASTNode {
            kind: out,
            range: self.range
        })
    }
}

impl Mul<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn mul(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(ASTKind::{}(lhs), ASTKind::{}(rhs)) => {{}}", get_variant_name(&self.kind), get_variant_name(&rhs.kind));
        let out = match (self.kind, rhs.kind) {
            (ASTKind::Integer(lhs), ASTKind::Integer(rhs)) => ASTKind::Integer(Box::new(lhs.as_ref() * rhs.as_ref())),
            (ASTKind::Decimal(lhs), ASTKind::Decimal(rhs)) => ASTKind::Decimal(Box::new(lhs.as_ref() * rhs.as_ref())),
            _ => unimplemented!("{}", error),
        };
        Ok(ASTNode {
            kind: out,
            range: self.range
        })
    }
}

impl Div<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn div(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(ASTKind::{}(lhs), ASTKind::{}(rhs)) => {{}}", get_variant_name(&self.kind), get_variant_name(&rhs.kind));
        let out = match (self.kind, rhs.kind) {
            (ASTKind::Integer(lhs), ASTKind::Integer(rhs)) => ASTKind::Integer(Box::new(lhs.as_ref() / rhs.as_ref())),
            (ASTKind::Decimal(lhs), ASTKind::Decimal(rhs)) => ASTKind::Decimal(Box::new(lhs.as_ref() / rhs.as_ref())),
            _ => unimplemented!("{}", error),
        };
        Ok(ASTNode {
            kind: out,
            range: self.range
        })
    }
}

impl Concat<ASTNode> for ASTNode {
    type Output = Result<ASTNode>;

    fn concat(self, rhs: ASTNode) -> Self::Output {
        let error = format!("(ASTKind::{}(lhs), ASTKind::{}(rhs)) => {{}}", get_variant_name(&self.kind), get_variant_name(&rhs.kind));
        let out = match (self.kind, rhs.kind) {
            (ASTKind::Integer(lhs), ASTKind::Integer(rhs)) => {
              let new = lhs.mul(rhs.to_string().len()) + rhs.as_ref();
                ASTKind::Integer(Box::new(new))
            },
            (ASTKind::Integer(lhs), ASTKind::String(rhs)) => {
                ASTKind::String(lhs.to_string() + rhs.as_str())
            }
            (ASTKind::String(lhs), ASTKind::String(rhs)) => {
                ASTKind::String(lhs + rhs.as_ref())
            }
            (ASTKind::String(lhs), ASTKind::Integer(rhs)) => {
                ASTKind::String(lhs + rhs.to_string().as_ref())
            }
            _ => unimplemented!("{}", error),
        };
        Ok(ASTNode {
            kind: out,
            range: self.range
        })
    }
}

impl ASTNode {
    pub fn get_index(&self, n: &BigInt) -> Result<ASTNode> {
        match n {
            n if n > &BigInt::zero()  => {
                // TODO: Invalid Index Error
                let n = n.to_usize().unwrap_or_default() - 1 ;
                let out = match &self.kind {
                    ASTKind::List(list) => list.get(n).cloned().unwrap_or_default(),
                    ASTKind::String(string) => {
                        match string.chars().nth(n) {
                            Some(s) => ASTNode {
                                kind: ASTKind::String(s.to_string()),
                                range: self.range
                            },
                            None => ASTNode {
                                kind: ASTKind::Null,
                                range: self.range
                            },
                        }
                    },
                    ASTKind::Null => ASTNode {
                        kind: ASTKind::Null,
                        range: self.range
                    },
                    _ => unimplemented!("{:?}", self),
                };
                Ok(out)
            }
            n if n < &BigInt::zero() => {
                // TODO: Invalid Index Error
                let n = n.neg().to_usize().unwrap_or_default();
                let out = match &self.kind {
                    ASTKind::List(list) => {
                        let l = match list.len().checked_sub(n) {
                            Some(u) => {u},
                            None => {return Ok(ASTNode {
                                kind: ASTKind::Null,
                                range: self.range
                            })}
                        };
                        list.get(l).cloned().unwrap_or_default()
                    },
                    ASTKind::String(string) => {
                        let l = match string.len().checked_sub(n) {
                            Some(u) => {u},
                            None => {return Ok(ASTNode {
                                kind: ASTKind::Null,
                                range: self.range
                            })}
                        };
                        let kind = match string.chars().nth(l) {
                            Some(s) => ASTKind::String(s.to_string()),
                            None => ASTKind::Null,
                        };
                        ASTNode {
                            kind,
                            range: self.range
                        }
                    },
                    ASTKind::Null => ASTNode {
                        kind: ASTKind::Null,
                        range: self.range
                    },

                    _ => unimplemented!("{:?}", self),
                };
                Ok(out)
            }
            // n is zero
            _ => {
                Ok(self.get_type())
            }
        }
    }
    pub fn get_type(&self) -> ASTNode {
        ASTNode {
            kind: ASTKind::String(get_variant_name(&self.kind)),
            range: self.range
        }
    }
}
