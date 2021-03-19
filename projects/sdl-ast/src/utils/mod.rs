mod transform;

use std::fmt::Debug;

pub fn get_variant_name(e: impl Debug) -> String {
    let mut out = String::new();
    for c in format!("{:?}", e).chars() {
        if ![' ', '(', '{'].contains(&c) { out.push(c) } else { break }
    }
    return out;
}
