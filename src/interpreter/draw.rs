use std::fmt::Display;

use if_chain::if_chain;
use itertools::Itertools;
use metric_rs::calc::point_on::PointOn;

use crate::structs::{Arc, Segment};

use super::{
    ast::{Config, ConfigValue},
    utils::DObject,
    utils::LabelError,
};

pub(super) const CM: f64 = 37.795;

#[derive(Debug)]
pub(super) struct StyledDObject<'conf> {
    pub obj: DObject,
    pub local_conf: Option<Config>,
    pub global_conf: &'conf Config,
}

impl StyledDObject<'_> {
    #[inline]
    pub(super) fn get(&self, key: &str) -> Option<&ConfigValue> {
        let key = key.to_string();
        if_chain! {
            if let Some(conf) = &self.local_conf;
            if let Some(value) = conf.get(&key);
            then { Some(value) }
            else { self.global_conf.get(&key) }
        }
    }
    #[inline]
    fn get_unchecked(&self, key: &str) -> &ConfigValue {
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
                write!(
                    f,
                    "<line x1=\"{}cm\" y1=\"{}cm\" x2=\"{}cm\" y2=\"{}cm\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
                    a.x,
                    -a.y,
                    b.x,
                    -b.y,
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
                write!(
                    f,
                    "<circle cx=\"{}cm\" cy=\"{}cm\" r=\"{}cm\" stroke=\"{}\" fill=\"{}\" stroke-width=\"{}\"{}/>",
                    circ.O.x,
                    -circ.O.y,
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
                write!(
                    f,
                    "<circle cx=\"{}cm\" cy=\"{}cm\" r=\"{}\" stroke=\"{}\" fill=\"{}\" stroke-width=\"{}\"/>",
                    p.x,
                    -p.y,
                    self.get_unchecked("dotsize"),
                    self.get_unchecked("dotstroke"),
                    self.get_unchecked("dotfill"),
                    self.get_unchecked("dotwidth"),
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
                    .into_iter()
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
    pub(super) fn label(&self) -> Result<String, LabelError> {
        let label = self.get_unchecked("label");
        let size = self.get_unchecked("labelsize");
        let dist = self
            .get_unchecked("dist")
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)?
            / CM;
        let angle = self
            .get_unchecked("angle")
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)?;
        let loc = self
            .get_unchecked("loc")
            .try_into_f64()
            .map_err(|_| LabelError::WrongConfigType)?;
        let font = self.get_unchecked("font");

        let obj = &self.obj;
        let pos = match obj {
            DObject::Point(p) => Ok(*p),
            DObject::Circle(c) => Ok(c.point_on(loc)),
            DObject::Arc(arc) => Ok(arc.point_on(loc)),
            DObject::Segment(seg) => Ok(seg.point_on(loc)),
            _ => Err(LabelError::ObjNotSupported),
        }?;
        Ok(format!(
            "<text font-size=\"{}\" font-family=\"{}\" font-style=\"italic\" text-anchor=\"middle\" dominant-baseline=\"middle\" x=\"{}cm\" y=\"{}cm\">{}</text>",
            size,
            font,
            pos.x + dist * angle.cos(),
            -(pos.y + dist * angle.sin()),
            label,
        ))
    }
}
