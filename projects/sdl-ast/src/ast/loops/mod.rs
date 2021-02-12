use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForInLoop {
    pub pattern: ASTNode,
    pub terms: ASTNode,
    pub guard: Option<ASTNode>,
    pub block: ASTNode,
    pub for_else: Option<ASTNode>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfElseChain {
    pub pairs: Vec<(ASTNode, ASTNode)>,
    pub cover: Option<ASTNode>,
}

impl IfElseChain {
    pub fn build(cds: Vec<ASTNode>, acts: Vec<ASTNode>) -> Self {
        let cover = match cds.len() == acts.len() {
            true => None,
            false => acts.last().cloned(),
        };
        let pairs = cds.into_iter().zip(acts.into_iter()).collect();
        Self { pairs, cover }
    }
}
