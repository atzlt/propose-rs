use crate::interpreter::utils::FuncErr;
use std::collections::HashMap;

use if_chain::if_chain;
use metric_rs::{
    calc::{
        basic::{angle, angle_between, Distance},
        construct::midpoint,
    },
    objects::{Circle, Line, Point},
};

use crate::structs::{Arc, Segment};

use super::{
    ast::*, default::DEFAULT_CONFIG, draw::StyledDObject, functions::FUNCTIONS, parser::parse,
    utils::InterpretError,
};

/// Objects related to calculation.
#[derive(Debug, Clone, Copy)]
pub(super) enum GObject {
    Point(Point),
    Line(Line),
    Circle(Circle),
    Trig((Point, Point, Point)),
    Number(f64),
    None,
}

/// Objects related to drawing.
#[derive(Debug)]
pub(super) enum DObject {
    Segment(Segment),
    Arc(Arc),
    Point(Point),
    Circle(Circle),
    Polygon(Vec<Point>),
    Angle3P(Point, Point, Point),
}

impl Into<Result<DObject>> for GObject {
    #[inline]
    fn into(self) -> Result<DObject> {
        match self {
            GObject::Circle(c) => Ok(DObject::Circle(c)),
            GObject::Point(p) => Ok(DObject::Point(p)),
            _ => Err(InterpretError::WrongType),
        }
    }
}

type Result<T> = std::result::Result<T, InterpretError>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LayerType {
    Dots,
    Lines,
    Decor,
    Text,
    Area,
}

const LAYERS_ORDER: [LayerType; 5] = [
    LayerType::Area,
    LayerType::Decor,
    LayerType::Lines,
    LayerType::Dots,
    LayerType::Text,
];

#[derive(Debug)]
pub struct Layer(HashMap<LayerType, String>);

impl Layer {
    fn emit(&mut self, layer: LayerType, string: &str) {
        let entry = self.0.get_mut(&layer);
        if let Some(entry) = entry {
            entry.push_str(string);
        } else {
            self.0.insert(layer, string.to_string());
        }
    }
}

#[derive(Debug)]
pub struct InterpreterState {
    objects: HashMap<String, GObject>,
    layer: Layer,
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
            layer: Layer(HashMap::new()),
            config: DEFAULT_CONFIG.clone(),
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
            FileLine::Draw(draw) => self.draw(draw),
            _ => todo!(),
        }
    }
    #[inline]
    fn config(&mut self, config: Config) -> Result<()> {
        for (key, value) in config {
            self.config.insert(key, value);
        }
        Ok(())
    }
    #[inline]
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
    #[inline]
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
    #[inline]
    fn draw(&mut self, draw: Draw) -> Result<()> {
        for step in draw {
            let obj = StyledDObject {
                obj: self.get_draw_obj(step.obj)?,
                local_conf: step.config,
                global_conf: &self.config,
            };
            let layer = match obj.obj {
                DObject::Point(_) => LayerType::Dots,
                DObject::Segment(_) => LayerType::Lines,
                DObject::Arc(_) => LayerType::Lines,
                DObject::Circle(_) => LayerType::Lines,
                DObject::Polygon(_) => LayerType::Area,
                _ => unreachable!(),
            };
            self.layer.emit(layer, obj.to_string().as_str());
            if let Some(_) = obj.get("label") {
                self.layer.emit(
                    LayerType::Text,
                    obj.label()
                        .map_err(|e| InterpretError::LabelError(e))?
                        .as_str(),
                );
            }
        }
        Ok(())
    }

    // Auxiliary functions.
    #[inline]
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
    #[inline]
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
    #[inline]
    fn get_common(&self, obj: Object) -> Result<GObject> {
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
    #[inline]
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
            _ => self.get_common(obj),
        }
    }
    #[inline]
    fn get_draw_obj(&self, obj: Object) -> Result<DObject> {
        match obj {
            Object::Line2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(DObject::Segment(Segment { from: *a, to: *b })) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Object::Arc(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(DObject::Arc(map_calc_err!(Arc::from_3p(*a, *b, *c)))) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            Object::ArcO(_, _, _) => unimplemented!(),
            Object::Polygon(p) => {
                let mut points = Vec::new();
                for s in p {
                    if let GObject::Point(a) = get!(self.objects, s) {
                        points.push(*a);
                    } else {
                        return Err(InterpretError::WrongType);
                    }
                }
                Ok(DObject::Polygon(points))
            }
            Object::Angle3P(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(DObject::Angle3P(*a, *b, *c)) }
                    else { Err(InterpretError::WrongType) }
                }
            }
            _ => self.get_common(obj)?.into(),
        }
    }
}
