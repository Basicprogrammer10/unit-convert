use std::fmt::{Debug, Display};

use crate::{
    shorthand::{self, Shorthand},
    Num,
};

use self::derived::DerivedConversion;

pub mod angle;
pub mod derived;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod quantity;
pub mod temperature;
pub mod time;

pub const UNIT_SPACES: &[&UnitSpace] = &[
    &angle::ANGLE,
    &electric_current::ELECTRIC_CURRENT,
    &length::LENGTH,
    &luminous_intensity::LUMINOUS_INTENSITY,
    &mass::MASS,
    &quantity::QUANTITY,
    &temperature::TEMPERATURE,
    &time::TIME,
];

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Space {
    Angle,
    ElectricCurrent,
    Length,
    LuminousIntensity,
    Mass,
    Quantity,
    Temperature,
    Time,
    /// A special unit space for dynamic units.
    /// This is just used to hack in support for derived units that have arbitrary multipliers.
    Dynamic,
}

#[derive(Debug, Clone)]
pub enum ConversionType {
    Conversion(&'static Conversion),
    DerivedConversion(&'static DerivedConversion),
    Shorthand(&'static Shorthand),
}

pub struct UnitSpace {
    /// Gets the name of the unit space.
    pub name: &'static str,
    /// The Space enum of the unit space.
    pub space: Space,
    /// All of the units in the space.
    pub units: &'static [&'static Conversion],
}

pub struct Conversion {
    /// Gets the name of the unit.
    pub name: &'static str,
    /// Gets the unit space of the unit.
    pub space: Space,
    /// Converts a value in this unit to the unit space's base unit.
    pub to_base: fn(Num) -> Num,
    /// Converts a value in the unit space's base unit to this unit.
    pub from_base: fn(Num) -> Num,
    /// Gets the aliases of the unit.
    pub aliases: &'static [&'static str],
    /// Checks if the unit is a metric unit.
    /// Metric units can use metric prefixes.
    pub metric: bool,
    /// Checks if the unit is a special unit, which are virtual just used to hack in support for constants in derived units.
    pub special: bool,
}

impl UnitSpace {
    pub fn get(&self, name: &str) -> Option<&'static Conversion> {
        self.units.iter().find(|u| u.is_alias(name)).copied()
    }
}

impl Conversion {
    /// Checks if the given name is the name or an alias of this unit.
    pub fn is_alias(&self, name: &str) -> bool {
        self.name == name.to_ascii_lowercase() || self.aliases.contains(&name)
    }
}

impl ConversionType {
    pub fn as_conversion(&self) -> Option<&'static Conversion> {
        if let ConversionType::Conversion(conversion) = self {
            return Some(*conversion);
        }

        None
    }

    pub fn is_metric(&self) -> bool {
        match self {
            ConversionType::Conversion(c) => c.metric,
            ConversionType::DerivedConversion(c) => c.metric,
            ConversionType::Shorthand(c) => c.metric,
        }
    }
}

// todo: cleanup with unit::get
pub fn find_unit(s: &str) -> Option<ConversionType> {
    UNIT_SPACES
        .iter()
        .find_map(|space| space.get(s).map(ConversionType::Conversion))
        .or_else(|| derived::get(s).map(ConversionType::DerivedConversion))
        .or_else(|| shorthand::get(s).map(ConversionType::Shorthand))
}

impl Display for UnitSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Space::Angle => "angle",
            Space::ElectricCurrent => "electric current",
            Space::Length => "length",
            Space::LuminousIntensity => "luminous intensity",
            Space::Mass => "mass",
            Space::Quantity => "quantity",
            Space::Temperature => "temperature",
            Space::Time => "time",
            Space::Dynamic => "dynamic",
        })
    }
}

impl Display for Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.name)
    }
}

impl Debug for Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl PartialEq for Conversion {
    fn eq(&self, other: &Self) -> bool {
        // ew
        // todo: make this less bad
        self.name == other.name
    }
}

impl PartialEq for ConversionType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Conversion(l0), Self::Conversion(r0)) => l0 == r0,
            (Self::DerivedConversion(l0), Self::DerivedConversion(r0)) => l0 == r0,
            (Self::Shorthand(..), Self::Shorthand(..)) => unreachable!(),
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! impl_units {
    ($space:ident => {
        $(
            $(#[$meta:meta])?
            $struct:ident => [
                <| $to_base:expr,
                |> $from_base:expr
                $(, aliases = [$($aliases:expr),*])?
                $(, metric = $metric:expr)?
            ]
        ),*
    }) => {
        use $crate::units::{Conversion, Space, UnitSpace};
        use identconv::lower_strify;

        pub const $space: UnitSpace = UnitSpace {
            name: lower_strify!($space),
            space: paste::expr! { Space::[< $space:camel >] },
            units: &[$(&$struct,)*]
        };

        $(
            $(#[$meta])?
            pub const $struct: Conversion = Conversion {
                name: lower_strify!($struct),
                space: paste::expr! { Space::[< $space:camel >] },
                to_base: $to_base,
                from_base: $from_base,
                aliases: &[$($($aliases),*)?],
                metric: false $(|| $metric)?,
                special: false
            };
        )*
    };
}
