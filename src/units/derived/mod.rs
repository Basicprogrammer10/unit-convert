use std::fmt::Debug;

use crate::dimension::Unit;

use super::{length, mass, time};

pub const DERIVED_UNITS: &[&'static dyn DerivedConversion] = &[&Newton];

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

// TODO: impl is_metric
macro_rules! impl_derived_units {
    {
        $(
            $(#[$meta:meta])?
            $name:ident => [
                <| [$($unit:expr),*]
                $(, aliases = [$($aliases:expr),*])?
            ]
        ),*
    } => {
        $(
            $(#[$meta])*
            pub struct $name;

            impl DerivedConversion for $name {
                fn name(&self) -> &'static str {
                    identconv::lower_strify!($name)
                }

                fn expand(&self) -> &'static [Unit] {
                    const UNITS: &'static [Unit] = &[$($unit),*];
                    UNITS
                }

                fn aliases(&self) -> &'static [&'static str] {
                    &[$($($aliases),*)?]
                }
            }
        )*
    };
}

impl_derived_units! {
    /// `kg*m*s^{−2}`
    Newton => [
        <| [
            Unit::new(&mass::Gram, 3.0, 1.0),
            Unit::new(&length::Meter, 1.0, 1.0),
            Unit::new(&time::Second, -2.0, 0.0)
        ],
        aliases = ["n"]
    ]
}