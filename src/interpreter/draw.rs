pub mod dobjects;
pub mod label;
pub mod decor;

pub const CM: f64 = 37.795;

#[macro_export]
macro_rules! write_line {
    ($str:ident, $a:expr, $b:expr => in px, $color:expr, $width:expr, $dash:expr) => {
        write!(
            $str,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
            $a.x,
            -$a.y,
            $b.x,
            -$b.y,
            $color,
            $width,
            $dash,
        )
    };
    ($str:ident, $a:expr, $b:expr, $color:expr, $width:expr, $dash:expr) => {
        write!(
            $str,
            "<line x1=\"{}cm\" y1=\"{}cm\" x2=\"{}cm\" y2=\"{}cm\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
            $a.x,
            -$a.y,
            $b.x,
            -$b.y,
            $color,
            $width,
            $dash,
        )
    };
}

#[macro_export]
macro_rules! write_circle {
    ($str:ident, $pos:expr, $r:expr => in px, $color:expr, $fill:expr, $width:expr, $dash:expr) => {
        write!(
            $str,
            "<circle cx=\"{}cm\" cy=\"{}cm\" r=\"{}\" stroke=\"{}\" fill=\"{}\" stroke-width=\"{}\"{}/>",
            $pos.x,
            -$pos.y,
            $r,
            $color,
            $fill,
            $width,
            $dash,
        )
    };
    ($str:ident, $pos:expr, $r:expr, $color:expr, $fill:expr, $width:expr, $dash:expr) => {
        write!(
            $str,
            "<circle cx=\"{}cm\" cy=\"{}cm\" r=\"{}cm\" stroke=\"{}\" fill=\"{}\" stroke-width=\"{}\"{}/>",
            $pos.x,
            -$pos.y,
            $r,
            $color,
            $fill,
            $width,
            $dash,
        )
    };
}
