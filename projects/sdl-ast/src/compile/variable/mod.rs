use super::*;

#[derive(Clone,Debug)]
pub enum Variable {
    Constant(Value),
    Variable(Value),
    Delay,
    Lazy
}

impl Variable {
    pub fn get(&self) -> Value {
        match self {
            Variable::Constant(v) => { v.clone() }
            Variable::Variable(v) => { v.clone() }
            Variable::Delay => { Default::default() }
            Variable::Lazy => { Default::default() }
        }
    }
}