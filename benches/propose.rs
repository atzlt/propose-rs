#![allow(unused_must_use)]

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

config color=\"grey\";
draw AP, BQ, CR;

config color=\"red\";
draw @(MNL);";

const TRISECT: &str = "
A = (4 : 40deg);
O = (0 : 0);
B = (4 : 0);
x = <AOB;
x = $ x / 3 $;
C = rot B, O, x;
D = rot B, O, $ 2 * x $;
draw O, OA, OB, OC[color=\"red\"], OD[color=\"red\"];";

const PROBLEM1: &str = "
O = (0, 0);
r = 4;
c = @(O, r);
C = (r : -20deg);
A = (r : 100deg);
B = (r : 200deg);
D = i AO, c, A;
x = <BAC;
x = $ -180 * deg + 2 * x $;
E = on A, B, 0.7;
F = rot E, D, x;
F = i DF, AC;
l = ab E, D, F;
G = i l, EF;
draw c, A, B, C, D, E, F, G, AB, BC, CA, AD, DE, DF, EF, DG;

config decorcolor=\"blue\";
K = i DG, BC;
decor DK: |, KG: |;";

pub fn interpreter_bench(c: &mut Criterion) {
    let mut interpreter = InterpreterState::new();
    c.bench_function("Interpreter - Reim's Thm", |b| {
        b.iter(|| {
            black_box(interpreter.interpret(REIM));
            interpreter.clear()
        })
    });
    c.bench_function("Interpreter - Ninepoint", |b| {
        b.iter(|| {
            black_box(interpreter.interpret(NINEPOINT));
            interpreter.clear()
        })
    });
    c.bench_function("Interpreter - Trisect", |b| {
        b.iter(|| {
            black_box(interpreter.interpret(TRISECT));
            interpreter.clear()
        })
    });
    c.bench_function("Interpreter - Problem 1", |b| {
        b.iter(|| {
            black_box(interpreter.interpret(PROBLEM1));
            interpreter.clear()
        })
    });
}

criterion_group!(interpreter, interpreter_bench);
criterion_main!(interpreter);
