use propose::interpreter::interpret::InterpreterState;

fn main() {
    let mut interpreter = InterpreterState::new();
    interpreter
        .interpret(
            "
config width=14, height=10, dotsize=7, dotstroke=\"red\", dotwidth=1;

O1 = (-3, 0);
O2 = (2, 0);
c = @(O1, 3);
d = @(O2, 4);
A, B = i c, d;
l = l 0.1, 1, A;
S, _ = i l, c, A;
T, _ = i l, d, A;
k = l -0.2, 1, B;
P, _ = i k, c, B;
Q, _ = i k, d, B;

draw c, d, A, B, S, T, ST, P, Q, PQ;            
config color=\"red\";
draw SP, TQ;
config decorcolor=\"blue\";
decor SP: >, TQ: >;
save out.svg;",
        )
        .unwrap();
    dbg!(interpreter);
}
