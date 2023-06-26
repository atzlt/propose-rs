use crate::interpreter::parser::parse;

#[test]
fn test_interpreter() {
    insta::with_settings!({sort_maps => true}, {
        insta::glob!("../../../test_input", "*.prs", |path| {
            let input = std::fs::read_to_string(path).unwrap();
            insta::assert_ron_snapshot!(parse(&input).unwrap());
        })
    })
}
