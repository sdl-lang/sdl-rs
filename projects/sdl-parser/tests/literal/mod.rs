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
    comment,
    number,
    string, string_escape,
    template_escape,
];
