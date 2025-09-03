use nom::IResult;
use nom::Parser as _;
use nom::bytes::complete::tag_no_case;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::sequence::separated_pair;

use crate::query::or::QueryOr;
use crate::query::parsing::expression::parse_filter_token_or_subexpr;
use crate::query::parsing::sp1;

pub(super) fn parse_explicit_or<'a, E>(input: &'a str) -> IResult<&'a str, QueryOr, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let or_parser = delimited(sp1, tag_no_case("or"), sp1);
    let (leftover_input, (left, right)) = separated_pair(
        parse_filter_token_or_subexpr,
        or_parser,
        parse_filter_token_or_subexpr,
    )
    .parse(input)?;

    Ok((leftover_input, QueryOr(left, right)))
}

#[cfg(test)]
pub mod test {
    use nom_language::error::VerboseError;

    use crate::query::any_tag_string::AnyTagString;
    use crate::query::or::QueryOr;
    use crate::query::parsing::assert_nom;
    use crate::query::parsing::or::parse_explicit_or;

    #[test]
    pub fn parse_explicit_or_test() {
        assert_nom(
            " oiia_oiia or maxwell ",
            parse_explicit_or,
            (
                " ",
                QueryOr(
                    AnyTagString::new1("oiia_oiia").into(),
                    AnyTagString::new1("maxwell").into(),
                ),
            ),
        );

        assert!(parse_explicit_or::<VerboseError<_>>(" \"oiia_oiia or maxwell\"").is_err())
    }
}
