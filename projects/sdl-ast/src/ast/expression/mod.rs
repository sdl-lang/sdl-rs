use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringExpression {
    handler: Option<AST>,
    value: AST,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Number {
    Integer {
        handler: Option<String>,
        value: String,
    },
    Decimal {
        handler: Option<String>,
        value: String,
    },

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
