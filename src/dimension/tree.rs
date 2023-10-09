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
    use crate::units::time::{MINUTE, SECOND};

    #[test]
    fn test_tree() {
        let tokens = vec![
            Token::Unit(Unit {
                conversion: &MINUTE,
                power: 1.0,
                sci_exponent: 1.0,
            }),
            Token::Op(Op::Div),
            Token::Unit(Unit {
                conversion: &SECOND,
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
                    conversion: &MINUTE,
                    power: 1.0,
                    sci_exponent: 1.0,
                })),
                Box::new(Token::Tree(
                    Op::Pow,
                    Box::new(Token::Unit(Unit {
                        conversion: &SECOND,
                        power: 1.0,
                        sci_exponent: 1.0,
                    })),
                    Box::new(Token::Num(2.0)),
                ))
            )
        );
    }
}
