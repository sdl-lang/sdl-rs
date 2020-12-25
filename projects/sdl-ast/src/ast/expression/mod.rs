use super::*;
use std::ops::AddAssign;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringExpression {
    pub handler: AST,
    pub inner: Vec<AST>,
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
    pub  base: AST,
    pub  chain: Vec<AST>,
}

impl Default for CallChain {
    fn default() -> Self {
        Self {
            base: Default::default(),
            chain: vec![]
        }
    }
}

impl AddAssign<AST> for CallChain {
    fn add_assign(&mut self, rhs: AST) {
        self.chain.push(rhs)
    }
}


impl CallChain {
    pub fn new(ast: AST) -> Self {
        Self { base: ast, chain: vec![] }
    }
}

