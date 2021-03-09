use super::*;


macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let out = render(include_str!(concat!($file_name, ".sdl"))).unwrap();
        assert_eq!(include_str!(concat!($file_name, ".out.sdl")), out)
    }
    };
}

run_test![
    for_i_in_list,
    for_i_in_string,
    for_if_guard,
];


const FOR_ELSE_GUARD: &'static str = r#"
for i in [ ] {
    false
}
else {
    true
}
"#;

#[test]
fn for_else_guard() {
    assert_eq!(render(FOR_ELSE_GUARD).unwrap(), "true")
}

const FOR_IF_ELSE_GUARD: &'static str = r#"
for i in [1, 2, 3] if x > 5 {
    false
}
else {
    true
}
"#;

#[test]
fn for_if_else_guard() {
    assert_eq!(render(FOR_IF_ELSE_GUARD).unwrap(), "true")
}
