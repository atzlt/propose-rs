# Introduction

Propose is a tool to generate geometric figures from text files. The source format is easy to learn and straightforward to understand.

I created this tool because I can't find an appropriate tool to generate SVG format images for geometric figures on my math textbooks. If you have similar needs you might find this useful.

This small project is still being developed.

# Propose Syntax

A Propose file consists of **lines**. A line can either be

1. a declaration,
2. a configuration, or
3. a drawing instruction.

## Declaration

The simpliest declaration is defining a point using coordinates:

```
A = (1, 5);
B = (6 : 70deg);
```

If two values are separated using comma `,` then this is the rectangular coordinate. If separated by colon `:`, then this is the polar coordinate.

A more complex declaration looks like this:

```
A = i XY, PQ;
```

Before the equal sign `=` is the **target**, the name to be assigned to. In this case it's the name `A`. After that comes the **method**. A method is a pre-defined function; in this case it's `i`, which means _intersection_. After the method is a comma-separated list of **arguments**.

### Identifiers

Notice that the name of a point should always start with a capital letter, and the name for every other thing (a _common_ identifier) should start with a lowercase letter; the following letters should be lowercase letters, underscore or an apostrophe `'`.

Examples of valid point identifiers: `P`, `I1`, `S'`, `Oa3`, `Na1'`.

Examples of valid common identifiers: `l`, `c`, `omega'`, `k4`.

### Target

There are two kinds of declaration: direct and destruct.

Direct declaration assigns the returned value to the single target provided, and destruct declaration assigns the first returned value two the first target, and the second value two the second target. For example, `i` (intersection) returns two points for a line and circle intersection, so we can do this:

```
S, T = i AB, (O, r);
```

If you want to discard one of the value, use an underscore:

```
_, T = i AB, (O, r);
```

Every identifier can be re-assigned, but re-assigning one identifier _does not_ update the objects depending on it.

### Arguments

There are many types of arguments:

1. Identifier; its corresponding value is passed to the method.
2. Line through 2 points: `XY` means the line passing through `X` and `Y`. **There should not be any whitespaces.**
3. Circle with center and radius: `(O, r)` means the circle with center `O` and radius `r`, where `r` can be either a number literal or a common identifier representing a number.
4. Circle with center and point: `(O, A)` means the circle with center `O` and passes through `A`.
5. Circle with diameter: `(O A)` means the circle with center `O` and passes through `A`.
6. Circle throught 3 points: `(A B C)` means the circle passing through `A`, `B` and `C`. Whitespaces are not required: `(ABC)` is equivalent to `(A B C)`.
7. A number: either a number literal like `1.3` or an angle in degree like `20deg`. A number literal can also be passed as an angle, but in radians.
8. A triangle: `ABC` means triangle `ABC`. **There should not be any whitespaces.**
9. A math expression.

### Expression Evaluation

You can also use an expression as the right-hand-side value.

```
x = $ sqrt(2) $
y = $ x + 4 $
```

## Configuration

```
config conf1=val1, conf1=val2, ...
```

There are multiple configurations available. Changing them only affects the lines after the change.

The value can be a string, a number or a number with `deg` suffix.

1. `width, height` The width and height of the output image (in cm). Default: both `10`.
2. `min-x, min-y` The `minX` and `minY` attributes of the `viewBox`. Default: if not set then `min-x = -width / 2`, `min-y = -height / 2`.
3. `color, fill, linewidth, dotsize` Very straightfoward. Default: `#000000`, `#00000000`, `1.5`, `2.5`.
4. `dash` Dash line style, set [the `stroke-dasharray` attribute](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-dasharray). Default: empty.
5. `loc, angle, dist` The default location, angle and distance for labels Default: `0`, `0`, `10`.
6. `labelsize` The default size of the label.
7. `font` The font family of the label.

## Drawing

```
draw A, B, c[label="c",loc=40deg], PQ[color="blue"], ...
```

Draw some objects. **Currently does not support drawing (infinite) lines.**

You can inline some temporary configurations in square brackets `[]`. The configurations only work for the current object. For all configurations see the previous section. There's only one additional configuration: `label`, which is the text to be labelled on this object.

You can also fill a polygon:

```
draw A-B-C-D-E[fill="#ff000033"];
```

or an 3-point arc:

```
draw A~B~C[dash=5];
```

or an angle:

```
draw <AOB[anglecolor="red"];
```

The unit length is 1 centimeter. The y-axis of SVG coordinate system is flipped, so a point `(a, b)` is actually rendered as `x="a cm", y="-b cm"` in the output.

### Units

Normally everything you draw is using `cm` as unit length, **_except for_** `dist` (distance of label), `dotsize`, `labelsize`, `dash`, or other configurations involving only _small distances_.

### Labelling

You can control the position of the label by setting `loc`, `angle`, `dist`.

`loc` controls the location of the label. On segments and arcs, this is the ratio between two parts (`0` is at the start, `1` at the end, allows negative values and values larger than `1`). On circles this is the angle (start from x-axis, counter-clockwise).

`angle` controls the angle of the label. The label is on a circle with center `loc` and radius `dist`. `angle` controls its exact position on this circle (start from x-axis, counter-clockwise).

`dist` is the distance between the label and `loc`.

# Using the CLI

CLI is currently very simple. You provide and output by `-o` (if not present, the output path will be your input path with extension `.svg`). For example, `propose test_input/incenter.prs` saves the output to `test_input/incenter.svg`.

# Appendix: List of Methods

- `i` intersection. If a third argument is given, this should be one of the common points, and the another intersection will be placed at the first returned value.
- `perp` perpendicular. `perp <point>, <line>`
- `par` parallel. `par <point>, <line>`
- `proj` projection. `proj <point>, <line>`
- `pb` perpendicular bisector. `pb <point>, <point>`
- `ab` angle bisector. `ab <point>, <point>, <point>` (interior angle bisector first, exterior second) or `<- <line>, <line>`.
- `tan` tangent line. `tan <point>, <circle>`
- `outer-tan` outer common tangents.
- `inner-tan` inner common tangents.
- `mid` midpoint. `mid <point>, <point>`
- `rad-ax` radical axis.
- `polar` polar line. `polar <point>, <circle>`

---

- `l` constructs a line. `l <point>, <point>` Define a line using two points. `l <a=number>, <b=number>, <point>` Define a line `ax+by+c=0` that passes through a point. `l <a=number>, <b=number>, <c=number>` Define a line `ax+by+c=0`.
- `circ` destructs a circle into `<center>, <radius>` pair.

---

- `rfl` reflection in another object. When reflecting in a circle, this means inversion.
- `inv` inversion. `inv <object>, <center=point> <power=number>` (`power` can be negative.)
- `rot` rotation. `rot <object>, <center=point> <angle=number>`
- `scl` scale. `scl <object> <center=point>, <ratio=number>` (`ratio` can be nagative.)

---

## Centers

These methods all accept a triangle as the single input.

- `cO` circumcenter
- `cI` incenter
- `cJ` excenter **in angle `A`**.
- `cH` orthocenter
- `cG` centroid
- `cGe` Gergonne point
- `cK` symmedian

And some other functions related to triangles:

- `bary` From barycentrics. `bary <triangle>, <number>, <number>, <number>`
- `isog-conj` Isogonal Conjugate. `isog-conj <triangle>, <point>`
