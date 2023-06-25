use std::fmt::Display;
use metric_rs::{calc::exception::CalcException, objects::{Point, Circle, Line}};
use serde::Serialize;
use super::{structs::{Segment, Arc}, interpret::InterpretError};

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug)]
pub enum ConversionError {
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
#[derive(Debug, Clone, Copy, Serialize)]
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

impl From<GObject> for Result<DObject, InterpretError> {
    #[inline]
    fn from(val: GObject) -> Result<DObject, InterpretError> {
        match val {
            GObject::Circle(c) => Ok(DObject::Circle(c)),
            GObject::Point(p) => Ok(DObject::Point(p)),
            _ => Err(InterpretError::WrongGeometricType),
        }
    }
}

#[derive(Debug)]
pub enum LabelError {
    WrongConfigType,
    ObjNotSupported,
}

#[derive(Debug)]
pub enum DecorError {
    NoSuchDecor,
    ObjNotSupported,
    WrongConfigType,
}

#[derive(Debug)]
pub enum FuncError {
    CalcError(CalcException),
    ArgError,
    NoFunc(String),
}

// Conversions.

impl From<bool> for ConfigValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<CalcException> for InterpretError {
    #[inline]
    fn from(value: CalcException) -> Self {
        Self::FuncError(FuncError::CalcError(value))
    }
}

impl From<FuncError> for InterpretError {
    #[inline]
    fn from(value: FuncError) -> Self {
        Self::FuncError(value)
    }
}

impl From<ConversionError> for InterpretError {
    #[inline]
    fn from(value: ConversionError) -> Self {
        match value {
            ConversionError::ToF64 => Self::WrongConfigType,
        }
    }
}

impl From<LabelError> for InterpretError {
    #[inline]
    fn from(value: LabelError) -> Self {
        match value {
            LabelError::ObjNotSupported => Self::LabelObjNotSupported,
            LabelError::WrongConfigType => Self::WrongConfigType,
        }
    }
}

impl From<DecorError> for InterpretError {
    #[inline]
    fn from(value: DecorError) -> Self {
        match value {
            DecorError::ObjNotSupported => Self::DecorObjNotSupported,
            DecorError::WrongConfigType => Self::WrongConfigType,
            DecorError::NoSuchDecor => Self::NoSuchDecor
        }
    }
}

// Display error.

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::DecorObjNotSupported => write!(f, "Cannot decorate object"),
            InterpretError::FuncError(e) => write!(f, "Error when running function: {}", e),
            InterpretError::IOError(e) => write!(f, "Error when saving file: {}", e),
            InterpretError::LabelObjNotSupported => write!(f, "Cannot label object"),
            InterpretError::MissingKey(key) => write!(f, "Cannot find name: {}", key),
            InterpretError::NoSuchDecor => write!(f, "No such decoration"),
            InterpretError::ParseError(e) => write!(f, "Cannot parse content: {}", e),
            InterpretError::WrongConfigType => write!(f, "Type of configuration is not correct"),
            InterpretError::WrongGeometricType => write!(f, "Type of geometric object is not correct"),
        }
    }
}

impl Display for FuncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncError::ArgError => write!(f, "Wrong argument"),
            FuncError::CalcError(e) => write!(f, "Exception during calculation: {}", e),
            FuncError::NoFunc(name) => write!(f, "No such method: {}", name),
        }
    }
}
