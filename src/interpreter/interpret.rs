use std::collections::HashMap;

use if_chain::if_chain;
use lazy_static::lazy_static;
use metric_rs::{
    calc::{
        basic::{angle, angle_between, Distance},
        construct::midpoint,
    },
    objects::{Circle, Line, Point},
};

use super::{
    ast::*,
    functions::{FuncErr, FUNCTIONS},
    parser::parse,
};

lazy_static! {
    static ref DEFAULT_CONFIG: HashMap<String, ConfigValue> =
        HashMap::from([(String::from("thick"), ConfigValue::Number(5.0)),]);
}

#[derive(Debug, Clone, Copy)]
pub enum GObject {
    Point(Point),
    Line(Line),
    Circle(Circle),
    Trig((Point, Point, Point)),
    Number(f64),
    None,
}

#[derive(Debug)]
pub enum InterpretError {
    ParseError(String),
    FuncError(FuncErr),
    MissingKey(String),
    WrongType,
}

type Result<T> = std::result::Result<T, InterpretError>;

#[derive(Debug)]
pub struct InterpreterState {
    objects: HashMap<String, GObject>,
    config: Config,
}

macro_rules! get {
    ($objs:expr , $key:ident) => {
        $objs
            .get(&$key)
            .ok_or_else(|| InterpretError::MissingKey($key))?
    };
}

macro_rules! map_calc_err {
    ($e:expr) => {
        $e.map_err(|e| InterpretError::FuncError(FuncErr::CalcError(e)))?
    };
}

impl InterpreterState {
    #[inline]
    pub fn new() -> Self {
        InterpreterState {
            objects: HashMap::new(),
            config: Config::new(),
        }
    }
    #[inline]
    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let input = parse(source);
        match input {
            Err(e) => {
                return Err(InterpretError::ParseError(e.to_string()));
            }
            Ok(input) => {
                for line in input {
                    self._interpret(line)?;
                }
                Ok(())
            }
        }
    }
    #[inline]
    fn _interpret(&mut self, input: FileLine) -> Result<()> {
        match input {
            FileLine::Config(config) => self.config(config),
            FileLine::Decl(decl) => self.decl(decl),
            _ => todo!(),
        }
    }

    fn config(&mut self, mut config: Config) -> Result<()> {
        config.extend((*DEFAULT_CONFIG).clone());
        self.config = config;
        Ok(())
    }
    fn decl(&mut self, decl: Decl) -> Result<()> {
        let Decl(left, right) = decl;
        let value = self.decl_right(right)?;
        match left {
            DeclLeft::Direct(x) => {
                self.objects.insert(x, value.0);
            }
            DeclLeft::Destruct(x, y) => {
                self.objects.insert(x, value.0);
                self.objects.insert(y, value.1);
            }
        }
        Ok(())
    }
    fn decl_right(&self, decl: DeclRight) -> Result<(GObject, GObject)> {
        match decl {
            DeclRight::OrthoCoord(x, y) => {
                let x = self.get_numeric(x)?;
                let y = self.get_numeric(y)?;
                Ok((GObject::Point(Point { x, y }), GObject::None))
            }
            DeclRight::PolarCoord(r, t) => {
                let r = self.get_numeric(r)?;
                let t = self.get_numeric(t)?;
                Ok((
                    GObject::Point(Point {
                        x: r * t.cos(),
                        y: r * t.sin(),
                    }),
                    GObject::None,
                ))
            }
            DeclRight::Object(obj) => Ok((self.get_arg_obj(obj)?, GObject::None)),
            DeclRight::Expr(method, args) => {
                let mut gobjs = Vec::new();
                for arg in args {
                    let obj = self.get_arg_obj(arg)?;
                    gobjs.push(obj);
                }
                let func = FUNCTIONS.get(&method);
                if let Some(func) = func {
                    let result = func(gobjs).map_err(|e| InterpretError::FuncError(e))?;
                    Ok(result)
                } else {
                    return Err(InterpretError::FuncError(FuncErr::NoFunc(method)));
                }
            }
        }
    }

    // Auxiliary functions.

    fn get_linear(&self, lin: Linear) -> Result<Line> {
        match lin {
            Linear::Name(s) => {
                // let GObject::Line(l) = get!(self.objects, s);
                // Ok(*l)
                if let GObject::Line(l) = get!(self.objects, s) {
                    Ok(*l)
                } else {
                    Err(InterpretError::WrongType)
                }
            }
            Linear::Line2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(map_calc_err!(Line::from_2p(*a, *b))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
        }
    }
    fn get_numeric(&self, num: Numeric) -> Result<f64> {
        match num {
            Numeric::Number(x) => Ok(x),
            Numeric::Name(s) => {
                if let GObject::Number(x) = get!(self.objects, s) {
                    Ok(*x)
                } else {
                    Err(InterpretError::WrongType)
                }
            }
            Numeric::Distance2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(a.distance(*b)) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Numeric::DistancePL(a, l) => {
                if let GObject::Point(a) = get!(self.objects, a) {
                    let l = self.get_linear(l)?;
                    Ok(a.distance(l))
                } else {
                    Err(InterpretError::WrongType)
                }
            }
            Numeric::Distance2L(k, l) => {
                let k = self.get_linear(k)?;
                let l = self.get_linear(l)?;
                Ok(k.distance(l))
            }
            Numeric::Angle3P(a, o, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(o) = get!(self.objects, o);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(map_calc_err!(angle(*a, *o, *b))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Numeric::Angle2L(k, l) => {
                let k = self.get_linear(k)?;
                let l = self.get_linear(l)?;
                Ok(angle_between(l, k))
            }
        }
    }
    fn get_circle(&self, obj: Object) -> Result<GObject> {
        match obj {
            Object::Name(s) => Ok(*get!(self.objects, s)),
            Object::Circ3P(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(GObject::Circle(map_calc_err!(Circle::from_3p(*a, *b, *c)))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Object::CircOr(o, r) => {
                if let GObject::Point(o) = get!(self.objects, o) {
                    let r = self.get_numeric(r)?;
                    Ok(GObject::Circle(map_calc_err!(Circle::from_center_radius(
                        *o, r
                    ))))
                } else {
                    Err(InterpretError::WrongType)
                }
            }
            Object::CircOA(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(GObject::Circle(map_calc_err!(Circle::from_center_point(
                        *a, *b
                    )))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Object::CircDiam(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(GObject::Circle(map_calc_err!(Circle::from_center_point(
                        midpoint(*a, *b),
                        *b
                    )))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            // This is assured by the parser.
            _ => unreachable!(),
        }
    }
    fn get_arg_obj(&self, obj: Object) -> Result<GObject> {
        match obj {
            Object::Line2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(GObject::Line(map_calc_err!(Line::from_2p(*a, *b)))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Object::Triangle(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(GObject::Trig((*a, *b, *c))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Object::Numeric(n) => Ok(GObject::Number(self.get_numeric(n)?)),
            Object::Eval(_) => unimplemented!(),
            _ => self.get_circle(obj),
        }
    }
}
