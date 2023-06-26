use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{eof, map_res},
    multi::many_till,
};

use crate::{language::SyntaxPair, parser::NomError};

pub fn litral_string<'a>(line: &'a str, pair: &SyntaxPair) -> Option<&'a str> {
    let mut line = line;
    loop {
        if line.is_empty() {
            break None;
        }
        if let Ok((rest, (_, (tagging, flag)))) = many_till(
            anychar::<_, NomError>,
            alt((
                map_res(tag("\"\""), |c| Ok::<_, NomError>((c, false))),
                map_res(tag(pair.right), |c| Ok::<_, NomError>((c, true))),
                map_res(eof, |_| Ok::<_, NomError>(("", false))),
            )),
        )(line)
        {
            line = rest;
            if tagging.is_empty() {
                continue;
            } else if flag {
                break Some(line);
            }
        }
    }
}

pub fn normal_string<'a>(line: &'a str, pair: &SyntaxPair) -> Option<&'a str> {
    let mut line = line;
    loop {
        if line.is_empty() {
            break None;
        }
        if let Ok((rest, (_, (tagging, flag)))) = many_till(
            anychar::<_, NomError>,
            alt((
                map_res(tag("\\\\\""), |c| Ok::<_, NomError>((c, true))),
                map_res(tag("\\\""), |c| Ok::<_, NomError>((c, false))),
                map_res(tag(pair.right), |c| Ok::<_, NomError>((c, true))),
                map_res(eof, |_| Ok::<_, NomError>(("", false))),
            )),
        )(line)
        {
            line = rest;
            if tagging.is_empty() {
                continue;
            } else if flag {
                break Some(line);
            }
        }
    }
}
