use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

pub fn gen_note_down() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./sdl.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./src/sdl"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct SDLParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct SDLParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}
