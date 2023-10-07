use std::fmt::Debug;

use super::{Conversion, Space};
use crate::{dimension::Unit, Num};

pub mod electric_charge;
pub mod energy;
pub mod force;
pub mod misc;
pub mod pressure;
pub mod voltage;

pub const DERIVED_UNITS: &[&[&'static dyn DerivedConversion]] = &[
    misc::UNITS,
    force::UNITS,
    &pressure::UNITS,
    &energy::UNITS,
    &electric_charge::UNITS,
    &voltage::UNITS,
];

pub trait DerivedConversion {
    fn name(&self) -> &'static str;
    fn expand(&self) -> &'static [Unit];

    fn is_metric(&self) -> bool {
        false
    }
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }
}

pub struct VarNum {
    multiplier: Num,
}

impl VarNum {
    pub const fn new(multiplier: Num) -> Self {
        Self { multiplier }
    }
}

impl Conversion for VarNum {
    fn name(&self) -> &'static str {
        "varnum"
    }

    fn space(&self) -> Space {
        Space::Quantity
    }

    fn to_base(&self, this: Num) -> Num {
        this * self.multiplier
    }

    fn from_base(&self, s: Num) -> Num {
        s / self.multiplier
    }
}

pub fn get(s: &str) -> Option<&'static dyn DerivedConversion> {
    DERIVED_UNITS.iter().find_map(|space| {
        space
            .iter()
            .find(|unit| unit.name() == s || unit.aliases().contains(&s))
            .copied()
    })
}

impl Debug for dyn DerivedConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl PartialEq for dyn DerivedConversion {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
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
        use crate::units::derived::DerivedConversion;
        pub const UNITS: &[&'static dyn DerivedConversion] = &[$(&$name),*];

        $(
            $(#[$meta])*
            pub struct $name;

            impl DerivedConversion for $name {
                fn name(&self) -> &'static str {
                    identconv::lower_strify!($name)
                }

                fn expand(&self) -> &'static [Unit] {
                    const UNITS: &[Unit] = &$unit;
                    UNITS
                }

                fn aliases(&self) -> &'static [&'static str] {
                    &[$($($aliases),*)?]
                }

                fn is_metric(&self) -> bool {
                    false
                    $(;$metric)?
                }
            }
        )*
    };
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
