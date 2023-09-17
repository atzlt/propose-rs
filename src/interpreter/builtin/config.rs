use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::interpreter::utils::ConfigValue;

macro_rules! entry {
    ($key:literal, $val:literal) => {
        (String::from($key), $val.into())
    };
}

lazy_static! {
    pub static ref DEFAULT_CONFIG: HashMap<String, ConfigValue> = HashMap::from([
        entry!("width", 10.0),
        entry!("height", 10.0),
        entry!("color", "#000000"),
        entry!("fill", "none"),
        entry!("linewidth", 1.5),
        entry!("dotsize", 2.5),
        entry!("dotstroke", "#000000"),
        entry!("dotfill", "#000000"),
        entry!("dotwidth", 0.0),
        entry!("labelsize", 15.0),
        entry!("dist", 10.0),
        entry!("angle", 0.0),
        entry!("anglesize", 20.0),
        entry!("anglecolor", "#000000"),
        entry!("anglewidth", 1.5),
        entry!("decorsize", 5.0),
        entry!("decorwidth", 1.5),
        entry!("decorcolor", "#000000"),
        entry!("decorfill", "none"),
        entry!("loc", 0.5),
        entry!("font", "serif"),
    ]);
}
