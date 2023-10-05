use anyhow::bail;
use unit_convert::{self, dimension::Dimensions, input};
use wasm_bindgen::prelude::wasm_bindgen;

use std::str::FromStr;

#[wasm_bindgen]
pub fn convert(input: &str) -> Result<f64, String> {
    fn convert_inner(input: &str) -> anyhow::Result<f64> {
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

        Ok(from_dim.convert(&to_dim, inp.value, false)?)
    }

    Ok(convert_inner(input).map_err(|x| x.to_string())?)
}
