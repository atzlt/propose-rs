use crate::interpreter::interpret::InterpreterState;

#[test]
fn test_interpreter() {
    insta::with_settings!({sort_maps => true}, {
        insta::glob!("../../../test_input", "*.prs", |path| {
            let input = std::fs::read_to_string(path).unwrap();
            let mut interpreter = InterpreterState::new();
            interpreter.interpret(&input).unwrap();
            insta::assert_yaml_snapshot!(interpreter);
        })
    })
}
