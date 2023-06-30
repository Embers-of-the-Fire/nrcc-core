use std::collections::BTreeMap;

use nom::{
    branch::alt,
    character::complete::anychar,
    combinator::{eof, map_res},
    multi::many_till,
};

use crate::{
    language::LanguageSyntax,
    parser::{tag_all, NomError},
};

pub fn multi_comment<'a>(
    leading_map: &mut BTreeMap<&'a str, usize>,
    trailing_map: &mut BTreeMap<&'a str, usize>,
    syntax: &LanguageSyntax,
    line: &'a str,
) -> Option<&'a str> {
    let mut line = line;
    loop {
        if leading_map.values().sum::<usize>() == trailing_map.values().sum::<usize>() {
            break Some(line);
        }
        if line.is_empty() {
            break None;
        } else if let Ok((rest, (_, (tagging, flag)))) = many_till(
            anychar,
            alt((
                map_res(tag_all(syntax.doc_comment_pairs, |p| p.right), |r| {
                    Ok::<_, NomError>((r.right, false))
                }),
                map_res(tag_all(syntax.comment_pairs, |p| p.right), |r| {
                    Ok::<_, NomError>((r.right, false))
                }),
                map_res(tag_all(syntax.doc_comment_pairs, |p| p.left), |r| {
                    Ok::<_, NomError>((r.left, true))
                }),
                map_res(tag_all(syntax.comment_pairs, |p| p.left), |r| {
                    Ok::<_, NomError>((r.left, true))
                }),
                map_res(eof, |_| Ok::<_, NomError>(("", false))),
            )),
        )(line)
        {
            line = rest;
            if tagging.is_empty() {
                continue;
            } else if flag {
                *leading_map.entry(tagging).or_insert(0) += 1;
                continue;
            } else {
                *trailing_map.entry(tagging).or_insert(0) += 1;
                continue;
            }
        }
    }
}
