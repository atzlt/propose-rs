use crate::structs::{Arc, Segment};
use crate::{
    interpreter::{
        ast::{Config, ConfigValue},
        draw::CM,
        utils::DObject,
    },
    write_circle, write_line,
};
use if_chain::if_chain;
use itertools::Itertools;
use metric_rs::calc::point_on::PointOn;
use metric_rs::objects::Point;
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
    /// Sometimes a config is already present in global config, so no need to check it.
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
        match &self.obj {
            DObject::Segment(seg) => {
                let Segment { from: a, to: b } = seg;
                write_line!(
                    f,
                    a,
                    b,
                    self.get_unchecked("color"),
                    self.get_unchecked("linewidth"),
                    if_chain! {
                        if let Some(ConfigValue::String(s)) = self.get("dash");
                        if !s.is_empty();
                        then { format!(" stroke-dasharray={}", s) }
                        else { String::new() }
                    }
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
                    if_chain! {
                        if let Some(ConfigValue::String(s)) = self.get("dash");
                        if !s.is_empty();
                        then { format!(" stroke-dasharray={}", s) }
                        else { String::new() }
                    }
                )
            }
            DObject::Point(p) => {
                write_circle!(
                    f,
                    p,
                    self.get_unchecked("dotsize"),
                    self.get_unchecked("dotstroke"),
                    self.get_unchecked("dotfill"),
                    self.get_unchecked("dotwidth"),
                    ""
                )
            }
            DObject::Arc(arc) => {
                let Arc {
                    from,
                    to,
                    O: _,
                    r,
                    sweep,
                    large_arc,
                    angle: _,
                } = arc;
                write!(
                    f,
                    "<path d=\"M {},{} A {} {} 0 {} {} {},{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
                    from.x * CM,
                    -from.y * CM,
                    r * CM,
                    r * CM,
                    if *large_arc { 1 } else { 0 },
                    if *sweep { 0 } else { 1 },
                    to.x * CM,
                    -to.y * CM,
                    self.get_unchecked("color"),
                    self.get_unchecked("linewidth"),
                    if_chain! {
                        if let Some(ConfigValue::String(s)) = self.get("dash");
                        if !s.is_empty();
                        then { format!(" stroke-dasharray={}", s) }
                        else { String::new() }
                    }
                )
            }
            DObject::Polygon(poly) => {
                let pts = poly
                    .iter()
                    .map(|p| format!("{},{}", p.x * CM, p.y * CM))
                    .join(" ");
                write!(
                    f,
                    "<polygon points=\"{}\" fill=\"{}\"/>",
                    pts,
                    self.get_unchecked("fill"),
                )
            }
            DObject::Angle3P(_, _, _) => unreachable!(),
        }
    }
}

impl StyledDObject<'_> {
    pub(super) fn get_position(&self, loc: f64) -> Option<Point> {
        match &self.obj {
            DObject::Point(p) => Some(*p),
            DObject::Circle(c) => Some(c.point_on(loc)),
            DObject::Arc(arc) => Some(arc.point_on(loc)),
            DObject::Segment(seg) => Some(seg.point_on(loc)),
            DObject::Angle3P(_, _, _) => todo!(),
            _ => None,
        }
    }
}