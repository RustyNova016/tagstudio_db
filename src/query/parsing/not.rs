use nom::IResult;
use nom::Parser as _;
use nom::bytes::complete::tag_no_case;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::sequence::preceded;

use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::parsing::expression::parse_filter_token_or_subexpr;
use crate::query::parsing::sp;
use crate::query::parsing::sp1;

pub(super) fn parse_explicit_not<'a, E>(input: &'a str) -> IResult<&'a str, EntrySearchQuery, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let not_parser = delimited(sp, tag_no_case("not"), sp1);
    let (leftover_input, cond) =
        preceded(not_parser, parse_filter_token_or_subexpr).parse(input)?;

    Ok((leftover_input, cond.invert()))
}

#[cfg(test)]
pub mod test {
    use nom_language::error::VerboseError;

    use crate::query::parsing::assert_nom;
    use crate::query::parsing::not::parse_explicit_not;
    use crate::query::tag_search_query::TagSearchQuery;

    #[test]
    pub fn parse_explicit_not_test() {
        assert_nom(
            " not maxwell ",
            parse_explicit_not,
            (
                " ",
                TagSearchQuery::eq_tag_string("maxwell")
                    .add_children_tags_opaque()
                    .into_entry_search_query()
                    .invert(),
            ),
        );

        assert!(parse_explicit_not::<VerboseError<_>>(" \"not maxwell\"").is_err())
    }
}
