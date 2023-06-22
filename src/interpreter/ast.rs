use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub enum Linear {
    Line2P(String, String),
    Name(String),
}

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

#[derive(Debug)]
pub enum Object {
    Line2P(String, String),
    Circ3P(String, String, String),
    CircOr(String, Numeric),
    CircOA(String, String),
    CircDiam(String, String),
    Arc(String, String, String),
    ArcO(String, String, String),
    Angle3P(String, String, String),
    Triangle(String, String, String),
    Polygon(Vec<String>),
    Name(String),
    Numeric(Numeric),
    Eval(String),
}

#[derive(Debug)]
pub enum DeclRight {
    OrthoCoord(Numeric, Numeric),
    PolarCoord(Numeric, Numeric),
    Expr(String, Vec<Object>),
    Object(Object),
}

#[derive(Debug)]
pub enum DeclLeft {
    Direct(String),
    Destruct(String, String),
}

#[derive(Debug)]
pub struct Decl(pub DeclLeft, pub DeclRight);

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

pub type Config = HashMap<String, ConfigValue>;

#[derive(Debug)]
pub struct StyledObject {
    pub obj: Object,
    pub config: Option<Config>,
}

#[derive(Debug)]
pub struct DecorObject {
    pub obj: Object,
    pub decor: String,
    pub config: Option<Config>,
}

pub type Draw = Vec<StyledObject>;
pub type Decor = Vec<DecorObject>;

#[derive(Debug)]
pub enum FileLine {
    Config(Box<Config>),
    Draw(Draw),
    Decor(Decor),
    Save(String),
    Decl(Box<Decl>),
}

pub type Main = Vec<FileLine>;
