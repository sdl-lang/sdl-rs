use crate::utils::get_variant_name;
use num::{ToPrimitive, Zero, BigInt};
use std::ops::{Add, Div, Mul, Sub, Neg};
use crate::{ASTNode, ASTKind, SDLError};
use crate::Result;
use crate::traits::Concat;
use bigdecimal::BigDecimal;
use lsp_types::Range;

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
        let out = match (&self.kind, rhs.kind) {
            (ASTKind::Integer(lhs), ASTKind::Integer(rhs)) => {
              let new = lhs.mul(rhs.to_string().len()) + rhs.as_ref();
                ASTKind::Integer(Box::new(new))
            },
            (ASTKind::Integer(lhs), ASTKind::String(rhs)) => {
                ASTKind::String(lhs.to_string() + rhs.as_str())
            }
            (ASTKind::String(lhs), ASTKind::String(rhs)) => {
                ASTKind::String(lhs.to_string() + rhs.as_ref())
            }
            (ASTKind::String(lhs), ASTKind::Integer(rhs)) => {
                ASTKind::String(lhs.to_string() + rhs.to_string().as_ref())
            }
            (_, ASTKind::List(rhs)) => {
                let mut new = vec![self.to_owned()];
                new.extend(rhs);
                ASTKind::List(new)
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
    pub fn get_index(&self, n: &BigInt, p: Range) -> Result<ASTNode> {
        match n {
            n if n > &BigInt::zero()  => {
                // TODO: Invalid Index Error
                let n = n.to_usize().unwrap_or_default() - 1 ;
                let kind = match &self.kind {
                    ASTKind::List(list) => {
                        match list.get(n) {
                            Some(s) => {s.kind.to_owned()},
                            None => ASTKind::Null,
                        }
                    },
                    ASTKind::String(string) => {
                        match string.chars().nth(n) {
                            Some(s) => ASTKind::String(s.to_string()),
                            None => ASTKind::Null,
                        }
                    },
                    _ => {
                        return Err(SDLError::invalid_index(
                             n.add(1).to_string(),
                             get_variant_name(&self.kind),
                          p
                        ))
                    }
                };
                Ok(ASTNode {
                    kind,
                    range: self.range
                })
            }
            n if n < &BigInt::zero() => {
                // TODO: Invalid Index Error
                let n = n.neg().to_usize().unwrap_or_default();
                let kind = match &self.kind {
                    ASTKind::List(list) => {
                        let l = match list.len().checked_sub(n) {
                            Some(u) => {u},
                            None => {return Ok(ASTNode {
                                kind: ASTKind::Null,
                                range: self.range
                            })}
                        };
                        match list.get(l) {
                            Some(s) => {s.kind.to_owned()},
                            None => ASTKind::Null,
                        }
                    },
                    ASTKind::String(string) => {
                        let l = match string.len().checked_sub(n) {
                            Some(u) => {u},
                            None => {return Ok(ASTNode {
                                kind: ASTKind::Null,
                                range: self.range
                            })}
                        };
                        match string.chars().nth(l) {
                            Some(s) => ASTKind::String(s.to_string()),
                            None => ASTKind::Null,
                        }
                    },
                    _ => {
                        return Err(SDLError::invalid_index(
                            n.to_string(),
                            get_variant_name(&self.kind),
                            p
                        ))
                    }
                };
                Ok(ASTNode {
                    kind,
                    range: self.range
                })
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
