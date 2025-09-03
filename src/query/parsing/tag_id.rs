use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser as _;
use nom::character::complete::i64;

use crate::query::eq_tag_id::EqAnyTagId;
use crate::query::parsing::sp;

pub(super) fn parse_tag_id(input: &str) -> IResult<&str, EqAnyTagId> {
    // Remove spaces
    let (leftover_input, _) = sp(input)?;
    // Grab the leading `tag_id:`
    let (leftover_input, _) = tag("tag_id:").parse(leftover_input)?;
    // Remove spaces
    let (leftover_input, _) = sp(leftover_input)?;
    // Grab the id
    let (leftover_input, id) = i64(leftover_input)?;

    Ok((leftover_input, EqAnyTagId::new1(id)))
}
