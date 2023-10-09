use std::fmt::Debug;

use crate::{
    units::{
        derived::{energy, DerivedConversion},
        length, time, Conversion, ConversionType,
    },
    Num,
};

pub const SHORTHANDS: &[Shorthand] = &[
    Shorthand::new_metric(
        "Wh",
        &[
            HandUnit::new_derived(&energy::WATT, 1.0, 0.0),
            HandUnit::new(&time::HOUR, 1.0, 0.0),
        ],
    ),
    Shorthand::new(
        "mph",
        &[
            HandUnit::new(&length::MILE, 1.0, 0.0),
            HandUnit::new(&time::HOUR, -1.0, 0.0),
        ],
    ),
];

#[derive(Debug, Clone)]
pub struct Shorthand {
    pub name: &'static str,
    pub unit: &'static [HandUnit],
    pub metric: bool,
}

#[derive(Debug)]
pub struct HandUnit {
    pub conversion: ConversionType,
    pub power: Num,
    pub sci_exponent: Num,
}

#[inline]
pub fn get(s: &str) -> Option<&'static Shorthand> {
    SHORTHANDS.iter().find(|u| u.name == s)
}

impl Shorthand {
    const fn new(name: &'static str, unit: &'static [HandUnit]) -> Self {
        Self {
            name,
            unit,
            metric: false,
        }
    }

    const fn new_metric(name: &'static str, unit: &'static [HandUnit]) -> Self {
        Self {
            name,
            unit,
            metric: true,
        }
    }
}

impl HandUnit {
    const fn new(conversion: &'static Conversion, power: f64, sci_exponent: f64) -> Self {
        Self {
            conversion: ConversionType::Conversion(conversion),
            power,
            sci_exponent,
        }
    }

    const fn new_derived(
        conversion: &'static DerivedConversion,
        power: f64,
        sci_exponent: f64,
    ) -> Self {
        Self {
            conversion: ConversionType::DerivedConversion(conversion),
            power,
            sci_exponent,
        }
    }
}
