use std::{borrow::Cow, fmt::Display, str::FromStr};

use anyhow::Result;
use hashbrown::HashMap;

use crate::{
    dimension::{expander::Expander, tokenizer::Tokenizer, tree::Treeifyer},
    misc::{NumToStringWithChars, SUPERSCRIPT_CHARSET},
    units::Conversion,
    Num,
};

#[derive(Debug)]
pub struct Dimensions {
    units: Vec<Unit>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit {
    conversion: &'static dyn Conversion,
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

impl Unit {
    /// A unit is special if it is not part of the base dimension.
    /// I should have probably thought of a better name...
    pub fn is_special(&self) -> bool {
        // TODO: add type ids or something :sob:
        self.conversion.name() == "varnum"
    }
}

impl Dimensions {
    pub fn convert(&self, other: &Dimensions, mut value: Num, debug: bool) -> Result<Num> {
        for i in &self.units {
            assert_eq!(i.power.fract(), 0.0);
            let old = value;
            for _ in 0..i.power.abs() as usize {
                value = if i.power.signum() > 0.0 {
                    i.conversion.to_base(value)
                } else {
                    i.conversion.from_base(value)
                }
            }
            value *= (10 as Num).powf(i.sci_exponent);
            debug_println!(
                debug,
                "{: <8} =[ {: <6} ]=> {}",
                old,
                i.conversion.name(),
                value
            );
        }

        debug_println!(debug);

        for i in &other.units {
            assert_eq!(i.power.fract(), 0.0);
            let old = value;
            for _ in 0..i.power.abs() as usize {
                value = if i.power.signum() > 0.0 {
                    i.conversion.from_base(value)
                } else {
                    i.conversion.to_base(value)
                }
            }
            value *= (10 as Num).powf(-i.sci_exponent);
            debug_println!(
                debug,
                "{: <8.5} =[ {: <6} ]=> {:.5}",
                old,
                i.conversion.name(),
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
                .find(|x| x.conversion.space() == i.conversion.space())
            {
                j.sci_exponent += i.sci_exponent;
                j.power += i.power
            } else {
                new_units.push(i.to_owned());
            }
        }

        Dimensions { units: new_units }
    }
}

impl Unit {
    pub const fn new(conversion: &'static dyn Conversion, power: Num, sci_exponent: Num) -> Self {
        Self {
            conversion,
            power,
            sci_exponent,
        }
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
        let tree = Treeifyer::treeify(tokens);
        let units = Expander::expand(tree)?;

        Ok(Dimensions { units })
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if f.alternate() {
            |unit: &Unit| Cow::Owned(unit.conversion.space().to_string())
        } else {
            |unit: &Unit| Cow::Borrowed(unit.conversion.name())
        };

        let mut out = String::new();

        for unit in self.units.iter().filter(|x| !x.is_special()) {
            out.push_str(&if unit.power == 1.0 {
                format!("[{}] ", name(unit))
            } else {
                format!(
                    "[{}]{} ",
                    name(unit),
                    unit.power.to_string_with_chars(SUPERSCRIPT_CHARSET)
                )
            });
        }

        out.pop();
        f.write_str(&out)
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        let mut self_dimensions = HashMap::new();
        for unit in self.units.iter().filter(|x| !x.is_special()) {
            *self_dimensions
                .entry(unit.conversion.space())
                .or_insert(0.0) += unit.power;
        }

        let mut other_dimensions = HashMap::new();
        for unit in other.units.iter().filter(|x| !x.is_special()) {
            *other_dimensions
                .entry(unit.conversion.space())
                .or_insert(0.0) += unit.power;
        }

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

pub mod expander {
    use anyhow::{bail, Result};

    use super::{Op, Token, Unit};
    use crate::Num;

    pub struct Expander {
        units: Vec<Unit>,
    }

    impl Expander {
        pub fn expand(token: Token) -> Result<Vec<Unit>> {
            let mut exp = Self::new();
            exp._expand(token, 1.0)?;

            Ok(exp.units)
        }

        fn new() -> Self {
            Self { units: Vec::new() }
        }

        fn _expand(&mut self, token: Token, power: Num) -> Result<()> {
            match token {
                Token::Tree(op, left, right) => match op {
                    Op::Pow => {
                        self._expand(
                            *left,
                            power
                                * match *right {
                                    Token::Num(num) => num,
                                    _ => bail!("Invalid exponent. (Expected number)"),
                                },
                        )?;
                    }
                    Op::Div => {
                        self._expand(*left, power)?;
                        self._expand(*right, -power)?;
                    }
                    _ => {
                        self._expand(*left, power)?;
                        self._expand(*right, power)?;
                    }
                },
                Token::Unit(Unit {
                    conversion,
                    power: unit_power,
                    sci_exponent,
                }) => {
                    let unit = Unit {
                        conversion,
                        // todo: is this correct?
                        power: power * unit_power,
                        sci_exponent,
                    };
                    self.units.push(unit);
                }
                Token::Group(group) => {
                    for i in group {
                        self._expand(i, power)?;
                    }
                }
                Token::Num(..) | Token::Op(..) => unreachable!(),
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use crate::{
            dimension::{Op, Token, Unit},
            units::{length::Meter, time::Second, Conversion},
        };

        use super::Expander;

        #[test]
        fn test_expander() {
            let sec = &Second as &'static dyn Conversion;
            let meter = &Meter as &'static dyn Conversion;

            let inp = Token::Tree(
                Op::Div,
                Box::new(Token::Unit(Unit {
                    conversion: meter,
                    power: 1.0,
                    sci_exponent: 0.0,
                })),
                Box::new(Token::Tree(
                    Op::Pow,
                    Box::new(Token::Unit(Unit {
                        conversion: sec,
                        power: 1.0,
                        sci_exponent: 0.0,
                    })),
                    Box::new(Token::Num(2.0)),
                )),
            );

            let exp = Expander::expand(inp).unwrap();
            assert_eq!(
                exp,
                vec![
                    Unit {
                        conversion: meter,
                        power: 1.0,
                        sci_exponent: 0.0,
                    },
                    Unit {
                        conversion: sec,
                        power: -2.0,
                        sci_exponent: 0.0,
                    }
                ]
            );
        }
    }
}

pub mod tree {
    use hashbrown::HashMap;

    use super::Token;

    pub struct Treeifyer {
        tokens: Vec<Token>,
        precedence: HashMap<u8, u32>,
    }

    impl Treeifyer {
        pub fn treeify(mut tokens: Vec<Token>) -> Token {
            if tokens.len() == 1 {
                let token = tokens.pop().unwrap();
                match token {
                    Token::Group(tokens) => return Treeifyer::treeify(tokens),
                    Token::Unit { .. } => return token,
                    Token::Op(..) | Token::Tree(..) | Token::Num(..) => {
                        unreachable!("Invalid token")
                    }
                }
            }

            let mut ctx = Self::new(tokens);
            ctx.update_precedence_counts();
            ctx._treeify();

            assert_eq!(ctx.tokens.len(), 1);
            ctx.tokens.pop().unwrap()
        }

        fn new(tokens: Vec<Token>) -> Self {
            Self {
                tokens,
                precedence: HashMap::new(),
            }
        }

        // this is probably inefficient or something but my brain cant handle thinking about math for any longer
        fn _treeify(&mut self) {
            while self.tokens.len() > 1 {
                let mut i = 0;
                while i < self.tokens.len() {
                    let Token::Op(op) = self.tokens[i] else {
                        i += 1;
                        continue;
                    };

                    let max_precedence = self
                        .precedence
                        .iter()
                        .filter(|x| *x.1 > 0)
                        .max_by_key(|x| x.0)
                        .unwrap()
                        .0;
                    if op.precedence() < *max_precedence {
                        i += 1;
                        continue;
                    }

                    let make_tree = |x| match x {
                        Token::Group(tokens) => Treeifyer::treeify(tokens),
                        _ => x,
                    };

                    let left = make_tree(self.tokens.remove(i - 1));
                    let right = make_tree(self.tokens.remove(i));

                    self.tokens[i - 1] = Token::Tree(op, Box::new(left), Box::new(right));
                    self.precedence
                        .entry(op.precedence())
                        .and_modify(|x| *x -= 1);
                    break;
                }
            }
        }

        fn update_precedence_counts(&mut self) {
            for i in &self.tokens {
                if let Token::Op(op) = i {
                    *self.precedence.entry(op.precedence()).or_insert(0) += 1;
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::super::{Op, Token};
        use super::Treeifyer;
        use crate::dimension::Unit;
        use crate::units::{
            time::{Minute, Second},
            Conversion,
        };

        #[test]
        fn test_tree() {
            let sec = &Second as &'static dyn Conversion;
            let min = &Minute as &'static dyn Conversion;

            let tokens = vec![
                Token::Unit(Unit {
                    conversion: min,
                    power: 1.0,
                    sci_exponent: 1.0,
                }),
                Token::Op(Op::Div),
                Token::Unit(Unit {
                    conversion: sec,
                    power: 1.0,
                    sci_exponent: 1.0,
                }),
                Token::Op(Op::Pow),
                Token::Num(2.0),
            ];

            let tree = Treeifyer::treeify(tokens);

            assert_eq!(
                tree,
                Token::Tree(
                    Op::Div,
                    Box::new(Token::Unit(Unit {
                        conversion: min,
                        power: 1.0,
                        sci_exponent: 1.0,
                    })),
                    Box::new(Token::Tree(
                        Op::Pow,
                        Box::new(Token::Unit(Unit {
                            conversion: sec,
                            power: 1.0,
                            sci_exponent: 1.0,
                        })),
                        Box::new(Token::Num(2.0)),
                    ))
                )
            );
        }
    }
}

pub mod tokenizer {
    use anyhow::{bail, Result};

    use super::{Op, Token, Unit};
    use crate::{prefix, units::ConversionType, Num};

    pub struct Tokenizer {
        chars: Box<[char]>,
        index: usize,
        depth: usize,

        pub(super) tokens: Vec<Token>,
        buffer: String,
    }

    impl Tokenizer {
        pub fn tokenize(raw: &str) -> Result<Vec<Token>> {
            let mut ctx = Self::new(raw);

            while ctx.index < ctx.chars.len() {
                let chr = ctx.chars[ctx.index];
                ctx.index += 1;

                if ctx.depth > 0 {
                    match chr {
                        '(' => ctx.depth += 1,
                        ')' => {
                            ctx.depth -= 1;
                            if ctx.depth == 0 {
                                ctx.tokens
                                    .push(Token::Group(Tokenizer::tokenize(&ctx.buffer)?));
                                ctx.buffer.clear();
                            }
                        }
                        _ => ctx.buffer.push(chr),
                    }
                    continue;
                }

                match chr {
                    x if x.is_whitespace() => continue,
                    '/' => ctx.add_token(Op::Div)?,
                    '*' => ctx.add_token(Op::Mul)?,
                    '^' => ctx.add_token(Op::Pow)?,
                    '(' => ctx.depth += 1,
                    ')' => bail!("Unmatched closing parenthesis"),
                    _ => ctx.buffer.push(chr),
                }
            }

            ctx.flush_buffer()?;
            Ok(ctx.tokens)
        }

        fn new(input: &str) -> Self {
            Self {
                chars: input.chars().collect(),
                index: 0,
                depth: 0,

                tokens: Vec::new(),
                buffer: String::new(),
            }
        }

        fn add_token(&mut self, op: Op) -> Result<()> {
            self.flush_buffer()?;
            self.tokens.push(Token::Op(op));
            Ok(())
        }

        fn flush_buffer(&mut self) -> Result<()> {
            if self.buffer.is_empty() {
                return Ok(());
            }

            if let Ok(num) = self.buffer.parse::<Num>() {
                self.tokens.push(Token::Num(num));
            } else if let Some((conversion, prefix)) = prefix::get(&self.buffer) {
                match conversion {
                    ConversionType::Conversion(conversion) => self.tokens.push(Token::Unit(Unit {
                        conversion,
                        power: 1.0,
                        sci_exponent: prefix.map(|x| x.power as Num).unwrap_or(0.0),
                    })),
                    ConversionType::DerivedConversion(conversion) => {
                        let mut tokens = conversion
                            .expand()
                            .iter()
                            .map(|x| Token::Unit(*x))
                            .intersperse(Token::Op(Op::Mul))
                            .collect::<Vec<_>>();
                        // TODO: clean this up, please
                        if let Some(prefix) = prefix {
                            if let Some(Token::Unit(unit)) = tokens.first_mut() {
                                unit.sci_exponent += prefix.power as Num;
                            }
                        }
                        self.tokens.push(Token::Group(tokens))
                    }
                }
            } else {
                bail!("Invalid token: {}", self.buffer);
            }

            self.buffer.clear();
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use crate::{
            dimension::Unit,
            units::{length::Meter, time::Second, Conversion},
        };

        use super::{Op, Token, Tokenizer};

        #[test]
        fn test_tokenize() {
            let sec = &Second as &'static dyn Conversion;
            let meter = &Meter as &'static dyn Conversion;

            let tokens = Tokenizer::tokenize("m/s^2").unwrap();
            assert_eq!(
                tokens,
                vec![
                    Token::Unit(Unit {
                        conversion: meter,
                        power: 1.0,
                        sci_exponent: 0.0,
                    }),
                    Token::Op(Op::Div),
                    Token::Unit(Unit {
                        conversion: sec,
                        power: 1.0,
                        sci_exponent: 0.0,
                    }),
                    Token::Op(Op::Pow),
                    Token::Num(2.0),
                ]
            );
        }

        #[test]
        fn test_tokenize_2() {
            let sec = &Second as &'static dyn Conversion;
            let meter = &Meter as &'static dyn Conversion;

            let tokens = Tokenizer::tokenize("m / (s * s)").unwrap();
            assert_eq!(
                tokens,
                vec![
                    Token::Unit(Unit {
                        conversion: meter,
                        power: 1.0,
                        sci_exponent: 0.0,
                    }),
                    Token::Op(Op::Div),
                    Token::Group(vec![
                        Token::Unit(Unit {
                            conversion: sec,
                            power: 1.0,
                            sci_exponent: 0.0,
                        }),
                        Token::Op(Op::Mul),
                        Token::Unit(Unit {
                            conversion: sec,
                            power: 1.0,
                            sci_exponent: 0.0,
                        })
                    ]),
                ]
            );
        }
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
