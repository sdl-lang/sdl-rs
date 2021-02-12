use super::*;
use std::ops::AddAssign;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringExpression {
    pub handler: Option<ASTNode>,
    pub inner: Vec<ASTNode>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InfixExpression {
    pub op: ASTNode,
    pub lhs: ASTNode,
    pub rhs: ASTNode,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnaryExpression {
    pub op: ASTNode,
    pub base: ASTNode,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CallChain {
    pub base: ASTNode,
    pub chain: Vec<ASTNode>,
}

impl Default for CallChain {
    fn default() -> Self {
        Self { base: Default::default(), chain: vec![] }
    }
}

impl AddAssign<ASTNode> for CallChain {
    fn add_assign(&mut self, rhs: ASTNode) {
        self.chain.push(rhs)
    }
}

impl CallChain {
    pub fn new(ast: ASTNode) -> Self {
        Self { base: ast, chain: vec![] }
    }
}
