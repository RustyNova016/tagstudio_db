use nom::IResult;
use nom::Parser;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::combinator::cut;
use nom::error::ContextError;
use nom::error::ParseError;
use nom::error::context;
use nom::sequence::preceded;
use nom::sequence::terminated;

use crate::query::any_tag_string::AnyTagString;
use crate::query::parsing::sp;

pub(super) fn parse_tag_string<'a, E>(input: &'a str) -> IResult<&'a str, AnyTagString, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let parser = preceded(sp, take_while1(|c: char| c.is_alphanumeric() || c == '_'));
    let (leftover_input, output) = context("Tag String", parser).parse(input)?;

    Ok((leftover_input, AnyTagString::new1(output)))
}

pub(super) fn parse_tag_string_escaped<'a, E>(input: &'a str) -> IResult<&'a str, AnyTagString, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let inner_parser = take_while1(|c: char| c != '"');
    let end_quote = cut(terminated(inner_parser, char('\"')));
    let start_quote = preceded(char('\"'), end_quote);
    let space_removed = preceded(sp, start_quote);

    let (leftover_input, output) =
        context("parse_tag_string_escaped", space_removed).parse(input)?;

    Ok((leftover_input, AnyTagString::new1(output)))
}

#[cfg(test)]
pub mod test {
    use nom_language::error::VerboseError;

    use crate::query::any_tag_string::AnyTagString;
    use crate::query::parsing::tag_string::parse_tag_string;
    use crate::query::parsing::tag_string::parse_tag_string_escaped;

    #[test]
    pub fn parse_tag_string_test() {
        assert_eq!(
            parse_tag_string::<VerboseError<_>>(" maxwell ").unwrap(),
            (" ", AnyTagString::new1("maxwell"))
        );

        assert_eq!(
            parse_tag_string::<VerboseError<_>>(" oiia_oiia and maxwell ").unwrap(),
            (" and maxwell ", AnyTagString::new1("oiia_oiia"))
        );
    }

    #[test]
    pub fn parse_tag_string_escaped_test() {
        assert_eq!(
            parse_tag_string_escaped::<VerboseError<_>>(" \"maxwell\" ").unwrap(),
            (" ", AnyTagString::new1("maxwell"))
        );

        assert_eq!(
            parse_tag_string_escaped::<VerboseError<_>>(" \"oiia_oiia and maxwell\"").unwrap(),
            ("", AnyTagString::new1("oiia_oiia and maxwell"))
        )
    }

    #[test]
    pub fn parse_tag_string_or_escaped_test() {
        assert_eq!(
            parse_tag_string::<VerboseError<_>>(" maxwell ").unwrap(),
            (" ", AnyTagString::new1("maxwell"))
        );

        assert_eq!(
            parse_tag_string::<VerboseError<_>>(" oiia_oiia and maxwell ").unwrap(),
            (" and maxwell ", AnyTagString::new1("oiia_oiia"))
        );
        assert_eq!(
            parse_tag_string_escaped::<VerboseError<_>>(" \"maxwell\" ").unwrap(),
            (" ", AnyTagString::new1("maxwell"))
        );

        assert_eq!(
            parse_tag_string_escaped::<VerboseError<_>>(" \"oiia_oiia and maxwell\"").unwrap(),
            ("", AnyTagString::new1("oiia_oiia and maxwell"))
        )
    }
}
