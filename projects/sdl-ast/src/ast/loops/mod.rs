use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ForInLoop {
    pub pattern: AST,
    pub terms: AST,
    pub block: AST,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfElseChain {
    pub pairs: Vec<(AST,AST)>,
    pub cover: Option<AST>,
}

impl IfElseChain {
    pub fn build(cds: Vec<AST>, acts: Vec<AST>)->Self {
        let align = cds.len()== acts.len()
        let cover = acts.last().unwrap();
        let pairs = cds.into_iter().zip(acts.into_iter()).collect();
        if align {
            Self {
                pairs,
                cover: None
            }
        }
        else {
            Self {
                pairs,
                cover:Some(cover.clone()),
            }
        }
    }
}