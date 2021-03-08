use sdl_ast::ast::{Range, Position};
use sdl_pest::{Pair, Rule};

pub struct ParserConfig {
    pub tab_size: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { tab_size: 4 }
    }
}

impl ParserConfig {
    pub fn get_position(&self, s: &Pair<Rule>) -> Range {
        let us = s.as_span().start_pos().line_col();
        let es = s.as_span().end_pos().line_col();
        Range {
            // index: s.start_pos().pos() as u64,
            start: Position { line: us.0 as u64 - 1, character: us.1 as u64 - 1 },
            end: Position { line: es.0 as u64 - 1, character: es.1 as u64 - 1 },
        }
    }
}
