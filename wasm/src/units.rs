use serde::Serialize;
use unit_convert::units::{derived::DERIVED_UNITS, UNIT_SPACES};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Serialize)]
pub struct Unit {
    name: String,
    description: Option<String>,
    link: Option<String>,
    space: String,
    aliases: Vec<String>,
    metric: bool,
}

#[wasm_bindgen]
pub fn get_units() -> Vec<JsValue> {
    UNIT_SPACES
        .iter()
        .flat_map(|x| x.units.iter())
        .map(|unit| Unit {
            name: unit.name.to_owned(),
            description: unit.description.map(|x| x.to_owned()),
            link: unit.link.map(|x| x.to_owned()),
            space: unit.space.to_string(),
            aliases: unit.aliases.iter().map(|&x| x.to_owned()).collect(),
            metric: unit.metric,
        })
        .map(|x| serde_wasm_bindgen::to_value(&x).unwrap())
        .collect()
}

#[derive(Serialize)]
pub struct DerivedUnit {
    name: String,
    description: Option<String>,
    link: Option<String>,
    aliases: Vec<String>,
    metric: bool,
    derived_from: String,
}

#[wasm_bindgen]
pub fn get_derived_units() -> Vec<JsValue> {
    DERIVED_UNITS
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| DerivedUnit {
            name: x.name.to_owned(),
            description: x.description.map(|x| x.to_owned()),
            link: x.link.map(|x| x.to_owned()),
            aliases: x.aliases.iter().map(|&x| x.to_owned()).collect(),
            metric: x.metric,
            derived_from: stringify_units(&x.expand),
        })
        .map(|x| serde_wasm_bindgen::to_value(&x).unwrap())
        .collect()
}

fn stringify_units(units: &[unit_convert::dimension::Unit]) -> String {
    units
        .iter()
        .filter(|x| !x.is_special())
        .map(|x| x.to_string())
        .intersperse(" ".to_owned())
        .collect()
}
