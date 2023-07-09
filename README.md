# Propose

Propose is a tool to generate geometric figures (SVG format) from text files.

For a usage guide see [Usage](./Usage.md).

## Example: Harmonic Quadrilateral

```
config height=10, width=13, min-x=-4;

O = (0, 0);
c = @(O, 3);
A = (3 : -40deg);
B = (3 : 50deg);
C = (3 : 220deg);
l, _ = tan A, c;
k, _ = tan C, c;
T = i l, k;
D, _ = i TB, c, B;
l, _ = tan B, c;
k, _ = tan D, c;
S = i l, k;

draw c[label="c",loc=160deg,angle=160deg], A[label="A",angle=-60deg], B[label="B",angle=50deg];
draw C[label="C",angle=220deg,dist=13];
draw D[label="D",angle=-65deg,dist=13];
draw AB, BC, CD, DA, T, AT, CT, BT, S;
draw CS[color="blue",dash=5], BS, DS;
draw A-B-C-D[fill="#ff000033"];
```

## Features

**_Warning: currently this project uses `meval` for evaluating expressions, but this dependency is using deprecated Rust features, which will be rejected by future Rust versions._** I don't have time to solve this problem recently, sorry.

- [x] Drawing with various styles
- [ ] REPL mode
- [x] Evaluate math expressions
- [x] Labels
- [x] Decoration

### Future Plans

- [ ] Draw infinite lines
