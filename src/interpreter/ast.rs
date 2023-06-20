use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub(super) enum Linear {
    Line2P(String, String),
    Name(String),
}

#[derive(Debug)]
pub(super) enum Numeric {
    Distance2P(String, String),
    DistancePL(String, Linear),
    Distance2L(Linear, Linear),
    Angle3P(String, String, String),
    Angle2L(Linear, Linear),
    Number(f64),
    Name(String),
}

#[derive(Debug)]
pub(super) enum Object {
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
pub(super) enum DeclRight {
    OrthoCoord(Numeric, Numeric),
    PolarCoord(Numeric, Numeric),
    Expr(String, Vec<Object>),
    Object(Object),
}

#[derive(Debug)]
pub(super) enum DeclLeft {
    Direct(String),
    Destruct(String, String),
}

#[derive(Debug)]
pub(super) struct Decl(pub(super) DeclLeft, pub(super) DeclRight);

#[derive(Debug, Clone)]
pub(super) enum ConfigValue {
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

pub(super) type Config = HashMap<String, ConfigValue>;

#[derive(Debug)]
pub(super) struct StyledObject {
    pub(super) obj: Object,
    pub(super) config: Option<Config>,
}

#[derive(Debug)]
pub(super) struct DecorObject {
    pub(super) obj: Object,
    pub(super) decor: String,
    pub(super) config: Option<Config>,
}

pub(super) type Draw = Vec<StyledObject>;
pub(super) type Decor = Vec<DecorObject>;

#[derive(Debug)]
pub(super) enum FileLine {
    Config(Config),
    Draw(Draw),
    Decor(Decor),
    Save(String),
    Decl(Decl),
}

pub(super) type Main = Vec<FileLine>;
