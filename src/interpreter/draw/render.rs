use crate::{
    interpreter::{
        draw::CM,
        parser::ast::Config,
        structs::{Arc, Segment},
        utils::{ConfigValue, DObject},
    },
    write_circle, write_line, write_polygon, write_arc,
};
use if_chain::if_chain;
use itertools::Itertools;
use metric_rs::calc::{construct::center, point_on::PointOn, basic::Distance};
use metric_rs::objects::Point;
use std::f64::consts::PI;
use std::fmt::Display;

#[derive(Debug)]
pub struct StyledDObject<'conf> {
    pub obj: DObject,
    pub local_conf: Option<Config>,
    pub global_conf: &'conf Config,
}

impl StyledDObject<'_> {
    #[inline]
    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        let key = key.to_string();
        if_chain! {
            if let Some(conf) = &self.local_conf;
            if let Some(value) = conf.get(&key);
            then { Some(value) }
            else { self.global_conf.get(&key) }
        }
    }
    /// times a config is already present in global config, so no need to check it.
    #[inline]
    pub(super) fn get_unchecked(&self, key: &str) -> &ConfigValue {
        let key = key.to_string();
        if_chain! {
            if let Some(conf) = &self.local_conf;
            if let Some(value) = conf.get(&key);
            then { value }
            else { self.global_conf.get(&key).unwrap() }
        }
    }
}

impl Display for StyledDObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dash = if let Some(val) = self.get("dash") {
            format!(" stroke-dasharray=\"{}\"", val)
        } else {
            String::new()
        };
        match &self.obj {
            DObject::Segment(seg) => {
                let Segment { from: a, to: b } = seg;
                write_line!(
                    f,
                    a,
                    b,
                    self.get_unchecked("color"),
                    self.get_unchecked("linewidth"),
                    dash
                )
            }
            DObject::Circle(circ) => {
                write_circle!(
                    f,
                    circ.O,
                    circ.r,
                    self.get_unchecked("color"),
                    self.get_unchecked("fill"),
                    self.get_unchecked("linewidth"),
                    dash
                )
            }
            DObject::Point(p) => {
                write_circle!(
                    f,
                    p,
                    self.get_unchecked("dotsize") => in px,
                    self.get_unchecked("dotstroke"),
                    self.get_unchecked("dotfill"),
                    self.get_unchecked("dotwidth"),
                    ""
                )
            }
            DObject::Arc(arc) => {
                write_arc!(
                    f,
                    arc.from,
                    arc.r,
                    arc.large_arc,
                    arc.sweep,
                    arc.to,
                    self.get_unchecked("color"),
                    self.get_unchecked("linewidth"),
                    dash
                )
            }
            DObject::Polygon(poly) => {
                let pts = poly
                    .iter()
                    .map(|p| format!("{},{}", p.x * CM, -p.y * CM))
                    .join(" ");
                write_polygon!(
                    f,
                    pts,
                    self.get_unchecked("fill")
                )
            }
            // todo: Error handling in this (very special) branch
            // todo: Draw right angle instead when `AOB` is an right angle
            DObject::Angle3P(a, o, b) => {
                let anglesize = self.get_unchecked("anglesize").try_into_f64().unwrap();
                let a = *a * CM;
                let o = *o * CM;
                let b = *b * CM;
                let dist = a.distance(o);
                let a = o + (a - o) * (anglesize / dist);
                let arc = Arc::from_center(a, o, b).unwrap();
                write_arc!(
                    f,
                    in px:
                    arc.from,
                    arc.r,
                    arc.large_arc,
                    arc.sweep,
                    arc.to,
                    self.get_unchecked("anglecolor"),
                    self.get_unchecked("anglewidth"),
                    dash
                )
            },
        }
    }
}

impl StyledDObject<'_> {
    pub(super) fn get_position(&self, loc: f64) -> Point {
        match &self.obj {
            DObject::Point(p) => *p,
            DObject::Circle(c) => c.point_on(loc),
            DObject::Arc(arc) => arc.point_on(loc),
            DObject::Segment(seg) => seg.point_on(loc),
            DObject::Polygon(poly) => center(poly),
            DObject::Angle3P(a, o, b) => {
                Arc::from_center(*a, *o, *b).unwrap().point_on(loc) // todo: Error handling
            },
        }
    }

    /// Get the angle of the tangent line at a certain point.
    pub(super) fn get_tan_angle(&self, loc: f64) -> f64 {
        match &self.obj {
            DObject::Segment(seg) => {
                let Segment { from, to } = seg;
                (from.y - to.y).atan2(from.x - to.x)
            }
            DObject::Circle(_) => -(loc + 0.25) * PI * 2.0,
            DObject::Arc(arc) => {
                let Arc { from, to, O, .. } = arc;
                let start = (O.x - from.x).atan2(from.y - O.y);
                let end = (O.x - to.x).atan2(to.y - O.y);
                loc * end + (1.0 - loc) * start
            }
            DObject::Angle3P(a, o, b) => {
                let start = (o.x - a.x).atan2(a.y - o.y);
                let end = (o.x - b.x).atan2(b.y - o.y);
                loc * end + (1.0 - loc) * start
            }
            DObject::Polygon(_) | DObject::Point(_) => 0.0,
        }
    }
}
