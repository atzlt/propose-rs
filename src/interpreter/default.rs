use std::collections::HashMap;

use lazy_static::lazy_static;

use super::ast::ConfigValue;

macro_rules! entry {
    ($key:literal, $val:literal) => {
        (String::from($key), $val.into())
    };
}

lazy_static! {
    pub(super) static ref DEFAULT_CONFIG: HashMap<String, ConfigValue> = HashMap::from([
        entry!("width", 10.0),
        entry!("height", 10.0),
        entry!("color", "#000000"),
        entry!("fill", "#00000000"),
        entry!("linewidth", 1.5),
        entry!("dotsize", 2.5),
        entry!("dotstroke", "#000000"),
        entry!("dotfill", "#000000"),
        entry!("dotwidth", 0.0),
        entry!("labelsize", 15.0),
        entry!("dist", 10.0),
        entry!("angle", 0.0),
        entry!("loc", 0.5),
        entry!("font", "serif"),
    ]);
}
