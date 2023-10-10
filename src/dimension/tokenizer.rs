use anyhow::{bail, Result};

use super::{Op, Token, Unit};
use crate::{
    prefix,
    units::{derived::constant, ConversionType},
    Num,
};

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
        } else if let Some((conversion, prefix)) =
            prefix::get(&self.buffer.replace(['_', '-'], " "))
        {
            add_conversion_tokens(
                &mut self.tokens,
                conversion,
                None,
                prefix.map(|x| x.power as Num).unwrap_or(0.0),
            );
        } else {
            bail!("Invalid token: {}", self.buffer);
        }

        self.buffer.clear();
        Ok(())
    }
}

pub fn add_conversion_tokens(
    tokens: &mut Vec<Token>,
    conversion: ConversionType,
    power: Option<Num>,
    sci_exponent: Num,
) {
    match conversion {
        ConversionType::Conversion(conversion) => tokens.push(Token::Unit(Unit {
            conversion,
            power: power.unwrap_or(1.0),
            sci_exponent,
        })),
        ConversionType::DerivedConversion(conversion) => {
            let mut new_tokens = conversion
                .expand
                .iter()
                .map(|x| Token::Unit(*x))
                .intersperse(Token::Op(Op::Mul))
                .collect::<Vec<_>>();

            if sci_exponent != 0.0 && !new_tokens.is_empty() {
                new_tokens.push(Token::Op(Op::Mul));
                new_tokens.push(Token::Unit(constant!(1.0, sci_exponent)));
            }

            tokens.push(Token::Group(new_tokens))
        }
        ConversionType::Shorthand(shorthand) => {
            debug_assert!(power.is_none());
            for (i, e) in shorthand.unit.iter().enumerate() {
                match e.conversion {
                    ConversionType::DerivedConversion(..) | ConversionType::Conversion(..) => {
                        add_conversion_tokens(
                            tokens,
                            e.conversion.clone(),
                            Some(e.power),
                            e.sci_exponent + if i == 0 { sci_exponent } else { 0.0 },
                        )
                    }
                    ConversionType::Shorthand(..) => unreachable!("Shorthands in shorthands!?"),
                }
                tokens.push(Token::Op(Op::Mul));
            }
            tokens.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        dimension::Unit,
        units::{length::METER, time::SECOND},
    };

    use super::{Op, Token, Tokenizer};

    #[test]
    fn test_tokenize() {
        let tokens = Tokenizer::tokenize("m/s^2").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Unit(Unit {
                    conversion: &METER,
                    power: 1.0,
                    sci_exponent: 0.0,
                }),
                Token::Op(Op::Div),
                Token::Unit(Unit {
                    conversion: &SECOND,
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
        let tokens = Tokenizer::tokenize("m / (s * s)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Unit(Unit {
                    conversion: &METER,
                    power: 1.0,
                    sci_exponent: 0.0,
                }),
                Token::Op(Op::Div),
                Token::Group(vec![
                    Token::Unit(Unit {
                        conversion: &SECOND,
                        power: 1.0,
                        sci_exponent: 0.0,
                    }),
                    Token::Op(Op::Mul),
                    Token::Unit(Unit {
                        conversion: &SECOND,
                        power: 1.0,
                        sci_exponent: 0.0,
                    })
                ]),
            ]
        );
    }
}
