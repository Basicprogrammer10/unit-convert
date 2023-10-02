use std::fmt::{Debug, Display};

use crate::{impl_conversion, impl_unit_space, Num};

pub mod duration;
pub mod length;

pub const UNIT_SPACES: &[&dyn UnitSpace] = &[&duration::Duration, &length::Length];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Space {
    Duration,
    Length,
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
    /// Converts a value in this unit to the unit space's base unit.
    fn to_base(&self, this: &Num) -> Num;
    /// Converts a value in the unit space's base unit to this unit.
    fn from_base(&self, s: &Num) -> Num;

    /// Gets the aliases of the unit.
    /// All aliases must be lowercase.
    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }
    /// Checks if the unit is a metric unit.
    /// Metric units can use metric prefixes.
    fn is_metric(&self) -> bool {
        false
    }

    /// Checks if the given name is the name or an alias of this unit.
    /// Case insensitive.
    fn is_alias(&self, name: &str) -> bool {
        let name = name.to_ascii_lowercase();
        self.name() == name || self.aliases().contains(&name.as_str())
    }
}

#[macro_export]
macro_rules! impl_conversion {
    ($struct:ident, $name:expr, $to_base:expr, $from_base:expr$(, aliases = [$($aliases:expr),*])?$(, metric = $metric:expr)?) => {
        pub struct $struct;
        impl Conversion for $struct {
            fn name(&self) -> &'static str {
                $name
            }

            fn to_base(&self, this: &Num) -> Num {
                let exe: fn(&Num) -> Num = $to_base;
                exe(this)
            }

            fn from_base(&self, s: &Num) -> Num {
                let exe: fn(&Num) -> Num = $from_base;
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
    };
}

#[macro_export]
macro_rules! impl_unit_space {
    ($struct:ident, $name:expr, $space:expr, $units:expr) => {
        pub struct $struct;
        impl UnitSpace for $struct {
            fn name(&self) -> &'static str {
                $name
            }

            fn space(&self) -> Space {
                $space
            }

            fn units(&self) -> &'static [&'static dyn Conversion] {
                $units
            }
        }
    };
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

impl Debug for dyn Conversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn test_name_collisions() {
        let mut sack = HashSet::new();
        let mut collisions = HashSet::new();

        for space in super::UNIT_SPACES {
            for unit in space.units() {
                let mut success = sack.insert(unit.name());
                for alias in unit.aliases() {
                    success &= sack.insert(alias);
                }

                if !success {
                    collisions.insert(unit.name());
                }
            }
        }

        assert!(collisions.is_empty(), "name collisions: {:?}", collisions);
    }
}
