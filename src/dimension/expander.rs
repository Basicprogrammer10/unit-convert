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
                    let Token::Num(exp) = *right else {
                        bail!("Invalid exponent. (Expected number literal)");
                    };
                    self._expand(*left, power * exp)?;
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
        units::{length::METER, time::SECOND},
    };

    use super::Expander;

    #[test]
    fn test_expander() {
        let inp = Token::Tree(
            Op::Div,
            Box::new(Token::Unit(Unit {
                conversion: &METER,
                power: 1.0,
                sci_exponent: 0.0,
            })),
            Box::new(Token::Tree(
                Op::Pow,
                Box::new(Token::Unit(Unit {
                    conversion: &SECOND,
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
                    conversion: &METER,
                    power: 1.0,
                    sci_exponent: 0.0,
                },
                Unit {
                    conversion: &SECOND,
                    power: -2.0,
                    sci_exponent: 0.0,
                }
            ]
        );
    }
}
