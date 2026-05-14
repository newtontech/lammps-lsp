use lammps_analyser::input_script::{self, InputScript};

#[test]
fn double_str() {
    let source = include_str!("./fixtures/string/in.double_string");

    let result = InputScript::new(source).expect("valid input script");
    insta::assert_debug_snapshot!(result.ast);
    insta::assert_debug_snapshot!(result.diagnostics);
}
