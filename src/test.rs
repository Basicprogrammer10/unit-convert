use std::str::FromStr;

use anyhow::Result;
use approx::assert_abs_diff_eq;
use hashbrown::HashSet;

use crate::{
    dimension::Dimensions,
    input,
    units::{derived::DERIVED_UNITS, UNIT_SPACES},
    Num,
};

const ERROR: Num = 0.01;

fn convert(inp: &str) -> Result<Num> {
    let inp = input::Input::from_str(&inp)?;

    let from_dim = Dimensions::from_str(&inp.from_unit)?;
    let to_dim = Dimensions::from_str(&inp.to_unit)?;

    let val = from_dim.convert(&to_dim, inp.value, true)?;
    Ok(val)
}

macro_rules! tests {
    ($(
        $id:ident => [
            $($test:expr => $res:expr),*
        ]
    ),*) => {
        paste::paste! {
            $(
                #[test]
                fn [<test_convert_ $id>]() {
                    $(
                        assert_abs_diff_eq!(convert($test).unwrap(), $res, epsilon = ERROR);
                    )*
                }
            )*
        }
    };
}

tests! {
    basic => [
        "10m/s => mi/h" => 22.37,
        "10m/s => cm/s" => 1000.0,
        "10m/s^2 => mi/h^2" => 80529.71,
        "10 m/s^3 => yard/s^3" => 10.94,
        "10 yard/ms^2 => feet/s^2" => 30000.0
    ],
    derived => [
        "1E2 footcandle to hefnerkerze*rad/ft^2" => 110.74,
        "10 footcandle => lux" => 107.64
    ],
    shorthand => [
        "10 km/h => mph" => 6.21,
        "50 Wh => J" => 180000.0
    ],
        prefix => [
        "50 kWh => J" => 180000000.0,
        "30 kHz => Hz" => 0.03,
        "30 kHz => GHz" => 30000000.0
    ]
}

#[test]
fn test_name_collisions() {
    let mut sack = HashSet::new();
    let mut collisions = HashSet::new();

    for space in UNIT_SPACES {
        for unit in space.units {
            if !sack.insert(unit.name) {
                collisions.insert(unit.name);
            }

            for alias in unit.aliases {
                if !sack.insert(alias) {
                    collisions.insert(alias);
                }
            }
        }
    }

    for space in DERIVED_UNITS {
        for unit in space.iter() {
            if !sack.insert(unit.name) {
                collisions.insert(unit.name);
            }

            for alias in unit.aliases {
                if !sack.insert(alias) {
                    collisions.insert(alias);
                }
            }
        }
    }

    assert!(collisions.is_empty(), "name collisions: {:?}", collisions);
}
