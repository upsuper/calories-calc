use super::{Expr, Factor, Operator, Unit};
use combine::{
    parser::{
        byte::{bytes_cmp, digit, spaces},
        choice::{choice, optional},
        combinator::attempt,
        range::recognize,
        repeat::{many, skip_many1},
        token::token,
    },
    Parser,
};

pub fn expr<'a>() -> impl Parser<&'a [u8], Output = Expr> {
    (
        spaces(),
        float(),
        spaces(),
        unit(),
        spaces(),
        many((factor(), spaces()).map(|(f, _)| f)),
    )
        .map(|(_, base, _, unit, _, factors)| Expr {
            base,
            unit,
            factors,
        })
}

fn float<'a>() -> impl Parser<&'a [u8], Output = f32> {
    (optional(token(b'-')), recognize(skip_many1(digit()))).map(|(neg, digits): (_, &[u8])| {
        let mut result = 0.;
        for digit in digits {
            let digit = digit - b'0';
            result = result * 10. + digit as f32;
        }
        if neg.is_some() {
            result = -result;
        }
        result
    })
}

fn unit<'a>() -> impl Parser<&'a [u8], Output = Unit> {
    choice((
        bytes_cmp_ignore_ascii_case(b"kj").map(|_| Unit::Kj),
        bytes_cmp_ignore_ascii_case(b"kcal").map(|_| Unit::Kcal),
    ))
}

fn bytes_cmp_ignore_ascii_case<'a>(
    s: &'static [u8],
) -> impl Parser<&'a [u8], Output = &'static [u8]> {
    attempt(bytes_cmp(s, |l, r| l.eq_ignore_ascii_case(&r)))
}

fn factor<'a>() -> impl Parser<&'a [u8], Output = Factor> {
    (operator(), spaces(), float()).map(|(op, _, val)| Factor { op, val })
}

fn operator<'a>() -> impl Parser<&'a [u8], Output = Operator> {
    choice((
        token(b'*').map(|_| Operator::Multiply),
        token(b'/').map(|_| Operator::Divide),
    ))
}
