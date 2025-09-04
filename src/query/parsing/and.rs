use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag_no_case;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::error::context;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use crate::query::and::QueryAnd;
use crate::query::parsing::expression::parse_filter_token_or_subexpr;
use crate::query::parsing::sp1;

pub(super) fn parse_explicit_and<'a, E>(input: &'a str) -> IResult<&'a str, QueryAnd, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let and_parser = delimited(sp1, tag_no_case("and"), sp1);
    let (leftover_input, (left, right)) = separated_pair(
        parse_filter_token_or_subexpr,
        and_parser,
        parse_filter_token_or_subexpr,
    )
    .parse(input)?;

    Ok((leftover_input, QueryAnd(left, right)))
}

pub(super) fn parse_implicit_and<'a, E>(input: &'a str) -> IResult<&'a str, QueryAnd, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let parser = separated_pair(
        parse_filter_token_or_subexpr,
        sp1,
        parse_filter_token_or_subexpr,
    );

    let (leftover_input, (left, right)) = context("Implicite And", parser).parse(input)?;

    Ok((leftover_input, QueryAnd(left, right)))
}

#[cfg(test)]
pub mod test {
    use nom::Finish as _;
    use nom_language::error::VerboseError;

    use crate::query::and::QueryAnd;
    use crate::query::any_tag_string::AnyTagString;
    use crate::query::parsing::and::parse_explicit_and;
    use crate::query::parsing::and::parse_implicit_and;
    use crate::query::parsing::assert_nom;

    #[test]
    pub fn parse_explicit_and_test() {
        assert_eq!(
            parse_explicit_and::<VerboseError<_>>(" oiia_oiia and maxwell ")
                .finish()
                .unwrap(),
            (
                " ",
                QueryAnd(
                    AnyTagString::new1("oiia_oiia").into(),
                    AnyTagString::new1("maxwell").into()
                )
            )
        );

        assert!(parse_explicit_and::<VerboseError<_>>(" \"oiia_oiia and maxwell\"").is_err())
    }

    #[test]
    pub fn parse_implicit_and_test() {
        assert_nom(
            " oiia_oiia maxwell ",
            parse_implicit_and,
            (
                " ",
                QueryAnd(
                    AnyTagString::new1("oiia_oiia").into(),
                    AnyTagString::new1("maxwell").into(),
                ),
            ),
        );

        assert!(parse_implicit_and::<VerboseError<_>>(" \"oiia_oiia and maxwell\"").is_err())
    }
}
