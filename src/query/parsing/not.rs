use nom::IResult;
use nom::Parser as _;
use nom::bytes::complete::tag_no_case;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::sequence::preceded;

use crate::query::not::QueryNot;
use crate::query::parsing::expression::parse_filter_token_or_subexpr;
use crate::query::parsing::sp;
use crate::query::parsing::sp1;

pub(super) fn parse_explicit_not<'a, E>(input: &'a str) -> IResult<&'a str, QueryNot, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let not_parser = delimited(sp, tag_no_case("not"), sp1);
    let (leftover_input, cond) =
        preceded(not_parser, parse_filter_token_or_subexpr).parse(input)?;

    Ok((leftover_input, QueryNot(cond)))
}

#[cfg(test)]
pub mod test {
    use nom_language::error::VerboseError;

    use crate::query::any_tag_string::AnyTagString;
    use crate::query::not::QueryNot;
    use crate::query::parsing::assert_nom;
    use crate::query::parsing::not::parse_explicit_not;

    #[test]
    pub fn parse_explicit_not_test() {
        assert_nom(
            " not maxwell ",
            parse_explicit_not,
            (" ", QueryNot(AnyTagString::new1("maxwell").into()).into()),
        );

        assert!(parse_explicit_not::<VerboseError<_>>(" \"not maxwell\"").is_err())
    }
}
