use std::collections::HashMap;

use crate::language::SyntaxType;
use crate::parser::ParseState;
use crate::{language::LanguageSyntax, parser::tag_all};
use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending, space0},
    combinator::eof,
    multi::many_till,
    IResult,
};

/// 这里的布尔值表示是否占据整行
pub fn parse_comment<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize, SyntaxType, bool)> {
    if let Ok((s, (v1, v2, v3))) = parse_multi_comment(syntax, input) {
        Ok((s, (v1, v2, v3, false)))
    } else {
        match parse_single_comment(syntax, input) {
            Ok((s, (v1, v2, v3))) => Ok((s, (v1, v2, v3, true))),
            Err(e) => Err(e),
        }
    }
}

fn parse_multi_comment<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize, SyntaxType)> {
    let (rest, res, typed) =
        if let Ok((rest, res)) = tag_all(syntax.doc_comment_pairs, |p| p.left)(input) {
            Ok((rest, res, SyntaxType::DocComment))
        } else if let Ok((rest, res)) = tag_all(syntax.comment_pairs, |p| p.left)(input) {
            Ok((rest, res, SyntaxType::NormalComment))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }?;

    let mut line = 1_usize;
    let mut input = rest;
    let mut left_state: HashMap<&'static str, usize> = HashMap::new();
    left_state.insert(res.left, 1);
    let mut right_state: HashMap<&'static str, usize> = HashMap::new();
    loop {
        let ipt = if syntax.ignore_prefix_space {
            let (input, _) = space0(input)?;
            input
        } else {
            input
        };
        #[allow(clippy::type_complexity)]
        let (ipt, (_, (status, tagging))): (
            &str,
            (Vec<char>, (Option<bool>, &'static str)),
        ) = many_till(
            anychar,
            alt((
                map_res(tag_all(syntax.doc_comment_pairs, |p| p.left), |v| {
                    Ok::<_, nom::error::Error<&str>>((Some(true), v.left))
                }),
                map_res(tag_all(syntax.comment_pairs, |p| p.left), |v| {
                    Ok::<_, nom::error::Error<&str>>((Some(true), v.left))
                }),
                map_res(tag_all(syntax.doc_comment_pairs, |p| p.right), |v| {
                    Ok::<_, nom::error::Error<&str>>((Some(false), v.right))
                }),
                map_res(tag_all(syntax.comment_pairs, |p| p.right), |v| {
                    Ok::<_, nom::error::Error<&str>>((Some(false), v.right))
                }),
                map_res(eof::<&str, nom::error::Error<&str>>, |_| {
                    Ok::<_, nom::error::Error<&str>>((None, ""))
                }),
                map_res(line_ending::<&str, nom::error::Error<&str>>, |_| {
                    Ok::<_, nom::error::Error<&str>>((None, "\n"))
                }),
            )),
        )(ipt)?;
        input = ipt;
        match status {
            Some(status) => {
                if status {
                    *left_state.entry(tagging).or_insert(0) += 1;
                } else if let Some(b) = right_state.get_mut(tagging) {
                    *b += 1;
                } else {
                    let findv = {
                        let mut v = syntax.find_left_doc_comment_pair(tagging);
                        v.append(&mut syntax.find_left_comment_pair(tagging));
                        v
                    };
                    if !findv.is_empty() && findv.get(1).is_none() {
                        if let Some(t) = findv.get(0) {
                            if let Some(b) = left_state.get_mut(t.left) {
                                if *b > 0 {
                                    *b -= 1;
                                }
                            }
                        }
                    } else {
                        right_state.insert(tagging, 1);
                    }
                }
                if left_state.values().sum::<usize>() == right_state.values().sum::<usize>() {
                    break Ok((input, (ParseState::Code, line, typed)));
                }
            }
            None => {
                line += 1;
                if tagging.is_empty() {
                    if left_state.values().sum::<usize>() == right_state.values().sum::<usize>() {
                        break Ok((input, (ParseState::Eoi, line, typed)));
                    } else {
                        break Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Tag,
                        )));
                    }
                } else {
                    continue;
                }
            }
        }
    }
}

fn parse_single_comment<'a>(
    syntax: &LanguageSyntax,
    input: &'a str,
) -> IResult<&'a str, (ParseState, usize, SyntaxType)> {
    let input = if syntax.ignore_prefix_space {
        let (input, _) = space0(input)?;
        input
    } else {
        input
    };

    for tagging in syntax.doc_comment.iter() {
        if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>(*tagging)(input) {
            let (ipt, (_, tagged)) = many_till(anychar, alt((eof, line_ending)))(rest)?;
            if tagged.is_empty() {
                return Ok((ipt, (ParseState::Eoi, 1, SyntaxType::DocComment)));
            } else {
                return Ok((ipt, (ParseState::Code, 1, SyntaxType::DocComment)));
            }
        }
    }

    for tagging in syntax.simple_comment.iter() {
        if let Ok((rest, _)) = tag::<_, _, nom::error::Error<_>>(*tagging)(input) {
            let (ipt, (_, tagged)) = many_till(anychar, alt((eof, line_ending)))(rest)?;
            if tagged.is_empty() {
                return Ok((ipt, (ParseState::Eoi, 1, SyntaxType::NormalComment)));
            } else {
                return Ok((ipt, (ParseState::Code, 1, SyntaxType::NormalComment)));
            }
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}
