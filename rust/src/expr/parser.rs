use super::{Expr, Factor, Operator, Unit};
use combine::{
    parser::{
        char::{digit, spaces, string_cmp},
        choice::{choice, optional},
        combinator::r#try,
        item::item,
        range::recognize,
        repeat::{many, skip_many1},
    },
    Parser,
};

pub fn expr<'a>() -> impl Parser<Input = &'a str, Output = Expr> {
    (
        spaces(),
        float(),
        spaces(),
        unit(),
        spaces(),
        many::<Vec<_>, _>((factor(), spaces()).map(|(f, _)| f)),
    )
        .map(|(_, base, _, unit, _, factors)| Expr {
            base,
            unit,
            factors,
        })
}

fn float<'a>() -> impl Parser<Input = &'a str, Output = f32> {
    (
        optional(item('-')),
        recognize(skip_many1(digit())),
    ).map(|(neg, digits): (_, &str)| {
        let mut result = 0.;
        for digit in digits.as_bytes() {
            let digit = digit - b'0';
            result = result * 10. + digit as f32;
        }
        if neg.is_some() {
            result = -result;
        }
        result
    })
}

fn unit<'a>() -> impl Parser<Input = &'a str, Output = Unit> {
    choice((
        string_cmp_ignore_ascii_case("kj").map(|_| Unit::Kj),
        string_cmp_ignore_ascii_case("kcal").map(|_| Unit::Kcal),
    ))
}

fn string_cmp_ignore_ascii_case<'a>(
    s: &'static str,
) -> impl Parser<Input = &'a str, Output = &'static str> {
    r#try(string_cmp(s, |l, r| l.eq_ignore_ascii_case(&r)))
}

fn factor<'a>() -> impl Parser<Input = &'a str, Output = Factor> {
    (operator(), spaces(), float()).map(|(op, _, val)| Factor { op, val })
}

fn operator<'a>() -> impl Parser<Input = &'a str, Output = Operator> {
    choice((
        item('*').map(|_| Operator::Multiply),
        item('/').map(|_| Operator::Divide),
    ))
}
