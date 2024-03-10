use crate::interpreter::utils::ConfigValue;
#[cfg(test)]
use serde::Serialize;
use std::collections::HashMap;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub enum Linear {
    Line2P(String, String),
    Name(String),
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub enum Numeric {
    Distance2P(String, String),
    DistancePL(String, Linear),
    Distance2L(Linear, Linear),
    Angle3P(String, String, String),
    Angle2L(Linear, Linear),
    Number(f64),
    Name(String),
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub enum Object {
    Line2P(String, String),
    Circ3P(String, String, String),
    CircOr(String, Box<Numeric>),
    CircOA(String, String),
    CircDiam(String, String),
    Arc(String, String, String),
    ArcO(String, String, String),
    Angle3P(String, String, String),
    Triangle(String, String, String),
    Polygon(Vec<String>),
    Name(String),
    Numeric(Box<Numeric>),
    Eval(String),
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub enum DeclRight {
    OrthoCoord(Box<Numeric>, Box<Numeric>),
    PolarCoord(Box<Numeric>, Box<Numeric>),
    Expr(String, Vec<Object>),
    Object(Box<Object>),
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub enum DeclLeft {
    Direct(String),
    Destruct(String, String),
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub struct Decl(pub DeclLeft, pub DeclRight);

pub type Config = HashMap<String, ConfigValue>;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub struct StyledObject {
    pub obj: Object,
    pub config: Option<Config>,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub struct DecorObject {
    pub obj: Object,
    pub decor: String,
    pub config: Option<Config>,
}

pub type Draw = Vec<StyledObject>;
pub type Decor = Vec<DecorObject>;

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug)]
pub enum FileLine {
    Config(Config),
    Draw(Draw),
    Decor(Decor),
    Decl(Box<Decl>),
}

pub type Main = Vec<FileLine>;
