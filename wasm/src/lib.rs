#![feature(iter_intersperse)]

use anyhow::bail;
use unit_convert::{self, dimension::Dimensions, input};
use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

pub mod units;

#[wasm_bindgen]
pub fn version() -> String {
    unit_convert::VERSION.to_owned()
}

#[wasm_bindgen]
pub fn convert(input: &str) -> Result<f64, String> {
    fn inner(input: &str) -> anyhow::Result<f64> {
        let inp = input::Input::from_str(input)?;

        let from_dim = Dimensions::from_str(&inp.from_unit)?;
        let to_dim = Dimensions::from_str(&inp.to_unit)?;

        if from_dim != to_dim {
            bail!(
                "Unit dimensions do not match. ({:#} vs {:#})",
                from_dim.simplify(),
                to_dim.simplify()
            );
        }

        from_dim.convert(&to_dim, inp.value, false)
    }

    inner(input).map_err(|x| x.to_string())
}

// Note to self: build with `wasm-pack build --target web`
