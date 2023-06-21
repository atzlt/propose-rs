use std::collections::HashMap;

use lazy_static::lazy_static;
use metric_rs::{
    calc::{basic::*, construct::*, transform::*},
    objects::*,
};

use super::{utils::FuncErr, utils::GObject};

macro_rules! ret_branch {
    ([$(<$var:ident>$param:ident),+] => <$ret:ident,None>$body:expr) => {
        return match $body {
            Err(e) => Err(FuncErr::CalcError(e)),
            Ok(x) => Ok((GObject::$ret(x), GObject::None)),
        }
    };
    ([$(<$var:ident>$param:ident),+] => <$ret1:ident,$ret2:ident>$body:expr) => {
        return match $body {
            Err(e) => Err(FuncErr::CalcError(e)),
            Ok((x, y)) => Ok((GObject::$ret1(x), GObject::$ret2(y))),
        }
    };
}

macro_rules! entry {
    ($name:literal; $([$(<$var:ident>$param:ident),+] => <$ret1:ident,$ret2:ident>$body:expr),+) => {
        (
            String::from($name),
            (|input: Vec<GObject>| {
                let slice = input.as_slice();
                $(
                    if let [$(GObject::$var($param)),+] = slice {
                        $(let $param = *$param;)+
                        ret_branch!([$(<$var>$param),+] => <$ret1, $ret2>$body);
                    }
                )+
                Err(FuncErr::ArgError)
            }) as _,
        )
    };
}

lazy_static! {
    pub(super) static ref FUNCTIONS: HashMap<String, fn(Vec<GObject>) -> Result<(GObject, GObject), FuncErr>> =
        HashMap::from([
            // Constructs
            entry!(
                "i";
                [<Line>l, <Line>k, <Point>p] => <Point, None>l.inter_common(k, p),
                [<Line>l, <Circle>c, <Point>p] => <Point, Point>l.inter_common(c, p),
                [<Circle>c, <Line>l, <Point>p] => <Point, Point>l.inter_common(c, p),
                [<Circle>c, <Circle>d, <Point>p] => <Point, Point>c.inter_common(d, p),
                [<Line>l, <Line>k] => <Point, None>l.inter(k),
                [<Line>l, <Circle>c] => <Point, Point>l.inter(c),
                [<Circle>c, <Line>l] => <Point, Point>l.inter(c),
                [<Circle>c, <Circle>d] => <Point, Point>c.inter(d)
            ),
            entry!(
                "perp";
                [<Point>p, <Line>l] => <Line, None>Ok(perp(p, l))
            ),
            entry!(
                "par";
                [<Point>p, <Line>l] => <Line, None>Ok(parallel(p, l))
            ),
            entry!(
                "proj";
                [<Point>p, <Line>l] => <Point, None>Ok(projection(p, l))
            ),
            entry!(
                "pb";
                [<Point>a, <Point>b] => <Line, None>perp_bisect(a, b)
            ),
            entry!(
                "ab";
                [<Point>a, <Point>o, <Point>b] => <Line, Line>angle_bisect_3p(a, o, b),
                [<Line>l, <Line>k] => <Line, Line>Ok(angle_bisect(l, k))
            ),
            entry!(
                "mid";
                [<Point>a, <Point>b] => <Point, None>Ok(midpoint(a, b))
            ),
            // Object creation
            entry!(
                "l";
                [<Point>a, <Point>b] => <Line, None>Line::from_2p(a, b),
                [<Number>a, <Number>b, <Number>c] => <Line, None>Line::from_coeff(a, b, c),
                [<Number>a, <Number>b, <Point>p] => <Line, None>Ok(Line::from_slope_and_point(a, b, p))
            )
        ]);
}
