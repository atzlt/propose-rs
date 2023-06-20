use propose::interpreter::interpret::InterpreterState;

fn main() {
    let mut interpreter = InterpreterState::new();
    interpreter
        .interpret(
            "
A = (2, 5);
B = (3, 4);
l = AB;
C = (1, 0);
D = (4, 7);
k = CD;
P = i l, k;
draw AB, CD, P[label=\"P\"];
    ",
        )
        .unwrap();
    dbg!(interpreter);
}
