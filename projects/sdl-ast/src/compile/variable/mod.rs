use super::*;

#[derive(Clone, Debug)]
pub enum Variable {
    Constant(ASTNode),
    Variable(ASTNode),
    Delay(ASTNode),
    Lazy(ASTNode),
}

impl Variable {
    pub fn get(&self) -> ASTNode {
        match self {
            Variable::Constant(v) => v.to_owned(),
            Variable::Variable(v) => v.to_owned(),
            Variable::Delay(v) => v.to_owned(),
            Variable::Lazy(v) => v.to_owned(),
        }
    }
}
