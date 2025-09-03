use nom::IResult;
use nom::Parser;
use nom::bytes::complete::take_while;
use nom::combinator::cut;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
#[cfg(test)]
use nom_language::error::VerboseError;

pub mod and;
pub mod expression;
pub mod not;
pub mod or;
//pub mod tag_id;
pub mod tag_string;

/// Parse spaces
pub(super) fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c)).parse(i)
}

pub(super) fn sp1<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c)).parse(i)
}

pub(super) fn sp_arround<'a, E, F, O>(parser: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(sp, parser, sp)
}

// pub(super) fn unwrap_nom<I, O>(input, res: IResult<I, O, VerboseError<I>>) -> (I, O) {
//     match res {
//         Ok(val) => val,
//         Err(err) => {
//             panic!("Parsing error:\n{}", convert_error(err, e));
//         }
//     }
// }

#[cfg(test)]
pub(super) fn assert_nom<'a, T, F>(input: &'a str, parser: F, output: (&str, T))
where
    F: Fn(&'a str) -> IResult<&'a str, T, VerboseError<&'a str>>,
    T: Eq + std::fmt::Debug,
{
    use nom::Finish as _;

    let res = parser(input).finish();
    match res {
        Ok(val) => {
            assert_eq!(val, output);
        }
        Err(err) => {
            use nom_language::error::convert_error;

            panic!("Parsing error:\n{}", convert_error(input, err));
        }
    }
}

pub(super) fn delimited_cut<I, O, E: ParseError<I>, F, G, H>(
    first: F,
    second: G,
    third: H,
) -> impl Parser<I, Output = O, Error = E>
where
    F: Parser<I, Error = E>,
    G: Parser<I, Output = O, Error = E>,
    H: Parser<I, Error = E>,
{
    preceded(first, cut(terminated(second, third)))
}

#[cfg(test)]
pub mod tests {}
