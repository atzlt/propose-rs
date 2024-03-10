pub mod ast;

use super::utils::ConfigValue;
use ast::*;
use pest_consume::Parser;
use pest_consume::{match_nodes, Error};
use std::f64::consts::PI;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "interpreter/parser.pest"]
struct ProposeParser;

#[inline]
#[allow(clippy::result_large_err)]
pub fn parse(src: &str) -> Result<Main> {
    let inputs = ProposeParser::parse(Rule::main, src)?;
    let input = inputs.single()?;
    ProposeParser::main(input)
}

#[pest_consume::parser]
impl ProposeParser {
    #[inline]
    fn point_id(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }
    #[inline]
    fn common_id(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }
    #[inline]
    fn line_2p(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b)] => Ok(Object::Line2P(a, b))
        )
    }
    #[inline]
    fn circ_3p(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b), point_id(c)] => Ok(Object::Circ3P(a, b, c))
        )
    }
    #[inline]
    fn circ_or(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), numeric(b)] => Ok(Object::CircOr(a, Box::new(b))),
        )
    }
    #[inline]
    fn circ_oa(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b)] => Ok(Object::CircOA(a, b))
        )
    }
    #[inline]
    fn circ_diam(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b)] => Ok(Object::CircDiam(a, b))
        )
    }
    #[inline]
    fn trig(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b), point_id(c)] => Ok(Object::Triangle(a, b, c))
        )
    }
    #[inline]
    fn arc(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b), point_id(c)] => Ok(Object::Arc(a, b, c))
        )
    }
    #[inline]
    fn arc_o(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b), point_id(c)] => Ok(Object::ArcO(a, b, c))
        )
    }
    #[inline]
    fn polygon(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [point_id(p)..] => Ok(Object::Polygon(p.collect()))
        )
    }
    #[inline]
    fn linear(input: Node) -> Result<Linear> {
        match_nodes!(
            input.into_children();
            [common_id(a)] => Ok(Linear::Name(a)),
            [line_2p(a)] => {
                if let Object::Line2P(l, k) = a {
                    Ok(Linear::Line2P(l, k))
                } else {
                    unreachable!()
                }
            }
        )
    }
    #[inline]
    fn distance(input: Node) -> Result<Numeric> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b)] => Ok(Numeric::Distance2P(a, b)),
            [point_id(a), linear(b)] => Ok(Numeric::DistancePL(a, b)),
            [linear(a), linear(b)] => Ok(Numeric::Distance2L(a, b)),
        )
    }
    #[inline]
    fn angle_3p(input: Node) -> Result<(String, String, String)> {
        match_nodes!(
            input.into_children();
            [point_id(a), point_id(b), point_id(c)] => Ok((a, b, c))
        )
    }
    #[inline]
    fn angle_2l(input: Node) -> Result<Numeric> {
        match_nodes!(
            input.into_children();
            [linear(a), linear(b)] => Ok(Numeric::Angle2L(a, b))
        )
    }
    #[inline]
    fn number(input: Node) -> Result<f64> {
        input
            .as_str()
            .to_string()
            .parse::<f64>()
            .map_err(|e| input.error(e))
    }
    #[inline]
    fn degree(input: Node) -> Result<f64> {
        let mut x = input.as_str().to_string();
        x.pop();
        x.pop();
        x.pop();
        let x = x.parse::<f64>().map_err(|e| input.error(e))?;
        Ok(x * PI / 180.0)
    }
    #[inline]
    fn rich_number(input: Node) -> Result<f64> {
        match_nodes!(
            input.into_children();
            [number(a)] => Ok(a),
            [degree(a)] => Ok(a),
        )
    }
    #[inline]
    fn numeric(input: Node) -> Result<Numeric> {
        match_nodes!(
            input.into_children();
            [distance(a)] => Ok(a),
            [angle_3p((a, b, c))] => Ok(Numeric::Angle3P(a, b, c)),
            [angle_2l(a)] => Ok(a),
            [rich_number(a)] => Ok(Numeric::Number(a)),
            [common_id(a)] => Ok(Numeric::Name(a)),
        )
    }
    #[inline]
    fn boolean(input: Node) -> Result<bool> {
        match input.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(input.error("Not a bool")),
        }
    }
    #[inline]
    fn str_inner(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }
    #[inline]
    fn string(input: Node) -> Result<String> {
        Self::str_inner(input.into_children().next().unwrap())
    }
    #[inline]
    fn ortho_coord(input: Node) -> Result<DeclRight> {
        match_nodes!(
            input.into_children();
            [numeric(a), numeric(b)] => Ok(DeclRight::OrthoCoord(Box::new(a), Box::new(b)))
        )
    }
    #[inline]
    fn polar_coord(input: Node) -> Result<DeclRight> {
        match_nodes!(
            input.into_children();
            [numeric(a), numeric(b)] => Ok(DeclRight::PolarCoord(Box::new(a), Box::new(b)))
        )
    }
    #[inline]
    fn coord(input: Node) -> Result<DeclRight> {
        match_nodes!(
            input.into_children();
            [ortho_coord(a)] => Ok(a),
            [polar_coord(b)] => Ok(b),
        )
    }
    #[inline]
    fn eval(input: Node) -> Result<Object> {
        let string = input
            .as_str()
            .strip_prefix('$')
            .unwrap()
            .strip_suffix('$')
            .unwrap();
        Ok(Object::Eval(string.to_string()))
    }
    #[inline]
    fn any_id(input: Node) -> Result<String> {
        match_nodes!(
            input.into_children();
            [point_id(a)] => Ok(a),
            [common_id(a)] => Ok(a),
        )
    }
    #[inline]
    fn common_obj(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [line_2p(a)] => Ok(a),
            [circ_3p(a)] => Ok(a),
            [circ_or(a)] => Ok(a),
            [circ_oa(a)] => Ok(a),
            [circ_diam(a)] => Ok(a),
            [any_id(a)] => Ok(Object::Name(a))
        )
    }
    #[inline]
    fn arg(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [trig(a)] => Ok(a),
            [common_obj(a)] => Ok(a),
            [numeric(a)] => Ok(Object::Numeric(Box::new(a))),
            [eval(a)] => Ok(a),
        )
    }
    #[inline]
    fn draw_obj(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [polygon(a)] => Ok(a),
            [arc(a)] => Ok(a),
            [arc_o(a)] => Ok(a),
            [angle_3p((a, b, c))] => Ok(Object::Angle3P(a, b, c)),
            [common_obj(a)] => Ok(a),
        )
    }
    #[inline]
    fn decor_obj(input: Node) -> Result<Object> {
        match_nodes!(
            input.into_children();
            [polygon(a)] => Ok(a),
            [angle_3p((a, b, c))] => Ok(Object::Angle3P(a, b, c)),
            [arc(a)] => Ok(a),
            [arc_o(a)] => Ok(a),
            [common_obj(a)] => Ok(a),
        )
    }
    #[inline]
    fn method(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }
    #[inline]
    fn args(input: Node) -> Result<Vec<Object>> {
        input.into_children().map(Self::arg).collect()
    }
    #[inline]
    fn expr(input: Node) -> Result<DeclRight> {
        match_nodes!(
            input.into_children();
            [method(a), args(b)] => Ok(DeclRight::Expr(a, b))
        )
    }
    #[inline]
    fn decl_right(input: Node) -> Result<DeclRight> {
        match_nodes!(
            input.into_children();
            [coord(a)] => Ok(a),
            [expr(a)] => Ok(a),
            [arg(a)] => Ok(DeclRight::Object(Box::new(a))),
            [eval(a)] => Ok(DeclRight::Object(Box::new(a))),
        )
    }
    #[inline]
    fn direct(input: Node) -> Result<DeclLeft> {
        match_nodes!(
            input.into_children();
            [any_id(a)] => Ok(DeclLeft::Direct(a))
        )
    }
    #[inline]
    fn destruct(input: Node) -> Result<DeclLeft> {
        match_nodes!(
            input.into_children();
            [any_id(a), any_id(b)] => Ok(DeclLeft::Destruct(a, b))
        )
    }
    #[inline]
    fn decl_left(input: Node) -> Result<DeclLeft> {
        match_nodes!(
            input.into_children();
            [direct(a)] => Ok(a),
            [destruct(a)] => Ok(a),
        )
    }
    #[inline]
    fn decl(input: Node) -> Result<Decl> {
        match_nodes!(
            input.into_children();
            [decl_left(a), decl_right(b)] => Ok(Decl(a, b))
        )
    }
    #[inline]
    fn config_value(input: Node) -> Result<ConfigValue> {
        match_nodes!(
            input.into_children();
            [rich_number(a)] => Ok(ConfigValue::Number(a)),
            [boolean(a)] => Ok(ConfigValue::Bool(a)),
            [string(a)] => Ok(ConfigValue::String(a)),
        )
    }
    #[inline]
    fn config_name(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }
    #[inline]
    fn config(input: Node) -> Result<(String, ConfigValue)> {
        match_nodes!(
            input.into_children();
            [config_name(key), config_value(val)] => Ok((key, val))
        )
    }
    #[inline]
    fn configs(input: Node) -> Result<Config> {
        let items = input.into_children().flat_map(Self::config);
        let mut map = Config::new();
        for (key, val) in items {
            map.insert(key, val);
        }
        Ok(map)
    }
    #[inline]
    fn config_line(input: Node) -> Result<Config> {
        Self::configs(input.into_children().single().unwrap())
    }
    #[inline]
    fn draw_step(input: Node) -> Result<StyledObject> {
        match_nodes!(
            input.into_children();
            [draw_obj(obj), configs(config)] => Ok(StyledObject { obj, config: Some(config) }),
            [draw_obj(obj)] => Ok(StyledObject { obj, config: None })
        )
    }
    #[inline]
    fn draw(input: Node) -> Result<Draw> {
        input.into_children().map(Self::draw_step).collect()
    }
    #[inline]
    fn decoration(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }
    #[inline]
    fn decor_step(input: Node) -> Result<DecorObject> {
        match_nodes!(
            input.into_children();
            [decor_obj(obj), decoration(decor), configs(config)] => Ok(DecorObject { obj, decor, config: Some(config) }),
            [decor_obj(obj), decoration(decor)] => Ok(DecorObject { obj, decor, config: None })
        )
    }
    #[inline]
    fn decor(input: Node) -> Result<Decor> {
        input.into_children().map(Self::decor_step).collect()
    }
    #[inline]
    fn file_line(input: Node) -> Result<FileLine> {
        match_nodes!(
            input.into_children();
            [config_line(a)] => Ok(FileLine::Config(a)),
            [draw(a)] => Ok(FileLine::Draw(a)),
            [decor(a)] => Ok(FileLine::Decor(a)),
            [decl(a)] => Ok(FileLine::Decl(Box::new(a))),
        )
    }
    #[inline]
    fn main(input: Node) -> Result<Main> {
        input.into_children().map(Self::file_line).collect()
    }
}
