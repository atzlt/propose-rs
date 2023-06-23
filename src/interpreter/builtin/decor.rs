use crate::{
    interpreter::draw::{decor::DecorConfig, CM},
    write_line,
};
use lazy_static::lazy_static;
use metric_rs::objects::Point;
use std::fmt::Write;
use std::{collections::HashMap, f64::consts::PI};

type DecorFunction = fn(DecorConfig) -> String;

macro_rules! entry {
    ($key:literal, $body:expr) => {
        ($key, ($body) as _)
    };
}

lazy_static! {
    pub static ref DECORATIONS: HashMap<&'static str, DecorFunction> = HashMap::from([
        entry!("|", |DecorConfig {
                         pos,
                         size,
                         angle,
                         width,
                         color,
                         fill: _,
                     }| {
            let offset = Point::new(-angle.sin() * size, angle.cos() * size);
            let pos = pos * CM;
            let p1 = pos + offset;
            let p2 = pos - offset;
            let mut string = String::new();
            write_line!(string, p1, p2 => in px, color, width, "").unwrap();
            string
        }),
        entry!("||", |DecorConfig {
                          pos,
                          size,
                          angle,
                          width,
                          color,
                          fill: _,
                      }| {
            let sin = angle.sin();
            let cos = angle.cos();
            let offset = Point::new(-sin * size, cos * size);
            let gap = Point::new(cos * size / 3.0, sin * size / 3.0);
            let pos = pos * CM;
            let mut string = String::new();
            write_line!(
                string,
                pos - gap + offset,
                pos - gap - offset => in px,
                color,
                width,
                ""
            )
            .unwrap();
            write_line!(
                string,
                pos + gap + offset,
                pos + gap - offset => in px,
                color,
                width,
                ""
            )
            .unwrap();
            string
        }),
        entry!(">", |DecorConfig {
                         pos,
                         size,
                         angle,
                         width,
                         color,
                         fill: _,
                     }| {
            let offset1 = Point::new(angle.cos() * size, angle.sin() * size);
            let offset2 = Point::new(
                (angle + PI * 2.0 / 3.0).cos() * size,
                (angle + PI * 2.0 / 3.0).sin() * size,
            );
            let offset3 = Point::new(
                (angle - PI * 2.0 / 3.0).cos() * size,
                (angle - PI * 2.0 / 3.0).sin() * size,
            );
            let pos = pos * CM;
            let mut string = String::new();
            write_line!(string, pos + offset1, pos + offset2 => in px, color, width, "").unwrap();
            write_line!(string, pos + offset1, pos + offset3 => in px, color, width, "").unwrap();
            string
        })
    ]);
}
