use super::{
    interpret::InterpretError,
    structs::{Arc, Segment},
};
use anyhow::Result;
use metric_rs::{
    calc::exception::CalcException, objects::{Circle, Line, Point}
};
#[cfg(test)]
use serde::Serialize;
use std::fmt::Display;
use thiserror::Error;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone)]
pub enum ConfigValue {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Display for ConfigValue {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Cannot convert into f64")]
    ToF64,
}

impl ConfigValue {
    #[inline]
    pub fn try_into_f64(&self) -> Result<f64, ConversionError> {
        match self {
            Self::Number(n) => Ok(*n),
            _ => Err(ConversionError::ToF64),
        }
    }
}

impl From<f64> for ConfigValue {
    #[inline]
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<&str> for ConfigValue {
    #[inline]
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

/// Objects related to calculation.
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Clone, Copy)]
pub enum GObject {
    Point(Point),
    Line(Line),
    Circle(Circle),
    Trig((Point, Point, Point)),
    Number(f64),
    None,
}

/// Objects related to drawing.
#[derive(Debug)]
pub enum DObject {
    Segment(Segment),
    Arc(Arc),
    Point(Point),
    Circle(Circle),
    Polygon(Vec<Point>),
    Angle3P(Point, Point, Point),
}

impl From<GObject> for Result<DObject> {
    #[inline]
    fn from(val: GObject) -> Result<DObject> {
        match val {
            GObject::Circle(c) => Ok(DObject::Circle(c)),
            GObject::Point(p) => Ok(DObject::Point(p)),
            _ => Err(InterpretError::WrongGeometricType)?,
        }
    }
}

#[derive(Debug, Error)]
pub enum LabelError {
    #[error("Wrong configuration type")]
    WrongConfigType,
}

#[derive(Debug, Error)]
pub enum DecorError {
    #[error("No such decoration")]
    NoSuchDecor,
    #[error("Wrong configuration type")]
    WrongConfigType,
}

#[derive(Debug, Error)]
pub enum FuncError {
    #[error("Argument error")]
    ArgError,
    #[error("No such method: {0}")]
    NoFunc(String),
    #[error("Calculation exception: {0}")]
    CalcError(CalcException)
}

// Conversions.

impl From<bool> for ConfigValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<meval::Error> for InterpretError {
    #[inline]
    fn from(value: meval::Error) -> Self {
        Self::EvalError(value)
    }
}
