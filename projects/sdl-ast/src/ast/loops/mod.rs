use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForInLoop {
    pub pattern: AST,
    pub terms: AST,
    pub block: AST,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfElseChain {
    pub pairs: Vec<(AST, AST)>,
    pub cover: Option<AST>,
}

impl IfElseChain {
    pub fn build(cds: Vec<AST>, acts: Vec<AST>) -> Self {
        let cover = match cds.len() == acts.len() {
            true => None,
            false => acts.last().cloned(),
        };
        let pairs = cds.into_iter().zip(acts.into_iter()).collect();
        Self { pairs, cover }
    }
}
