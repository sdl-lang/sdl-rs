use super::*;

use std::ops::Index;
use num::ToPrimitive;

impl Index<usize> for Value {
    type Output = Result<Value>;

    fn index(&self, n: usize) -> &Self::Output {
        let out = match self {
            Value::List(list) => {
                list.get(n).cloned().unwrap_or_default()
            },
            _ => unimplemented!()
        };
        &Ok(out)
    }
}

impl Index<&BigInt> for Value {
    type Output = Result<Value>;

    fn index(&self, n: &BigInt) -> &Self::Output {
        let n = n.to_usize().unwrap_or_default();
        self.index(n)
    }
}

