// Provides additional structures than `metric-rs`, like
// `Segment` and `Arc`.
#![allow(non_snake_case)]

use std::f64::consts::PI;

use metric_rs::{
    calc::{basic::angle, exception::Result as CalcResult, point_on::PointOn},
    objects::{Circle, Point},
};

pub struct Segment {
    pub from: Point,
    pub to: Point,
}

pub struct Arc {
    pub from: Point,
    pub to: Point,
    pub O: Point,
    pub r: f64,
    pub sweep: bool,
    pub large_arc: bool,
    pub angle: f64,
}

impl Arc {
    pub fn from_3p(A: Point, B: Point, C: Point) -> CalcResult<Self> {
        let Circle { O, r } = Circle::from_3p(A, B, C)?;
        let Point { x: x1, y: y1 } = B - A;
        let Point { x: x2, y: y2 } = C - B;
        let large_arc = x1 * x2 + y1 * y2 < 0.0;
        let sweep = x1 * y2 > x2 * y1;
        let mut angle = angle(A, O, C)?;
        angle = if large_arc { 2.0 * PI - angle } else { angle };
        angle = if sweep { angle } else { -angle };
        Ok(Arc {
            from: A,
            to: C,
            O,
            r,
            large_arc,
            sweep,
            angle,
        })
    }
}

impl Segment {
    pub fn new(A: Point, B: Point) -> Self {
        Segment { from: A, to: B }
    }
}

impl PointOn for Segment {
    #[inline]
    fn point_on(self, pos: f64) -> Point {
        self.to * pos + self.from * (1.0 - pos)
    }
}
