pub mod decor;
pub mod label;
pub mod render;

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

#[macro_export]
macro_rules! write_arc {
    ($str:ident, $from:expr, $r: expr, $large_arc:expr, $sweep:expr, $to:expr, $color:expr, $width:expr, $dash:expr) => {
        write!(
            $str,
            "<path d=\"M {},{} A {} {} 0 {} {} {},{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
            $from.x * CM,
            -$from.y * CM,
            $r * CM,
            $r * CM,
            if $large_arc { 1 } else { 0 },
            if $sweep { 0 } else { 1 },
            $to.x * CM,
            -$to.y * CM,
            $color,
            $width,
            $dash
        )
    };
    ($str:ident, in px: $from:expr, $r: expr, $large_arc:expr, $sweep:expr, $to:expr, $color:expr, $width:expr, $dash:expr) => {
        write!(
            $str,
            "<path d=\"M {},{} A {} {} 0 {} {} {},{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"{}/>",
            $from.x,
            -$from.y,
            $r,
            $r,
            if $large_arc { 1 } else { 0 },
            if $sweep { 0 } else { 1 },
            $to.x,
            -$to.y,
            $color,
            $width,
            $dash
        )
    };
}

#[macro_export]
macro_rules! write_polygon {
    ($str:ident, $pts:ident, $fill:expr) => {
        write!($str, "<polygon points=\"{}\" fill=\"{}\"/>", $pts, $fill)
    };
}

#[macro_export]
macro_rules! write_polyline {
    ($str:ident, $pts:expr, $color:expr, $width:expr) => {
        write!(
            $str,
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"/>",
            $pts, $color, $width,
        )
    };
}
