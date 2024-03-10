use std::f64::consts::{E, PI};

use meval::{ContextProvider, Expr};

use super::{
    interpret::{InterpretError, InterpreterState},
    utils::GObject,
};

macro_rules! func {
    ($args:ident; $len:literal, $ret:expr) => {
        if $args.len() < $len {
            Err(meval::FuncEvalError::TooFewArguments)
        } else {
            Ok($ret)
        }
    };
}

impl ContextProvider for InterpreterState {
    #[inline]
    fn get_var(&self, key: &str) -> Option<f64> {
        match key {
            "pi" => Some(PI),
            "e" => Some(E),
            "deg" => Some(PI / 180.0),
            _ => {
                if let Some(GObject::Number(x)) = self.get(key) {
                    Some(*x)
                } else {
                    None
                }
            }
        }
    }
    #[inline]
    fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, meval::FuncEvalError> {
        match name {
            "sqrt" => func!(args; 1, args[0].sqrt()),
            "sin" => func!(args; 1, args[0].sin()),
            "cos" => func!(args; 1, args[0].cos()),
            "tan" => func!(args; 1, args[0].tan()),
            _ => Err(meval::FuncEvalError::UnknownFunction),
        }
    }
}

impl InterpreterState {
    #[inline]
    pub(super) fn eval(&self, expr: &str) -> Result<f64, InterpretError> {
        let expr: Expr = expr.parse()?;
        let result = expr.eval_with_context(self)?;
        Ok(result)
    }
}
