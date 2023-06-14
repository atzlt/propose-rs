#[derive(Debug)]
pub enum Name {
    Point(String),
    Common(String),
}

#[derive(Debug)]
pub enum Linear {
    Line2P(Name, Name),
    Name(Name),
}

#[derive(Debug)]
pub enum Numeric {
    Distance2P(Name, Name),
    Distance2L(Linear, Linear),
    Angle3P(Name, Name, Name),
    Angle2L(Linear, Linear),
    Number(f64),
    Name(Name),
}

#[derive(Debug)]
pub enum Object {
    Line2P(Name, Name),
    Circ3P(Name, Name, Name),
    CircOr(Name, Numeric),
    CircOA(Name, Name),
    CircDiam(Name, Name),
    Arc(Name, Name, Name),
    ArcO(Name, Name, Name),
    Angle3P(Name, Name, Name),
    Triangle(Name, Name, Name),
    Polygon(Vec<Name>),
    Name(Name),
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
    Direct(Name),
    Destruct(Name, Name),
}

#[derive(Debug)]
pub struct Decl(pub DeclLeft, pub DeclRight);

#[derive(Debug)]
pub enum ConfigValue {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
pub struct Config {
    pub key: String,
    pub val: ConfigValue,
}

#[derive(Debug)]
pub struct StyledObject {
    pub obj: Object,
    pub config: Vec<Config>,
}

#[derive(Debug)]
pub struct DecorObject {
    pub obj: Object,
    pub decor: String,
    pub config: Vec<Config>,
}

pub type ConfigLine = Vec<Config>;
pub type Draw = Vec<StyledObject>;
pub type Decor = Vec<DecorObject>;
#[derive(Debug)]
pub struct Save(pub String);

#[derive(Debug)]
pub enum FileLine {
    Config(ConfigLine),
    Draw(Draw),
    Decor(Decor),
    Save(Save),
    Decl(Decl),
}

pub type Main = Vec<FileLine>;
