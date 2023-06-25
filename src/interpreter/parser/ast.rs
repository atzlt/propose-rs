use crate::interpreter::utils::ConfigValue;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub enum Linear {
    Line2P(String, String),
    Name(String),
}

#[derive(Debug, Serialize)]
pub enum Numeric {
    Distance2P(String, String),
    DistancePL(String, Linear),
    Distance2L(Linear, Linear),
    Angle3P(String, String, String),
    Angle2L(Linear, Linear),
    Number(f64),
    Name(String),
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub enum DeclRight {
    OrthoCoord(Box<Numeric>, Box<Numeric>),
    PolarCoord(Box<Numeric>, Box<Numeric>),
    Expr(String, Vec<Object>),
    Object(Box<Object>),
}

#[derive(Debug, Serialize)]
pub enum DeclLeft {
    Direct(String),
    Destruct(String, String),
}

#[derive(Debug, Serialize)]
pub struct Decl(pub DeclLeft, pub DeclRight);

pub type Config = HashMap<String, ConfigValue>;

#[derive(Debug, Serialize)]
pub struct StyledObject {
    pub obj: Object,
    pub config: Option<Config>,
}

#[derive(Debug, Serialize)]
pub struct DecorObject {
    pub obj: Object,
    pub decor: String,
    pub config: Option<Config>,
}

pub type Draw = Vec<StyledObject>;
pub type Decor = Vec<DecorObject>;

#[derive(Debug, Serialize)]
pub enum FileLine {
    Config(Config),
    Draw(Draw),
    Decor(Decor),
    Save(String),
    Decl(Box<Decl>),
}

pub type Main = Vec<FileLine>;
