use std::fmt::{Debug, Display};

use crate::Num;

use self::derived::{DerivedConversion, DERIVED_UNITS};
pub mod derived;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod quantity;
pub mod temperature;
pub mod time;

pub const UNIT_SPACES: &[&dyn UnitSpace] = &[
    &electric_current::ElectricCurrent,
    &length::Length,
    &luminous_intensity::LuminousIntensity,
    &mass::Mass,
    &quantity::Quantity,
    &temperature::Temperature,
    &time::Time,
];

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Space {
    ElectricCurrent,
    Length,
    LuminousIntensity,
    Mass,
    Quantity,
    Temperature,
    Time,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionType {
    Conversion(&'static dyn Conversion),
    DerivedConversion(&'static dyn DerivedConversion),
}

pub trait UnitSpace {
    /// Gets the name of the unit space.
    fn name(&self) -> &'static str;
    /// Gets the Space enum of the unit space.
    fn space(&self) -> Space;
    /// Gets all of the units in the space.
    fn units(&self) -> &'static [&'static dyn Conversion];

    /// Gets the unit with the given name.
    fn get(&self, name: &str) -> Option<&&dyn Conversion> {
        self.units().iter().find(|u| u.is_alias(name))
    }
}

pub trait Conversion {
    /// Gets the name of the unit.
    /// Must be lowercase.
    fn name(&self) -> &'static str;
    /// Gets the unit space of the unit.
    fn space(&self) -> Space;
    // todo: Use Num not a ref
    /// Converts a value in this unit to the unit space's base unit.
    fn to_base(&self, this: Num) -> Num;
    /// Converts a value in the unit space's base unit to this unit.
    #[allow(clippy::wrong_self_convention)]
    fn from_base(&self, s: Num) -> Num;

    /// Gets the aliases of the unit.
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }
    /// Checks if the unit is a metric unit.
    /// Metric units can use metric prefixes.
    fn is_metric(&self) -> bool {
        false
    }

    /// Checks if the given name is the name or an alias of this unit.
    fn is_alias(&self, name: &str) -> bool {
        self.name() == name || self.aliases().contains(&name)
    }
}

pub fn find_unit(s: &str) -> Option<ConversionType> {
    UNIT_SPACES
        .iter()
        .find_map(|space| space.get(s).map(|x| ConversionType::Conversion(*x)))
        .or_else(|| {
            DERIVED_UNITS
                .iter()
                .find(|x| x.name() == s || x.aliases().contains(&s))
                .map(|x| ConversionType::DerivedConversion(*x))
        })
}

impl ConversionType {
    pub fn as_conversion(&self) -> Option<&'static dyn Conversion> {
        if let ConversionType::Conversion(conversion) = self {
            return Some(*conversion);
        }

        None
    }

    pub fn is_metric(&self) -> bool {
        match self {
            ConversionType::Conversion(c) => c.is_metric(),
            ConversionType::DerivedConversion(c) => c.is_metric(),
        }
    }
}

impl Display for dyn UnitSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl Display for dyn Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.name())
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Space::ElectricCurrent => "electric current",
            Space::Length => "length",
            Space::LuminousIntensity => "luminous intensity",
            Space::Mass => "mass",
            Space::Quantity => "quantity",
            Space::Temperature => "temperature",
            Space::Time => "time",
        })
    }
}

impl Debug for dyn Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl PartialEq for dyn Conversion {
    fn eq(&self, other: &Self) -> bool {
        // ew
        // todo: make this less bad
        self.name() == other.name()
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
        use $crate::units::{Conversion, Num, Space, UnitSpace};
        use identconv::lower_strify;

        pub struct $space;
        impl UnitSpace for $space {
            fn name(&self) -> &'static str {
                lower_strify!($space)
            }

            fn space(&self) -> Space {
                Space::$space
            }

            fn units(&self) -> &'static [&'static dyn Conversion] {
                &[$(&$struct,)*]
            }
        }

        $(
            $(#[$meta])?
            pub struct $struct;
            impl Conversion for $struct {
                fn name(&self) -> &'static str {
                    lower_strify!($struct)
                }

                fn space(&self) -> Space {
                    Space::$space
                }

                fn to_base(&self, this: Num) -> Num {
                    let exe: fn(Num) -> Num = $to_base;
                    exe(this)
                }

                fn from_base(&self, s: Num) -> Num {
                    let exe: fn(Num) -> Num = $from_base;
                    exe(s)
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::prefix::METRIC_PREFIX;

    #[test]
    fn test_name_collisions() {
        let mut sack = HashSet::new();
        let mut collisions = HashSet::new();

        for space in super::UNIT_SPACES {
            for unit in space.units() {
                println!("checking {}", unit.name());
                let mut success = sack.insert(unit.name());

                for i in METRIC_PREFIX {
                    success |= unit.name().starts_with(i.name) || unit.name().starts_with(i.symbol);

                    for j in unit.aliases() {
                        success |= j.starts_with(i.name) || j.starts_with(i.symbol);
                    }
                }

                for alias in unit.aliases() {
                    success |= sack.insert(alias);
                }

                if !success {
                    collisions.insert(unit.name());
                }
            }
        }

        assert!(collisions.is_empty(), "name collisions: {:?}", collisions);
    }
}
