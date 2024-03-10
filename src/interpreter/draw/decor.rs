use super::render::StyledDObject;
use crate::interpreter::{
    builtin::decor::DECORATIONS,
    utils::{ConfigValue, DecorError},
};
use anyhow::Result;
use if_chain::if_chain;
use metric_rs::objects::Point;

#[derive(Debug)]
pub struct DecorConfig {
    pub pos: Point,
    pub size: f64,
    pub angle: f64,
    pub width: f64,
    pub color: String,
    pub fill: String,
}

impl DecorConfig {
    pub fn get_from_styled_dobj(dobj: &StyledDObject) -> Result<Self> {
        if_chain! {
            if let ConfigValue::Number(loc) = dobj.get_unchecked("loc");
            if let ConfigValue::Number(size) = dobj.get_unchecked("decorsize");
            if let ConfigValue::Number(width) = dobj.get_unchecked("decorwidth");
            if let ConfigValue::String(color) = dobj.get_unchecked("decorcolor");
            if let ConfigValue::String(fill) = dobj.get_unchecked("decorfill");
            then {
                Ok(DecorConfig {
                    pos: dobj.get_position(*loc),
                    size: *size,
                    angle: dobj.get_tan_angle(*loc),
                    width: *width,
                    color: color.clone(),
                    fill: fill.clone(),
                })
            }
            else { Err(DecorError::WrongConfigType)? }
        }
    }
}

impl StyledDObject<'_> {
    pub fn decor(&self, decor: &str) -> Result<String> {
        let decor_config = DecorConfig::get_from_styled_dobj(self)?;
        let decor_func = DECORATIONS.get(decor).ok_or(DecorError::NoSuchDecor)?;
        Ok(decor_func(decor_config))
    }
}
