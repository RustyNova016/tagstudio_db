use nom::IResult;
use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::error::ContextError;
use nom::error::ParseError;

use crate::query::eq_tag_id::EqTagId;
use crate::query::parsing::sp;

pub(super) fn parse_tag_id<'a, E>(input: &'a str) -> IResult<&'a str, EqTagId, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    // Remove spaces
    let (leftover_input, _) = sp(input)?;
    // Grab the leading `tag_id:`
    let (leftover_input, _) = tag("tag_id:").parse(leftover_input)?;
    // Remove spaces
    let (leftover_input, _) = sp(leftover_input)?;
    // Grab the id
    let (leftover_input, id) = i64(leftover_input)?;

    Ok((leftover_input, EqTagId(id)))
}
