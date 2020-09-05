use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForInLoop {
    pub pattern: AST,
    pub terms: AST,
    pub block: AST,
}
