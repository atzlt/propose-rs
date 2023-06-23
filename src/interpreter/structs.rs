// Provides additional structures than `metric-rs`, like
// `Segment` and `Arc`.
#![allow(non_snake_case)]

use std::f64::consts::PI;

use metric_rs::{
    calc::{
        basic::{angle, Distance},
        exception::Result as CalcResult,
        point_on::PointOn,
        transform::Rotate,
    },
    objects::{Circle, Point},
};

#[derive(Debug, Clone)]
pub struct Segment {
    pub from: Point,
    pub to: Point,
}

#[derive(Debug, Clone)]
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
        let angle = angle(A, O, C)?;
        let angle = if large_arc { 2.0 * PI - angle } else { angle };
        let angle = if sweep { angle } else { -angle };
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
    pub fn from_center(A: Point, O: Point, B: Point) -> CalcResult<Self> {
        let Point { x: x1, y: y1 } = O - A;
        let Point { x: x2, y: y2 } = B - O;
        let r = O.distance(A);
        let large_arc = x1 * y2 < x2 * y1;
        let angle = angle(A, O, B)?;
        let angle = if large_arc { 2.0 * PI - angle } else { angle };
        let angle = -angle;
        Ok(Arc {
            from: A,
            to: B,
            O,
            r,
            sweep: false,
            large_arc,
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
    fn point_on(&self, pos: f64) -> Point {
        self.to * pos + self.from * (1.0 - pos)
    }
}

impl PointOn for Arc {
    #[inline]
    fn point_on(&self, pos: f64) -> Point {
        let angle = pos * self.angle;
        self.from.rotate(self.O, angle)
    }
}
