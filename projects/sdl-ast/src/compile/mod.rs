mod value;
mod variable;

use crate::{
    traits::{Evaluate, Render},
    Result, AST,
};
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};
pub use value::{HTMLElement, Value};
pub use variable::Variable;

#[derive(Clone, Debug)]
pub struct SDLContext {
    config: Option<Box<SDLContextConfig>>,
    father: Option<Weak<SDLContext>>,
    variables: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct SDLContextConfig {
    pub is_debug: bool,
}

impl Default for SDLContext {
    fn default() -> Self {
        Self { config: None, father: None, variables: Default::default() }
    }
}

impl Default for SDLContextConfig {
    fn default() -> Self {
        Self { is_debug: false }
    }
}

impl SDLContext {
    pub fn evaluate(&mut self, code: &AST) -> Result<Value> {
        code.evaluate(self)
    }
    pub fn render(&mut self, code: &Value) -> Result<String> {
        let mut output = String::new();
        code.render(&mut output, &*self)?;
        Ok(output)
    }

    pub fn insert(&mut self, key: &str, v: impl Into<Value>) {
        self.variables.insert(key.to_string(), v.into());
    }
    pub fn get(&mut self, key: &str) -> Value {
        self.variables.get(key).cloned().unwrap_or_default()
    }

    pub fn config(&self) -> SDLContextConfig {
        match &self.config {
            None => Default::default(),
            Some(x) => *x.clone(),
        }
    }

    pub fn fork(&self) -> SDLContext {
        let new = Rc::downgrade(&Rc::new(self.to_owned()));
        SDLContext { config: None, father: Some(new), variables: Default::default() }
    }
}

impl SDLContext {
    // pub fn get_value(&self, name: &str) -> Value {
    //     match self.variables.get(name) {
    //         Some(v) => v.get(),
    //         None => self.father.as_ref().and_then(|ctx| ctx.upgrade()).map(|ctx| ctx.get_value(name)).unwrap_or_default(),
    //     }
    // }
}
