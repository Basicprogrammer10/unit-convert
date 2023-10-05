use unit_convert::{self, dimension::Dimensions, input};
use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

#[wasm_bindgen]
pub fn convert(input: &str) -> Option<f64> {
    let inp = input::Input::from_str(input).ok()?;

    let from_dim = Dimensions::from_str(&inp.from_unit).ok()?;
    let to_dim = Dimensions::from_str(&inp.to_unit).ok()?;

    assert_eq!(from_dim, to_dim, "Unit dimensions do not match.");
    from_dim.convert(&to_dim, inp.value, false).ok()
}
