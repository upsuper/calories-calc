use super::{Expr, Factor, Operator, Unit};
use combine::{
    error::StringStreamError,
    parser::{
        char::{digit, spaces, string_cmp},
        choice::{choice, optional},
        combinator::r#try,
        item::item,
        range::recognize,
        repeat::{many, skip_many, skip_many1},
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
    recognize((
        optional(item('-')),
        skip_many1(digit()),
        optional((item('.'), skip_many(digit()))),
    )).and_then(|s: &str| s.parse().map_err(|_| StringStreamError::UnexpectedParse))
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
