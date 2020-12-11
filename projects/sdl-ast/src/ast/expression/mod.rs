use super::*;
use std::ops::{AddAssign};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringExpression {
    pub handler: AST,
    pub value: AST,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InfixExpression {
    pub op: AST,
    pub lhs: AST,
    pub rhs: AST,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryExpression {
    pub op: AST,
    pub base: AST,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CallChain {
    pub(crate)   base: AST,
    pub(crate)   chain: Vec<AST>,
}

impl CallChain {
    pub fn new(ast: AST) -> Self {
        Self {
            base: ast,
            chain: vec![]
        }
    }
}

impl AddAssign<AST> for CallChain {
    fn add_assign(&mut self, rhs: AST) {
        self.chain.push(rhs)
    }
}