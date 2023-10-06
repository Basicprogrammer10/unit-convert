use std::fmt::Debug;

use crate::{dimension::Unit, Num};

use super::{length, mass, time, Conversion, Space};

pub const DERIVED_UNITS: &[&'static dyn DerivedConversion] = &[&Newton, &Joule, &PoundForce, &Hz];

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

macro_rules! impl_derived_units {
    {
        $(
            $(#[$meta:meta])?
            $name:ident => [
                <| [$($unit:expr),*]
                $(, aliases = [$($aliases:expr),*])?
                $(, metric = $metric:expr)?
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

                fn is_metric(&self) -> bool {
                    false
                    $(;$metric)?
                }
            }
        )*
    };
}

impl_derived_units! {
    /// `s^{-1}`
    Hz => [
        <| [
            Unit::new(&time::Second, -1.0, 0.0)
        ],
        metric = true
    ],
    /// `kg*m*s^{−2}`
    Newton => [
        <| [
            Unit::new(&mass::Gram, 1.0, 3.0),
            Unit::new(&length::Meter, 1.0, 0.0),
            Unit::new(&time::Second, -2.0, 0.0)
        ],
        aliases = ["N"],
        metric = true
    ],
    /// `kg*m^2*s^{−2}`
    Joule => [
        <| [
            Unit::new(&mass::Gram, 1.0, 3.0),
            Unit::new(&length::Meter, 2.0, 0.0),
            Unit::new(&time::Second, -2.0, 0.0)
        ],
        aliases = ["J"],
        metric = true
    ],
    /// `4.448222*N`
    PoundForce => [
        <| [
            Unit::new(&VarNum::new(4.448222), 1.0, 0.0),
            Unit::new(&mass::Gram, 1.0, 3.0),
            Unit::new(&length::Meter, 1.0, 0.0),
            Unit::new(&time::Second, -2.0, 0.0)
        ],
        aliases = ["lbf"],
        metric = true
    ]
}
