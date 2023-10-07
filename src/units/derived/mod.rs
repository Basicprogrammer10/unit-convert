use std::fmt::Debug;

use super::{Conversion, Space};
use crate::dimension::Unit;

pub mod electric_charge;
pub mod energy;
pub mod force;
pub mod illuminance;
pub mod misc;
pub mod pressure;
pub mod radiation_dose;
pub mod radioactivity;
pub mod voltage;

pub const DERIVED_UNITS: &[&[&DerivedConversion]] = &[
    electric_charge::UNITS,
    energy::UNITS,
    force::UNITS,
    illuminance::UNITS,
    misc::UNITS,
    pressure::UNITS,
    radiation_dose::UNITS,
    radioactivity::UNITS,
    voltage::UNITS,
];

pub struct DerivedConversion {
    pub name: &'static str,
    pub expand: &'static [Unit],
    pub aliases: &'static [&'static str],
    pub metric: bool,
}

pub fn get(s: &str) -> Option<&'static DerivedConversion> {
    DERIVED_UNITS.iter().find_map(|space| {
        space
            .iter()
            .find(|unit| unit.name == s || unit.aliases.contains(&s))
            .copied()
    })
}

impl Debug for DerivedConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl PartialEq for DerivedConversion {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[macro_export]
macro_rules! impl_derived_units {
    {
        $(
            $(#[$meta:meta])*
            $name:ident => [
                <| $unit: expr
                $(, aliases = [$($aliases:expr),*])?
                $(, metric = $metric:expr)?
            ]
        ),*
    } => {
        use $crate::units::derived::DerivedConversion;
        pub const UNITS: &[&'static DerivedConversion] = &[$(&$name),*];

        $(
            $(#[$meta])*
            pub const $name: DerivedConversion = DerivedConversion {
                name: identconv::lower_strify!($name),
                expand: &$unit,
                aliases: &[$($($aliases),*)?],
                metric: false $(|| $metric)?
            };
        )*
    };
}

pub macro constant {
    ($conversion:literal, $exponent:literal) => {
        #[allow(clippy::excessive_precision)]
        Unit::new(
            &Conversion {
                name: "virtual-unit",
                space: Space::Dynamic,
                to_base: |x| x * $conversion,
                from_base: |x| x / $conversion,
                aliases: &[],
                metric: false,
                special: true,
            },
            1.0,
            0.0,
        )
    },
    ($conversion:literal) => {
        constant!($conversion, 0.0)
    }
}

#[macro_export]
macro_rules! concat_arrays_size {
    ($( $array:expr ),*) => {{
        0 $(+ $array.len())*
    }};
}

/// Modified from [array_concat](https://crates.io/crates/array-concat) crate.
#[macro_export]
macro_rules! join_arrays {
    ($($array:expr),*) => ({
        #[repr(C)]
        struct ArrayConcatDecomposed<T>($([T; $array.len()]),*);

        #[repr(C)]
        union ArrayConcatComposed<T, const N: usize> {
            full: core::mem::ManuallyDrop<[T; N]>,
            decomposed: core::mem::ManuallyDrop<ArrayConcatDecomposed<T>>,
        }

        const SIZE: usize = $crate::concat_arrays_size!($($array),*);
        let composed = ArrayConcatComposed::<_, SIZE> { decomposed: core::mem::ManuallyDrop::new(ArrayConcatDecomposed ( $($array),* ))};

        core::mem::ManuallyDrop::into_inner(unsafe { composed.full })
    })
}
