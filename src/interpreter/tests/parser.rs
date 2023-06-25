use crate::interpreter::parser::parse;

const TEST_STR: [&str; 13] = [
    "config width=14, height=10, dotsize=7, dotstroke=\"red\", dotwidth=1;",
    "O2 = (2, 0);",
    "d = @(O2, 4);",
    "A, B = i c, d;",
    "l = l 0.1, 1, A;",
    "S, _ = i l, c, A;",
    "k = l -0.2, 1, B;",
    "draw c, d, A, B, S, T, ST, P, Q, PQ;",
    "decor SP: >, TQ: >;",
    "A = (3 : -40deg);",
    "draw c[label=\"c\",loc=160deg,angle=160deg], A[label=\"A\",angle=-60deg], B[label=\"B\",angle=50deg];",
    "draw CS[color=\"blue\",dash=5], BS, DS;",
    "draw A-B-C-D[fill=\"#ff000033\"];"
];

#[test]
fn test_parser() {
    insta::with_settings!({sort_maps => true}, {
        for string in TEST_STR {
            insta::assert_yaml_snapshot!(parse(string).unwrap());
        }
    })
}
