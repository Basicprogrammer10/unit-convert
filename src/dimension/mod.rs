use std::{borrow::Cow, fmt::Display, str::FromStr};

use anyhow::Result;
use hashbrown::HashMap;

use crate::{
    dimension::{expander::Expander, tokenizer::Tokenizer, tree::Treeifyer},
    misc::{NumToStringWithChars, SUPERSCRIPT_CHARSET},
    units::Conversion,
    Num,
};

pub mod expander;
pub mod tokenizer;
pub mod tree;

#[derive(Debug)]
pub struct Dimensions {
    units: Vec<Unit>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit {
    conversion: &'static Conversion,
    power: Num,
    /// * 10^exponent
    sci_exponent: Num,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unit(Unit),
    Num(Num),
    Op(Op),
    Group(Vec<Token>),
    Tree(Op, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Op {
    #[default]
    Mul,
    Div,
    Pow,
}

macro_rules! debug_println {
    ($is_debug:expr) => {
        debug_println!($is_debug,);
    };
    ($is_debug:expr, $($arg:tt)*) => {
        if $is_debug {
            println!($($arg)*);
        }
    };
}

impl Dimensions {
    pub fn convert(&self, other: &Dimensions, mut value: Num, debug: bool) -> Result<Num> {
        for i in &self.units {
            assert_eq!(i.power.fract(), 0.0);
            let old = value;
            for _ in 0..i.power.abs() as usize {
                value = if i.power.signum() > 0.0 {
                    (i.conversion.to_base)(value)
                } else {
                    (i.conversion.from_base)(value)
                }
            }
            value *= (10 as Num).powf(i.sci_exponent * i.power.signum());
            debug_println!(
                debug,
                "{: <8} =[ {: <6} ]=> {}",
                old,
                i.conversion.name,
                value
            );
        }

        debug_println!(debug);

        for i in &other.units {
            assert_eq!(i.power.fract(), 0.0);
            let old = value;
            for _ in 0..i.power.abs() as usize {
                value = if i.power.signum() > 0.0 {
                    (i.conversion.from_base)(value)
                } else {
                    (i.conversion.to_base)(value)
                }
            }
            value *= (10 as Num).powf(-i.sci_exponent * i.power.signum());
            debug_println!(
                debug,
                "{: <8.5} =[ {: <6} ]=> {:.5}",
                old,
                i.conversion.name,
                value
            );
        }

        debug_println!(debug);
        Ok(value)
    }

    pub fn simplify(&self) -> Self {
        let mut new_units = Vec::<Unit>::new();

        for i in &self.units {
            if let Some(j) = new_units
                .iter_mut()
                .find(|x| x.conversion.space == i.conversion.space)
            {
                j.sci_exponent += i.sci_exponent * i.power.signum();
                j.power += i.power
            } else {
                new_units.push(i.to_owned());
            }
        }

        Dimensions { units: new_units }
    }
}

impl Unit {
    pub const fn new(conversion: &'static Conversion, power: Num, sci_exponent: Num) -> Self {
        Self {
            conversion,
            power,
            sci_exponent,
        }
    }

    pub fn is_special(&self) -> bool {
        self.conversion.special
    }
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Self::Mul => 2,
            Self::Div => 2,
            Self::Pow => 3,
        }
    }
}

impl FromStr for Dimensions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = Tokenizer::tokenize(s)?;
        let tree = Treeifyer::treeify(tokens)?;
        let units = Expander::expand(tree)?;

        Ok(Dimensions { units })
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut units = self.units.iter().filter(|x| !x.is_special());

        if let Some(unit) = units.next() {
            write!(f, "{}", unit)?;
        }

        for unit in units {
            write!(f, " {}", unit)?;
        }

        Ok(())
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if f.alternate() {
            |unit: &Unit| Cow::Owned(unit.conversion.space.to_string())
        } else {
            |unit: &Unit| Cow::Borrowed(unit.conversion.name)
        };

        if self.power == 1.0 {
            write!(f, "[{}]", name(self))
        } else {
            write!(
                f,
                "[{}]{}",
                name(self),
                self.power.to_string_with_chars(SUPERSCRIPT_CHARSET)
            )
        }
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        let mut self_dimensions = HashMap::new();
        for unit in self.units.iter().filter(|x| !x.conversion.special) {
            *self_dimensions.entry(unit.conversion.space).or_insert(0.0) += unit.power;
        }

        let mut other_dimensions = HashMap::new();
        for unit in other.units.iter().filter(|x| !x.conversion.special) {
            *other_dimensions.entry(unit.conversion.space).or_insert(0.0) += unit.power;
        }

        // self_dimensions.retain(|_, &mut v| v != 0.0);
        if self_dimensions.len() != other_dimensions.len() {
            return false;
        }

        for (&space, &exponent) in &self_dimensions {
            match other_dimensions.get(&space) {
                Some(&i) if i == exponent => {}
                _ => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Dimensions;

    #[test]
    fn test_dimensions() {
        let a = "m/s^2";
        let b = "m/(s*s)";
        let c = "m/s/s";

        let a = Dimensions::from_str(a).unwrap();
        for i in &[b, c] {
            let j = Dimensions::from_str(i).unwrap();
            assert_eq!(a, j, "Failed on: `{i}`");
        }
    }
}
