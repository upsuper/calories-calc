mod parser;

use combine::parser::Parser;
use crate::display::{Display, Rounded};
use std::fmt;

const KJ_PER_KCAL: f32 = 4.2;

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    base: f32,
    unit: Unit,
    factors: Vec<Factor>,
}

impl Expr {
    pub fn parse(input: &str) -> Result<Self, ()> {
        match parser::expr().parse(input.as_bytes()) {
            Ok((expr, b"")) => Ok(expr),
            _ => Err(()),
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

    pub fn adjust_factor(&mut self, delta: f32) {
        let mut new_factor = None;
        if let Some(mut poped_factor) = self.factors.pop() {
            if poped_factor.op == Operator::Multiply {
                poped_factor.val += delta;
                new_factor = Some(poped_factor);
            } else {
                self.factors.push(poped_factor);
            }
        }
        let new_factor = new_factor.unwrap_or(Factor {
            op: Operator::Multiply,
            val: delta + 1.,
        });
        if (new_factor.val - 1.).abs() >= 1e-5 {
            self.factors.push(new_factor);
        }
    }
}

impl Display for Expr {
    fn display(&self, output: &mut impl fmt::Write) -> fmt::Result {
        Rounded(self.base).display(output)?;
        output.write_char(' ')?;
        self.unit.display(output)?;
        for factor in self.factors.iter() {
            factor.display(output)?;
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
    fn display(&self, output: &mut impl fmt::Write) -> fmt::Result {
        output.write_str(match self {
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
    fn display(&self, output: &mut impl fmt::Write) -> fmt::Result {
        output.write_char(' ')?;
        self.op.display(output)?;
        output.write_char(' ')?;
        Rounded(self.val).display(output)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Multiply,
    Divide,
}

impl Display for Operator {
    fn display(&self, output: &mut impl fmt::Write) -> fmt::Result {
        output.write_char(match self {
            Operator::Multiply => '*',
            Operator::Divide => '/',
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! op {
        (*) => {
            Operator::Multiply
        };
        (/) => {
            Operator::Divide
        };
    }
    macro_rules! expr {
        (($base:expr) $unit:ident $($op:tt ($val:expr))*) => {
            Expr {
                base: $base,
                unit: Unit::$unit,
                factors: vec![
                    $(Factor {
                        op: op!($op),
                        val: $val,
                    },)*
                ],
            }
        };
    }

    #[test]
    fn expr_parsing() {
        assert_eq!(Expr::parse("15kj"), Ok(expr!((15.) Kj)));
        assert_eq!(Expr::parse("50kJ"), Ok(expr!((50.) Kj)));
        assert_eq!(Expr::parse("   15  kcal  "), Ok(expr!((15.) Kcal)));
        assert_eq!(
            Expr::parse("18kcal/10*3"),
            Ok(expr!((18.) Kcal / (10.) * (3.)))
        );
        assert_eq!(
            Expr::parse("  15 KJ * 7  / 9 "),
            Ok(expr!((15.) Kj * (7.) / (9.)))
        );
        assert_eq!(
            Expr::parse("-17kcal/-10*-3"),
            Ok(expr!((-17.) Kcal / (-10.) * (-3.)))
        );
        assert_eq!(
            Expr::parse("  -15kj * -7 / -2  "),
            Ok(expr!((-15.) Kj * (-7.) / (-2.)))
        );
    }

    #[test]
    fn expr_calc() {
        assert_eq!(expr!((15.) Kj).calc(Unit::Kj), 15.);
        assert_eq!(expr!((50.2) Kj).calc(Unit::Kcal), 50.2 / KJ_PER_KCAL);
        assert_eq!(expr!((15.) Kcal).calc(Unit::Kj), 15. * KJ_PER_KCAL);
        assert_eq!(
            expr!((17.7) Kcal / (10.) * (3.)).calc(Unit::Kcal),
            17.7 / 10. * 3.
        );
        assert_eq!(
            expr!((15.) Kj * (7.) / (9.5)).calc(Unit::Kj),
            15. * 7. / 9.5
        );
    }

    #[test]
    fn expr_adjust_factor() {
        fn adjusted(mut expr: Expr, delta: f32) -> Expr {
            expr.adjust_factor(delta);
            expr
        }
        assert_eq!(adjusted(expr!((15.) Kj), 1.5), expr!((15.) Kj * (2.5)));
        assert_eq!(adjusted(expr!((50.2) Kcal), 2.), expr!((50.2) Kcal * (3.)));
        assert_eq!(
            adjusted(expr!((17.7) Kcal / (10.)), 1.1),
            expr!((17.7) Kcal / (10.) * (2.1))
        );
        assert_eq!(
            adjusted(expr!((15.) Kj * (7.)), 9.),
            expr!((15.) Kj * (16.))
        );
        assert_eq!(adjusted(expr!((15.) Kj * (2.)), -1.), expr!((15.) Kj));
    }
}
