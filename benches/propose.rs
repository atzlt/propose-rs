use criterion::{black_box, criterion_group, criterion_main, Criterion};
use propose::interpreter::interpret::InterpreterState;

const REIM: &str = "
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
decor SP: >, TQ: >;";

const NINEPOINT: &str = "
A = (-3, -2);
B = (3, -2);
C = (1, 2);
M = mid A, B;
N = mid B, C;
L = mid C, A;
P = proj A, BC;
Q = proj B, CA;
R = proj C, AB;

draw A, B, C, AB, BC, CA, M, N, L, P, Q, R;

config color=\"grey\"
draw AP, BQ, CR;

config color=\"red\";
draw @(MNL);";

pub fn interpreter_bench(c: &mut Criterion) {
    c.bench_function("Interpreter - Reim's Thm", |b| {
        b.iter(|| {
            let mut interpreter = InterpreterState::new();
            black_box(interpreter.interpret(REIM))
        })
    });
    c.bench_function("Interpreter - Ninepoint", |b| {
        b.iter(|| {
            let mut interpreter = InterpreterState::new();
            black_box(interpreter.interpret(NINEPOINT))
        })
    });
    c.bench_function("Interpreter - Reim's Thm with Clear", |b| {
        b.iter(|| {
            let mut interpreter = InterpreterState::new();
            black_box(interpreter.interpret(REIM).unwrap());
            interpreter.clear()
        })
    });
}

criterion_group!(interpreter, interpreter_bench);
criterion_main!(interpreter);
