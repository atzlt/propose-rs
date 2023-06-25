use super::structs::{Arc, Segment};
use super::{
    builtin::{config::DEFAULT_CONFIG, functions::FUNCTIONS},
    draw::dobjects::StyledDObject,
    parser::ast::*,
    parser::parse,
    utils::{ConfigValue, DObject, GObject},
};
use crate::interpreter::{draw::CM, utils::FuncError};
use if_chain::if_chain;
use metric_rs::{
    calc::{
        basic::{angle, angle_between, Distance},
        construct::midpoint,
    },
    objects::{Circle, Line, Point},
};
use std::path::Path;
use std::{collections::HashMap, fs};

type Result<T> = std::result::Result<T, InterpretError>;

/// Types of layers.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LayerType {
    Dots,
    Lines,
    Decor,
    Text,
    Area,
}

/// A wrapper of HashMap, representing the layers.
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
    fn get(&self, layer: LayerType) -> String {
        self.0.get(&layer).unwrap_or(&String::new()).clone()
    }
}

#[derive(Debug)]
pub enum InterpretError {
    ParseError(String),
    FuncError(FuncError),
    MissingKey(String),
    IOError(std::io::Error),
    WrongGeometricType,
    WrongConfigType,
    LabelObjNotSupported,
    DecorObjNotSupported,
    NoSuchDecor,
}

/// Represents the state of an interpreter.
#[derive(Debug)]
pub struct InterpreterState {
    objects: HashMap<String, GObject>,
    layer: Layer,
    config: Config,
}

/// Convenience macro to get a value or fail with `MissingKey`.
macro_rules! get {
    ($objs:expr , $key:ident) => {
        $objs
            .get(&$key)
            .ok_or_else(|| InterpretError::MissingKey($key))?
    };
}

impl Default for InterpreterState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
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
    pub fn clear(&mut self) {
        self.objects.clear();
        self.layer.0.clear();
        self.config.clone_from(&DEFAULT_CONFIG);
    }
    #[inline]
    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let input = parse(source);
        match input {
            Err(e) => Err(InterpretError::ParseError(e.to_string())),
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
            FileLine::Decl(decl) => self.decl(*decl),
            FileLine::Draw(draw) => self.draw(draw),
            FileLine::Decor(decor) => self.decor(decor),
            FileLine::Save(path) => self.save(path),
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
                if y != "_" {
                    self.objects.insert(y, value.1);
                }
            }
        }
        Ok(())
    }
    /// Get the value on the right for a `decl` statement.
    /// This method should always return a result of a tuple, so destrct assignment could work.
    #[inline]
    fn decl_right(&self, decl: DeclRight) -> Result<(GObject, GObject)> {
        match decl {
            DeclRight::OrthoCoord(x, y) => {
                let x = self.get_numeric(*x)?;
                let y = self.get_numeric(*y)?;
                Ok((GObject::Point(Point { x, y }), GObject::None))
            }
            DeclRight::PolarCoord(r, t) => {
                let r = self.get_numeric(*r)?;
                let t = self.get_numeric(*t)?;
                Ok((
                    GObject::Point(Point {
                        x: r * t.cos(),
                        y: r * t.sin(),
                    }),
                    GObject::None,
                ))
            }
            DeclRight::Object(obj) => Ok((self.get_arg_obj(*obj)?, GObject::None)),
            DeclRight::Expr(method, args) => {
                let mut gobjs = Vec::with_capacity(args.len());
                // Get all arguments.
                for arg in args {
                    let obj = self.get_arg_obj(arg)?;
                    gobjs.push(obj);
                }
                let func = FUNCTIONS.get(&method);
                if let Some(func) = func {
                    let result = func(gobjs)?;
                    Ok(result)
                } else {
                    Err(InterpretError::FuncError(FuncError::NoFunc(method)))
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
            // Emit code at the correct layer.
            let layer = match obj.obj {
                DObject::Point(_) => LayerType::Dots,
                DObject::Segment(_) => LayerType::Lines,
                DObject::Arc(_) => LayerType::Lines,
                DObject::Circle(_) => LayerType::Lines,
                DObject::Polygon(_) => LayerType::Area,
                _ => unreachable!(),
            };
            self.layer.emit(layer, obj.to_string().as_str());
            // If a label is present, emit that label.
            if obj.get("label").is_some() {
                self.layer.emit(LayerType::Text, obj.label()?.as_str());
            }
        }
        Ok(())
    }
    #[inline]
    fn decor(&mut self, decor: Decor) -> Result<()> {
        for step in decor {
            let obj = StyledDObject {
                obj: self.get_draw_obj(step.obj)?,
                local_conf: step.config,
                global_conf: &self.config,
            };
            self.layer
                .emit(LayerType::Decor, obj.decor(&step.decor)?.as_str());
        }
        Ok(())
    }
    #[inline]
    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        fs::write(path, self.emit()?).map_err(InterpretError::IOError)
    }
    /// Emit the complete SVG code.
    #[inline]
    fn emit(&self) -> Result<String> {
        let width = self.config.get("width").unwrap().try_into_f64()? * CM;
        let height = self.config.get("height").unwrap().try_into_f64()? * CM;
        Ok(format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\">\n{}\n{}\n{}\n{}\n{}\n</svg>\n",
            width,
            height,
            if let Some(ConfigValue::Number(min_x)) = self.config.get("minX") {
                *min_x
            } else {
                -width / 2.0
            },
            if let Some(ConfigValue::Number(min_y)) = self.config.get("minY") {
                *min_y
            } else {
                -height / 2.0
            },
            width,
            height,
            self.layer.get(LayerType::Area),
            self.layer.get(LayerType::Lines),
            self.layer.get(LayerType::Decor),
            self.layer.get(LayerType::Dots),
            self.layer.get(LayerType::Text),
        ))
    }

    // Auxiliary functions.
    /// Get Linear objects.
    #[inline]
    fn get_linear(&self, lin: Linear) -> Result<Line> {
        match lin {
            Linear::Name(s) => {
                // let GObject::Line(l) = get!(self.objects, s);
                // Ok(*l)
                if let GObject::Line(l) = get!(self.objects, s) {
                    Ok(*l)
                } else {
                    Err(InterpretError::WrongGeometricType)
                }
            }
            Linear::Line2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(Line::from_2p(*a, *b)?) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
        }
    }
    /// Get Numeric values.
    #[inline]
    fn get_numeric(&self, num: Numeric) -> Result<f64> {
        match num {
            Numeric::Number(x) => Ok(x),
            Numeric::Name(s) => {
                if let GObject::Number(x) = get!(self.objects, s) {
                    Ok(*x)
                } else {
                    Err(InterpretError::WrongGeometricType)
                }
            }
            Numeric::Distance2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(a.distance(*b)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Numeric::DistancePL(a, l) => {
                if let GObject::Point(a) = get!(self.objects, a) {
                    let l = self.get_linear(l)?;
                    Ok(a.distance(l))
                } else {
                    Err(InterpretError::WrongGeometricType)
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
                    then { Ok(angle(*a, *o, *b)?) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Numeric::Angle2L(k, l) => {
                let k = self.get_linear(k)?;
                let l = self.get_linear(l)?;
                Ok(angle_between(l, k))
            }
        }
    }
    /// Get common patterns in arguments and `draw_obj`s.
    #[inline]
    fn get_common(&self, obj: Object) -> Result<GObject> {
        match obj {
            Object::Name(s) => Ok(*get!(self.objects, s)),
            Object::Circ3P(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(GObject::Circle(Circle::from_3p(*a, *b, *c)?)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::CircOr(o, r) => {
                if let GObject::Point(o) = get!(self.objects, o) {
                    let r = self.get_numeric(*r)?;
                    Ok(GObject::Circle(Circle::from_center_radius(*o, r)?))
                } else {
                    Err(InterpretError::WrongGeometricType)
                }
            }
            Object::CircOA(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(GObject::Circle(Circle::from_center_point(*a, *b)?)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::CircDiam(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(GObject::Circle(Circle::from_center_point(
                        midpoint(*a, *b),
                        *b
                    )?)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            // This is assured by the parser.
            _ => unreachable!(),
        }
    }
    /// Get objects that appears in arguments.
    #[inline]
    fn get_arg_obj(&self, obj: Object) -> Result<GObject> {
        match obj {
            Object::Line2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(GObject::Line(Line::from_2p(*a, *b)?)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::Triangle(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(GObject::Trig((*a, *b, *c))) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::Numeric(n) => Ok(GObject::Number(self.get_numeric(*n)?)),
            // TODO: Evaluation.
            Object::Eval(_) => unimplemented!(),
            _ => self.get_common(obj),
        }
    }
    /// Get objects to draw.
    #[inline]
    fn get_draw_obj(&self, obj: Object) -> Result<DObject> {
        match obj {
            Object::Line2P(a, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(DObject::Segment(Segment { from: *a, to: *b })) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::Arc(a, b, c) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(b) = get!(self.objects, b);
                    if let GObject::Point(c) = get!(self.objects, c);
                    then { Ok(DObject::Arc(Arc::from_3p(*a, *b, *c)?)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::ArcO(a, o, b) => {
                if_chain! {
                    if let GObject::Point(a) = get!(self.objects, a);
                    if let GObject::Point(o) = get!(self.objects, o);
                    if let GObject::Point(b) = get!(self.objects, b);
                    then { Ok(DObject::Arc(Arc::from_center(*a, *o, *b)?)) }
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            Object::Polygon(p) => {
                let mut points = Vec::with_capacity(p.len());
                for s in p {
                    if let GObject::Point(a) = get!(self.objects, s) {
                        points.push(*a);
                    } else {
                        return Err(InterpretError::WrongGeometricType);
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
                    else { Err(InterpretError::WrongGeometricType) }
                }
            }
            _ => self.get_common(obj)?.into(),
        }
    }
}
