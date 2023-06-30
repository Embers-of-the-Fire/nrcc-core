#![allow(unused_assignments)]

mod comment;
mod string;
mod sublang;

use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, space0},
    combinator::{eof, map_res, peek},
    multi::many_till,
    sequence::tuple,
    IResult,
};

use crate::{
    error::CoreError,
    language::{LanguageSyntax, LanguageType, SyntaxType},
    parser::NomError,
};

use self::{
    comment::multi_comment,
    string::{litral_string, normal_string},
    sublang::split_sublang_part,
};

use super::{tag_all, ParseResult};

#[derive(Debug, Clone)]
pub struct CoreParser {
    content: String,
    syntax: LanguageSyntax,
}

impl CoreParser {
    pub fn from_lang(lang: &LanguageType) -> Self {
        Self {
            content: String::new(),
            syntax: lang.get_language_syntax(),
        }
    }

    pub fn init_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn split_lines(&self) -> impl Iterator<Item = &str> {
        self.content.split_terminator('\n').map(|line| {
            let Some(line) = line.strip_suffix('\r') else { return line };
            let Some(line) = line.strip_suffix('\n') else { return line };
            line
        })
    }

    pub fn parse(&self) -> Result<ParseResult, CoreError> {
        Self::parse_lines(self.split_lines(), &self.syntax)
    }

    pub fn parse_lines<'a>(
        lines: impl Iterator<Item = &'a str>,
        syntax: &LanguageSyntax,
    ) -> Result<ParseResult, CoreError> {
        let mut result = ParseResult::default();
        let mut lines = lines;
        let mut trailing_line: Option<&str> = None;
        let mut prev_is_code = false;
        loop {
            let (line, mut is_newline) = if let Some(t) = trailing_line {
                trailing_line = None;
                if t.is_empty() {
                    prev_is_code = false;
                    continue;
                }
                (t, false)
            } else if let Some(t) = lines.next() {
                result.all += 1;
                prev_is_code = false;
                (t, true)
            } else {
                break;
            };

            let line = if is_newline {
                if let Some(pf) = syntax.line_prefix {
                    let (input, _) = tag::<_, _, NomError>(pf)(line)?;
                    input
                } else {
                    line
                }
            } else {
                line
            };

            if line.is_empty() || tuple((space0::<_, NomError>, eof))(line).is_ok() {
                if !prev_is_code && is_newline {
                    result.blank += 1;
                }
            } else {
                let parsed: IResult<&str, (Vec<char>, SyntaxType)> = many_till(
                    anychar,
                    peek(alt((
                        map_res(tag_all(syntax.sublang_pairs, |p| p.0.left), |_| {
                            Ok::<_, NomError>(SyntaxType::SubLanguage)
                        }),
                        map_res(tag_all(syntax.doc_comment_pairs, |p| p.left), |_| {
                            Ok::<_, NomError>(SyntaxType::DocMultiComment)
                        }),
                        map_res(tag_all(syntax.comment_pairs, |p| p.left), |_| {
                            Ok::<_, NomError>(SyntaxType::MultiComment)
                        }),
                        map_res(tag_all(syntax.literal_quote_pairs, |p| p.left), |_| {
                            Ok::<_, NomError>(SyntaxType::LitString)
                        }),
                        map_res(tag_all(syntax.doc_quote_pairs, |p| p.left), |_| {
                            Ok::<_, NomError>(SyntaxType::DocString)
                        }),
                        map_res(tag_all(syntax.quote_pairs, |p| p.left), |_| {
                            Ok::<_, NomError>(SyntaxType::String)
                        }),
                        map_res(tag_all(syntax.doc_comment, |p| *p), |_| {
                            Ok::<_, NomError>(SyntaxType::DocComment)
                        }),
                        map_res(tag_all(syntax.simple_comment, |p| *p), |_| {
                            Ok::<_, NomError>(SyntaxType::SimpleComment)
                        }),
                    ))),
                )(line);
                if let Ok((rest, (chars, syntax_type))) = parsed {
                    if !prev_is_code
                        && !matches!(
                            syntax_type,
                            SyntaxType::DocString
                                | SyntaxType::LitString
                                | SyntaxType::String
                                | SyntaxType::Code
                        )
                        && !chars.is_empty()
                        && !space0::<_, NomError>(chars.into_iter().collect::<String>().as_str())?
                            .0
                            .is_empty()
                    {
                        prev_is_code = true;
                        is_newline = false;
                        result.code += 1;
                    }
                    match syntax_type {
                        SyntaxType::SubLanguage => {
                            let (rest, (pair, lang_type)) =
                                tag_all(syntax.sublang_pairs, |p| p.0.left)(rest)?;
                            let (trailing_s, parsed) =
                                split_sublang_part(pair, lang_type, rest, &mut lines)?;

                            trailing_line = Some(trailing_s);
                            result.join((*lang_type, parsed));

                            if !is_newline {
                                result.all -= 1;
                            }

                            prev_is_code = false;
                        }
                        SyntaxType::DocComment => {
                            trailing_line = None;
                            result.comment.doc += 1;
                        }
                        SyntaxType::SimpleComment => {
                            trailing_line = None;
                            result.comment.normal += 1;
                        }
                        SyntaxType::DocMultiComment => {
                            let (rest, pair) = tag_all(syntax.doc_comment_pairs, |p| p.left)(rest)?;
                            let mut leading_map: BTreeMap<&str, usize> = BTreeMap::new();
                            leading_map.insert(pair.left, 1);
                            let mut trailing_map: BTreeMap<&str, usize> = BTreeMap::new();
                            result.comment.doc += 1;
                            if let Some(trailing) =
                                multi_comment(&mut leading_map, &mut trailing_map, syntax, rest)
                            {
                                trailing_line = Some(trailing)
                            } else {
                                'doc_comment: loop {
                                    if let Some(comment_line) = lines.next() {
                                        result.all += 1;
                                        result.comment.doc += 1;
                                        if let Some(trailing) = multi_comment(
                                            &mut leading_map,
                                            &mut trailing_map,
                                            syntax,
                                            comment_line,
                                        ) {
                                            trailing_line = Some(trailing);
                                            break 'doc_comment;
                                        } else {
                                            continue 'doc_comment;
                                        }
                                    } else {
                                        return Err(CoreError::SyntaxError(
                                            "No Comment Ending found.".to_string(),
                                        ));
                                    }
                                }
                            }

                            prev_is_code = false;
                        }
                        SyntaxType::MultiComment => {
                            let (rest, pair) = tag_all(syntax.comment_pairs, |p| p.left)(rest)?;

                            let mut leading_map: BTreeMap<&str, usize> = BTreeMap::new();
                            leading_map.insert(pair.left, 1);
                            let mut trailing_map: BTreeMap<&str, usize> = BTreeMap::new();
                            result.comment.normal += 1;
                            if let Some(trailing) =
                                multi_comment(&mut leading_map, &mut trailing_map, syntax, rest)
                            {
                                trailing_line = Some(trailing)
                            } else {
                                'normal_comment: loop {
                                    if let Some(comment_line) = lines.next() {
                                        result.all += 1;
                                        result.comment.normal += 1;
                                        if let Some(trailing) = multi_comment(
                                            &mut leading_map,
                                            &mut trailing_map,
                                            syntax,
                                            comment_line,
                                        ) {
                                            trailing_line = Some(trailing);
                                            break 'normal_comment;
                                        } else {
                                            continue 'normal_comment;
                                        }
                                    } else {
                                        return Err(CoreError::SyntaxError(
                                            "No Comment Ending found.".to_string(),
                                        ));
                                    }
                                }
                            }

                            prev_is_code = false;
                        }
                        SyntaxType::DocString => {
                            let (rest, pair) = tag_all(syntax.doc_quote_pairs, |p| p.left)(rest)?;

                            if !prev_is_code {
                                result.code += 1;
                            }
                            if let Some(trailing) = normal_string(rest, pair) {
                                result.comment.doc_quote += 1;
                                trailing_line = Some(trailing);
                            } else {
                                result.comment.doc_quote += 1;
                                'quote: loop {
                                    if let Some(comment_line) = lines.next() {
                                        result.all += 1;
                                        result.code += 1;
                                        result.comment.doc_quote += 1;
                                        if let Some(trailing) = normal_string(comment_line, pair) {
                                            trailing_line = Some(trailing);
                                            break 'quote;
                                        } else {
                                            continue 'quote;
                                        }
                                    } else {
                                        return Err(CoreError::SyntaxError(
                                            "No Normal Document Quote Ending found.".to_string(),
                                        ));
                                    }
                                }
                            }

                            prev_is_code = true;
                        }
                        SyntaxType::LitString => {
                            let (rest, pair) =
                                tag_all(syntax.literal_quote_pairs, |p| p.left)(rest)?;

                            if !prev_is_code {
                                result.code += 1;
                            }
                            if let Some(trailing) = litral_string(rest, pair) {
                                trailing_line = Some(trailing)
                            } else {
                                'lit_quote: loop {
                                    if let Some(comment_line) = lines.next() {
                                        result.all += 1;
                                        result.code += 1;
                                        if let Some(trailing) = litral_string(comment_line, pair) {
                                            trailing_line = Some(trailing);
                                            break 'lit_quote;
                                        } else {
                                            continue 'lit_quote;
                                        }
                                    } else {
                                        return Err(CoreError::SyntaxError(
                                            "No Literal Quote Ending found.".to_string(),
                                        ));
                                    }
                                }
                            }

                            prev_is_code = true;
                        }
                        SyntaxType::String => {
                            let (rest, pair) = tag_all(syntax.quote_pairs, |p| p.left)(rest)?;

                            if !prev_is_code {
                                result.code += 1;
                            }
                            if let Some(trailing) = normal_string(rest, pair) {
                                trailing_line = Some(trailing)
                            } else {
                                'quote: loop {
                                    if let Some(comment_line) = lines.next() {
                                        result.all += 1;
                                        result.code += 1;
                                        if let Some(trailing) = normal_string(comment_line, pair) {
                                            trailing_line = Some(trailing);
                                            break 'quote;
                                        } else {
                                            continue 'quote;
                                        }
                                    } else {
                                        return Err(CoreError::SyntaxError(
                                            "No Normal Quote Ending found.".to_string(),
                                        ));
                                    }
                                }
                            }

                            prev_is_code = true;
                        }
                        SyntaxType::Soi
                        | SyntaxType::Blank
                        | SyntaxType::Code
                        | SyntaxType::Eoi => unreachable!(),
                    }
                } else if !prev_is_code {
                    trailing_line = None;
                    prev_is_code = false;
                    result.code += 1;
                }
            }

            // dbg!((line, &result, is_newline, trailing_line));
        }
        Ok(result)
    }
}
