use super::*;

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
