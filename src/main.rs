use std::str::FromStr;

use anyhow::{bail, Result};
use clap::Parser;
use thousands::Separable;

use unit_convert::{args::Args, dimension::Dimensions, input};

fn main() -> Result<()> {
    let args = Args::parse();
    let inp = input::Input::from_str(&args.input)?;

    let from_dim = Dimensions::from_str(&inp.from_unit)?;
    let to_dim = Dimensions::from_str(&inp.to_unit)?;

    if from_dim != to_dim {
        println!("{}\n{}", from_dim, to_dim);
        bail!("Unit dimensions do not match.");
    } else if args.dimensions {
        println!("{:#}\n", from_dim.simplify())
    }

    let val = from_dim.convert(&to_dim, inp.value, args.debug)?;
    println!(
        "{} {} => {} {}",
        inp.value.separate_with_spaces(),
        inp.from_unit,
        val.separate_with_spaces(),
        inp.to_unit
    );
    Ok(())
}
