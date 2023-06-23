use crate::language::{LanguageSyntax, SyntaxType};
use crate::parser::{tag_all, ParseState};
use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending, space0},
    combinator::eof,
    multi::many_till,
    IResult,
};

pub fn parse_string<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize, SyntaxType)> {
    alt((
        map_res(
            |i| parse_lit_string(syntax, i),
            |res| Ok::<_, nom::error::Error<&str>>((res.0, res.1, SyntaxType::Code)),
        ),
        map_res(
            |i| parse_doc_string(syntax, i),
            |res| Ok::<_, nom::error::Error<&str>>((res.0, res.1, SyntaxType::DocQuote)),
        ),
        map_res(
            |i| parse_normal_string(syntax, i),
            |res| Ok::<_, nom::error::Error<&str>>((res.0, res.1, SyntaxType::Code)),
        ),
    ))(input)
}

fn parse_doc_string<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize)> {
    let input = if syntax.ignore_prefix_space {
        let (ipt, _) = space0(input)?;
        ipt
    } else {
        input
    };
    let (rest, syntax_pair) = tag_all(syntax.doc_quote_pairs, |p| p.left)(input)?;
    let (input, state, line) = {
        let mut line = 0_usize;
        let mut input = rest;
        loop {
            let (ipt, (_, tagging)) =
                many_till(anychar, alt((tag(syntax_pair.right), eof, line_ending)))(input)?;
            input = ipt;
            match tagging {
                "" => {
                    line += 1;
                    break (input, ParseState::Eoi, line);
                }
                "\r" | "\n" | "\r\n" => {
                    line += 1;
                    continue;
                }
                _ => {
                    line += 1;
                    break (input, ParseState::Code, line);
                }
            }
        }
    };
    Ok((input, (state, line)))
}

fn parse_lit_string<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize)> {
    let input = if syntax.ignore_prefix_space {
        let (ipt, _) = space0(input)?;
        ipt
    } else {
        input
    };
    let (rest, syntax_pair) = tag_all(syntax.literal_quote_pairs, |p| p.left)(input)?;
    let (input, state, line) = {
        let mut line = 0_usize;
        let mut input = rest;
        loop {
            let (ipt, (_, tagging)) =
                many_till(anychar, alt((tag(syntax_pair.right), eof, line_ending)))(input)?;
            input = ipt;
            match tagging {
                "" => {
                    line += 1;
                    break (input, ParseState::Eoi, line);
                }
                "\r" | "\n" | "\r\n" => {
                    line += 1;
                    continue;
                }
                _ => {
                    line += 1;
                    break (input, ParseState::Code, line);
                }
            }
        }
    };
    Ok((input, (state, line)))
}

fn parse_normal_string<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize)> {
    let input = if syntax.ignore_prefix_space {
        let (ipt, _) = space0(input)?;
        ipt
    } else {
        input
    };
    let (rest, syntax_pair) = tag_all(syntax.quote_pairs, |p| p.left)(input)?;
    let (input, state, line) = {
        let mut line = 0_usize;
        let mut input = rest;
        loop {
            let (ipt, (_, tagging)) = many_till(
                anychar,
                alt((tag("\\\""), tag(syntax_pair.right), eof, line_ending)),
            )(input)?;
            input = ipt;
            match tagging {
                "\\\"" => continue,
                "" => {
                    line += 1;
                    break (input, ParseState::Eoi, line);
                }
                "\r" | "\n" | "\r\n" => {
                    line += 1;
                    continue;
                }
                _ => {
                    line += 1;
                    break (input, ParseState::Code, line);
                }
            }
        }
    };
    Ok((input, (state, line)))
}
