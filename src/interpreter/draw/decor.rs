use super::dobjects::StyledDObject;
use crate::{
    interpreter::{ast::ConfigValue, draw::CM, utils::DecorError},
    write_line,
};
use if_chain::if_chain;
use lazy_static::lazy_static;
use metric_rs::objects::Point;
use std::fmt::Write;
use std::{collections::HashMap, f64::consts::PI};

#[derive(Debug)]
struct DecorConfig {
    pos: Point,
    size: f64,
    angle: f64,
    width: f64,
    color: String,
    fill: String,
}

impl DecorConfig {
    pub fn get_from_styled_dobj(dobj: &StyledDObject) -> Result<Self, DecorError> {
        if_chain! {
            if let ConfigValue::Number(loc) = dobj.get_unchecked("loc");
            if let ConfigValue::Number(size) = dobj.get_unchecked("decorsize");
            if let ConfigValue::Number(width) = dobj.get_unchecked("decorwidth");
            if let ConfigValue::String(color) = dobj.get_unchecked("decorcolor");
            if let ConfigValue::String(fill) = dobj.get_unchecked("decorfill");
            then {
                Ok(DecorConfig {
                    pos: dobj.get_position(*loc).ok_or(DecorError::ObjNotSupported)?,
                    size: *size,
                    angle: dobj.get_tan_angle(*loc).ok_or(DecorError::ObjNotSupported)?,
                    width: *width,
                    color: color.clone(),
                    fill: fill.clone(),
                })
            }
            else { Err(DecorError::WrongConfigType) }
        }
    }
}

impl StyledDObject<'_> {
    pub fn decor(&self, decor: &str) -> Result<String, DecorError> {
        let decor_config = DecorConfig::get_from_styled_dobj(self)?;
        let decor_func = DECORATIONS.get(decor).ok_or(DecorError::NoSuchDecor)?;
        Ok(decor_func(decor_config))
    }
}

type DecorFunction = fn(DecorConfig) -> String;

macro_rules! entry {
    ($key:literal, $body:expr) => {
        ($key, ($body) as _)
    };
}

lazy_static! {
    static ref DECORATIONS: HashMap<&'static str, DecorFunction> = HashMap::from([
        entry!("|", |DecorConfig {
                         pos,
                         size,
                         angle,
                         width,
                         color,
                         fill: _,
                     }| {
            let offset = Point::new(-angle.sin() * size, angle.cos() * size);
            let pos = pos * CM;
            let p1 = pos + offset;
            let p2 = pos - offset;
            let mut string = String::new();
            write_line!(string, p1, p2 => in px, color, width, "").unwrap();
            string
        }),
        entry!("||", |DecorConfig {
                          pos,
                          size,
                          angle,
                          width,
                          color,
                          fill: _,
                      }| {
            let sin = angle.sin();
            let cos = angle.cos();
            let offset = Point::new(-sin * size, cos * size);
            let gap = Point::new(cos * size / 3.0, sin * size / 3.0);
            let pos = pos * CM;
            let mut string = String::new();
            write_line!(
                string,
                pos - gap + offset,
                pos - gap - offset => in px,
                color,
                width,
                ""
            )
            .unwrap();
            write_line!(
                string,
                pos + gap + offset,
                pos + gap - offset => in px,
                color,
                width,
                ""
            )
            .unwrap();
            string
        }),
        entry!(">", |DecorConfig {
                         pos,
                         size,
                         angle,
                         width,
                         color,
                         fill: _,
                     }| {
            let offset1 = Point::new(angle.cos() * size, angle.sin() * size);
            let offset2 = Point::new(
                (angle + PI * 2.0 / 3.0).cos() * size,
                (angle + PI * 2.0 / 3.0).sin() * size,
            );
            let offset3 = Point::new(
                (angle - PI * 2.0 / 3.0).cos() * size,
                (angle - PI * 2.0 / 3.0).sin() * size,
            );
            let pos = pos * CM;
            let mut string = String::new();
            write_line!(string, pos + offset1, pos + offset2 => in px, color, width, "").unwrap();
            write_line!(string, pos + offset1, pos + offset3 => in px, color, width, "").unwrap();
            string
        })
    ]);
}
