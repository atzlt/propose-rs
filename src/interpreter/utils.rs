use metric_rs::calc::exception::CalcException;

use super::ast::ConfigValue;

#[derive(Debug)]
pub enum ConversionError {
    ToF64,
}

impl ConfigValue {
    #[inline]
    pub(super) fn try_into_f64(&self) -> Result<f64, ConversionError> {
        match self {
            Self::Number(n) => Ok(*n),
            _ => Err(ConversionError::ToF64),
        }
    }
}

#[derive(Debug)]
pub enum LabelError {
    WrongConfigType,
    ObjNotSupported,
}

#[derive(Debug)]
pub enum InterpretError {
    ParseError(String),
    LabelError(LabelError),
    FuncError(FuncErr),
    MissingKey(String),
    WrongType,
}

#[derive(Debug)]
pub enum FuncErr {
    CalcError(CalcException),
    ArgError,
    NoFunc(String),
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

impl From<bool> for ConfigValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
