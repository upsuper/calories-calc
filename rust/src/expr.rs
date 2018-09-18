mod parser;

use combine::parser::Parser;
use std::fmt::{self, Display, Formatter};

const KJ_PER_KCAL: f32 = 4.2;

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    base: f32,
    unit: Unit,
    factors: Box<[Factor]>,
}

impl Expr {
    pub fn parse(input: &str) -> Result<Self, ()> {
        match parser::expr().parse(input) {
            Ok((expr, "")) => Ok(expr),
            Ok((_, remaining)) => {
                eprintln!("remaining: '{}'", remaining);
                Err(())
            }
            Err(e) => {
                eprintln!("error: {}", e);
                Err(())
            }
        }
    }

    pub fn calc(&self, unit: Unit) -> f32 {
        let result = self
            .factors
            .iter()
            .fold(self.base, |acc, factor| match factor.op {
                Operator::Multiply => acc * factor.val,
                Operator::Divide => acc / factor.val,
            });
        match (self.unit, unit) {
            (Unit::Kj, Unit::Kj) | (Unit::Kcal, Unit::Kcal) => result,
            (Unit::Kj, Unit::Kcal) => result / KJ_PER_KCAL,
            (Unit::Kcal, Unit::Kj) => result * KJ_PER_KCAL,
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.0} {}", self.base, self.unit)?;
        for factor in self.factors.iter() {
            write!(f, " {}", factor)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Unit {
    Kj,
    Kcal,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Unit::Kj => "kJ",
            Unit::Kcal => "kcal",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Factor {
    op: Operator,
    val: f32,
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {:.0}", self.op, self.val)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Multiply,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Operator::Multiply => "*",
            Operator::Divide => "/",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr_parsing() {
        assert_eq!(
            Expr::parse("15kj"),
            Ok(Expr {
                base: 15.,
                unit: Unit::Kj,
                factors: Default::default(),
            })
        );
        assert_eq!(
            Expr::parse("50.2kJ"),
            Ok(Expr {
                base: 50.2,
                unit: Unit::Kj,
                factors: Default::default(),
            })
        );
        assert_eq!(
            Expr::parse("   15  kcal  "),
            Ok(Expr {
                base: 15.,
                unit: Unit::Kcal,
                factors: Default::default(),
            })
        );
        assert_eq!(
            Expr::parse("17.7kcal/10*3"),
            Ok(Expr {
                base: 17.7,
                unit: Unit::Kcal,
                factors: vec![
                    Factor {
                        op: Operator::Divide,
                        val: 10.,
                    },
                    Factor {
                        op: Operator::Multiply,
                        val: 3.,
                    },
                ].into_boxed_slice(),
            })
        );
        assert_eq!(
            Expr::parse("  15 KJ * 7  / 9.5 "),
            Ok(Expr {
                base: 15.,
                unit: Unit::Kj,
                factors: vec![
                    Factor {
                        op: Operator::Multiply,
                        val: 7.,
                    },
                    Factor {
                        op: Operator::Divide,
                        val: 9.5,
                    },
                ].into_boxed_slice(),
            })
        );
        assert_eq!(
            Expr::parse("-17.7kcal/-10*-3"),
            Ok(Expr {
                base: -17.7,
                unit: Unit::Kcal,
                factors: vec![
                    Factor {
                        op: Operator::Divide,
                        val: -10.,
                    },
                    Factor {
                        op: Operator::Multiply,
                        val: -3.,
                    },
                ].into_boxed_slice(),
            })
        );
        assert_eq!(
            Expr::parse("  -15kj * -7 / -2  "),
            Ok(Expr {
                base: -15.,
                unit: Unit::Kj,
                factors: vec![
                    Factor {
                        op: Operator::Multiply,
                        val: -7.,
                    },
                    Factor {
                        op: Operator::Divide,
                        val: -2.,
                    },
                ].into_boxed_slice(),
            })
        );
    }

    #[test]
    fn expr_calc() {
        assert_eq!(
            Expr {
                base: 15.,
                unit: Unit::Kj,
                factors: Default::default(),
            }.calc(Unit::Kj),
            15.
        );
        assert_eq!(
            Expr {
                base: 50.2,
                unit: Unit::Kj,
                factors: Default::default(),
            }.calc(Unit::Kcal),
            50.2 / KJ_PER_KCAL
        );
        assert_eq!(
            Expr {
                base: 15.,
                unit: Unit::Kcal,
                factors: Default::default(),
            }.calc(Unit::Kj),
            15. * KJ_PER_KCAL
        );
        assert_eq!(
            Expr {
                base: 17.7,
                unit: Unit::Kcal,
                factors: vec![
                    Factor {
                        op: Operator::Divide,
                        val: 10.,
                    },
                    Factor {
                        op: Operator::Multiply,
                        val: 3.,
                    },
                ].into_boxed_slice(),
            }.calc(Unit::Kcal),
            17.7 / 10. * 3.
        );
        assert_eq!(
            Expr {
                base: 15.,
                unit: Unit::Kj,
                factors: vec![
                    Factor {
                        op: Operator::Multiply,
                        val: 7.,
                    },
                    Factor {
                        op: Operator::Divide,
                        val: 9.5,
                    },
                ].into_boxed_slice(),
            }.calc(Unit::Kj),
            15. * 7. / 9.5
        );
    }
}
