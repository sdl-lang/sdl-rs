mod variable;
mod value;

use std::collections::HashMap;
pub use variable::{Variable};
pub use value::Value;
use std::rc::Weak;
use crate::AST;

#[derive(Clone,Debug)]
pub struct Context {
    father: Option<Weak<Context>>,
    variables: HashMap<String, Variable>
}

impl Default for Context {
    fn default() -> Self {
        Self {
            father: None,
            variables: Default::default()
        }
    }
}

impl Context {
    pub fn get_value(&self, name:&str) -> Value {
        self.variables.get(name).map(|v|v.get()).unwrap_or_default()
    }
}