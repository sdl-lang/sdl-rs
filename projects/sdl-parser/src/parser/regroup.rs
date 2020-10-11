use std::lazy::SyncLazy;
use sdl_pest::{PrecClimber, Rule, Operator};
use sdl_pest::Assoc::{Left,Right};

#[rustfmt::skip]
pub static PREC_CLIMBER: SyncLazy<PrecClimber<Rule>> = SyncLazy::new(|| {
    use Rule::*;
    //TODO: use macro
    PrecClimber::new(vec![
        Operator::new(Set, Left),
        Operator::new(Is, Left) | Operator::new(IsNot, Left),
        Operator::new(Plus, Left) | Operator::new(Minus, Left),
        Operator::new(Power, Right),
        Operator::new(Dot, Left)
    ])
});