use nom::IResult;
use nom::Parser as _;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::error::context;
use nom::sequence::preceded;

use crate::query::entry_search_query::EntrySearchQuery;
use crate::query::parsing::and::parse_explicit_and;
use crate::query::parsing::and::parse_implicit_and;
use crate::query::parsing::delimited_cut;
use crate::query::parsing::not::parse_explicit_not;
use crate::query::parsing::or::parse_explicit_or;
use crate::query::parsing::sp;
use crate::query::parsing::sp_arround;
use crate::query::parsing::tag_id::parse_tag_id;
use crate::query::parsing::tag_string::parse_tag_string;
use crate::query::parsing::tag_string::parse_tag_string_escaped;
use crate::query::tag_search_query::TagSearchQuery;

pub(in crate::query) fn parse_expression<'a, E>(
    input: &'a str,
) -> IResult<&'a str, EntrySearchQuery, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "expression",
        preceded(
            sp,
            alt((
                map(parse_explicit_or, EntrySearchQuery::from),
                map(parse_explicit_and, EntrySearchQuery::from),
                map(parse_implicit_and, EntrySearchQuery::from),
                parse_filter_token_or_subexpr,
            )),
        ),
    )
    .parse(input)
}

pub(super) fn parse_parentesis<'a, E>(input: &'a str) -> IResult<&'a str, EntrySearchQuery, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "parentesis",
        delimited_cut(
            sp_arround(char('(')),
            parse_expression,
            sp_arround(char(')')),
        ),
    )
    .parse(input)
}

pub(super) fn parse_filter_token_or_subexpr<'a, E>(
    input: &'a str,
) -> IResult<&'a str, EntrySearchQuery, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "filter_token_or_subexpr",
        preceded(sp, alt((parse_filter_token, parse_parentesis))),
    )
    .parse(input)
}

pub(super) fn parse_filter_token<'a, E>(input: &'a str) -> IResult<&'a str, EntrySearchQuery, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    context(
        "filter_token",
        preceded(
            sp,
            alt((
                parse_tag_id.map(|elem| TagSearchQuery::from(elem).into_entry_search_query()),
                map(parse_tag_string, EntrySearchQuery::from),
                map(parse_tag_string_escaped, EntrySearchQuery::from),
                map(parse_explicit_not, EntrySearchQuery::from),
            )),
        ),
    )
    .parse(input)
}

#[cfg(test)]
pub mod test {
    use crate::query::parsing::assert_nom;
    use crate::query::parsing::expression::parse_expression;
    use crate::query::tag_search_query::TagSearchQuery;

    #[test]
    pub fn parse_expression_test() {
        assert_nom(
            "(oiia_oiia (maxwell or dingus) )",
            parse_expression,
            (
                "",
                TagSearchQuery::eq_tag_string("oiia_oiia")
                    .add_children_tags_opaque()
                    .into_entry_search_query()
                    .and(
                        TagSearchQuery::eq_tag_string("maxwell")
                            .add_children_tags_opaque()
                            .into_entry_search_query()
                            .or(TagSearchQuery::eq_tag_string("dingus")
                                .add_children_tags_opaque()
                                .into_entry_search_query()),
                    ),
            ),
        );
    }
}
