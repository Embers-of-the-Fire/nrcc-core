mod comment;
mod string;
mod sublang;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, line_ending, space1},
    combinator::{eof, map_res, opt, peek},
    multi::many_till,
    sequence::tuple,
    IResult,
};

use crate::{
    error::CoreError,
    language::{LanguageSyntax, LanguageType, SyntaxType},
};

use super::{tag_all, ParseResult, ParseState};

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

    pub fn parse(&self) -> Result<ParseResult, CoreError> {
        let mut result = ParseResult::default();
        let mut input = self.content.clone();
        let mut is_newline = true;
        #[allow(unused_assignments)]
        let mut prev_state = ParseState::Code;
        let mut prev_syntax = SyntaxType::Blank;
        loop {
            let (state, syntax, newline) = if let Ok((rest, (state, use_all_line))) =
                Self::parse_as_blank(&self.syntax, is_newline, input.as_str())
            {
                input = rest.to_string();
                if is_newline && use_all_line {
                    result.blank += 1;
                    (state, SyntaxType::Blank, true)
                } else if !is_newline && use_all_line {
                    match prev_syntax {
                        SyntaxType::Blank => result.blank += 1,
                        SyntaxType::Code => result.code += 1,
                        SyntaxType::Sublang
                        | SyntaxType::DocQuote
                        | SyntaxType::DocComment
                        | SyntaxType::NormalComment => {}
                    }
                    (state, SyntaxType::Blank, true)
                } else if !is_newline && !use_all_line {
                    (state, SyntaxType::Blank, false)
                } else {
                    (state, prev_syntax, false)
                }
            } else {
                let ipt = if is_newline {
                    if let Some(pf) = self.syntax.line_prefix {
                        let (ipt, _) = tag::<_, _, nom::error::Error<&str>>(pf)(input.as_str())
                            .map_err(|e| CoreError::SyntaxError(e.to_string()))?;
                        ipt.to_string()
                    } else {
                        input
                    }
                } else {
                    input
                };
                input = ipt;

                if let Ok((rest, (state, lang_type, iresult))) =
                    Self::parse_as_sublang(&self.syntax, input.as_str())
                {
                    input = rest.to_string();
                    result.join((lang_type, iresult));
                    result.all -= 1;
                    (state, SyntaxType::Sublang, false)
                } else if let Ok((rest, (state, line, syntax, newline))) =
                    Self::parse_as_comment(&self.syntax, input.as_str())
                {
                    match syntax {
                        SyntaxType::DocComment => {
                            result.comment.doc += line;
                            result.all += line - 1;
                        }
                        SyntaxType::NormalComment => {
                            result.comment.normal += line;
                            result.all += line - 1;
                        }
                        _ => unreachable!(),
                    }
                    input = rest.to_string();
                    (state, syntax, newline)
                } else if let Ok((rest, (state, line, syntax))) =
                    Self::parse_as_string(&self.syntax, input.as_str())
                {
                    match syntax {
                        SyntaxType::Code => {
                            result.code += line - 1;
                            result.all += line - 1;
                        }
                        SyntaxType::DocQuote => {
                            result.comment.doc_quote += line;
                            result.all += line - 1;
                        }
                        _ => panic!(),
                    }
                    input = rest.to_string();
                    (state, syntax, false)
                } else if let Ok((rest, (state, newline))) =
                    Self::parse_as_code(&self.syntax, input.as_str())
                {
                    if is_newline {
                        result.code += 1;
                    } else {
                        match prev_state {
                            ParseState::Code => {}
                            ParseState::Blank => result.code += 1,
                            _ => result.code += 1,
                        }
                    }
                    input = rest.to_string();
                    (state, SyntaxType::Code, newline)
                } else {
                    dbg!("unexpected");
                    (ParseState::Eoi, prev_syntax, false)
                }
            };
            prev_state = state;
            prev_syntax = syntax;
            is_newline = newline;
            if is_newline {
                result.all += 1;
            }
            /*
            dbg!((
                input.clone(),
                is_newline,
                prev_state,
                prev_syntax,
                result.clone()
            ));
            */
            if matches!(prev_state, ParseState::Eoi) {
                break;
            }
        }
        Ok(result)
    }

    /// 布尔值表示是否解决了尾换行。如果没有解决，实际物理行数为返回值-1
    pub fn parse_as_string<'a>(
        syntax: &LanguageSyntax,
        input: &'a str,
    ) -> IResult<&'a str, (ParseState, usize, SyntaxType)> {
        let (input, res) = string::parse_string(syntax, input)?;
        Ok((input, (res.0, res.1, res.2)))
    }

    /// 布尔值表示是否解决了尾换行。如果没有解决，实际行数为返回值+1
    pub fn parse_as_comment<'a>(
        syntax: &LanguageSyntax,
        input: &'a str,
    ) -> IResult<&'a str, (ParseState, usize, SyntaxType, bool)> {
        let (input, res) = comment::parse_comment(syntax, input)?;
        Ok((input, res))
    }

    pub fn parse_as_sublang<'a>(
        syntax: &LanguageSyntax,
        input: &'a str,
    ) -> IResult<&'a str, (ParseState, LanguageType, ParseResult)> {
        let (input, (tp, res)) = sublang::parse_sublang(syntax, input)?;
        Ok((input, (ParseState::Code, tp, res)))
    }

    /// 布尔值为真代表使用了整行
    pub fn parse_as_blank<'a>(
        syntax: &LanguageSyntax,
        is_newline: bool,
        input: &'a str,
    ) -> IResult<&'a str, (ParseState, bool)> {
        let input = if is_newline {
            if let Some(pf) = syntax.line_prefix {
                let (ipt, _) = tag(pf)(input)?;
                ipt
            } else {
                input
            }
        } else {
            input
        };
        if let Ok((input, (_, r))) = tuple((
            space1::<_, nom::error::Error<&str>>,
            opt(alt((line_ending, eof))),
        ))(input)
        {
            if let Some(d) = r {
                Ok((
                    input,
                    (
                        if d.is_empty() {
                            ParseState::Eoi
                        } else {
                            ParseState::Blank
                        },
                        true,
                    ),
                ))
            } else {
                Ok((input, (ParseState::Blank, false)))
            }
        } else if let Ok((input, _)) = line_ending::<_, nom::error::Error<&str>>(input) {
            Ok((input, (ParseState::Blank, true)))
        } else if let Ok((input, _)) = eof::<_, nom::error::Error<&str>>(input) {
            Ok((input, (ParseState::Eoi, true)))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    }

    /// 布尔值表示是否用完整行
    pub fn parse_as_code<'a>(
        syntax: &LanguageSyntax,
        input: &'a str,
    ) -> IResult<&'a str, (ParseState, bool)> {
        let (input, (c, _)) = many_till(
            anychar,
            peek(alt((
                eof,
                line_ending,
                map_res(tag_all(syntax.doc_quote_pairs, |p| p.left), |r| {
                    Ok::<_, nom::error::Error<&str>>(r.left)
                }),
                map_res(tag_all(syntax.literal_quote_pairs, |p| p.left), |r| {
                    Ok::<_, nom::error::Error<&str>>(r.left)
                }),
                map_res(tag_all(syntax.quote_pairs, |p| p.left), |r| {
                    Ok::<_, nom::error::Error<&str>>(r.left)
                }),
                map_res(tag_all(syntax.comment_pairs, |p| p.left), |r| {
                    Ok::<_, nom::error::Error<&str>>(r.left)
                }),
                map_res(tag_all(syntax.doc_comment_pairs, |p| p.left), |r| {
                    Ok::<_, nom::error::Error<&str>>(r.left)
                }),
                map_res(tag_all(syntax.doc_comment, |p| p), |r| {
                    Ok::<_, nom::error::Error<&str>>(*r)
                }),
                map_res(tag_all(syntax.simple_comment, |p| p), |r| {
                    Ok::<_, nom::error::Error<&str>>(*r)
                }),
                map_res(tag_all(syntax.sublang_pairs, |p| p.0.left), |r| {
                    Ok::<_, nom::error::Error<&str>>(r.0.left)
                }),
            ))),
        )(input)?;

        if c.is_empty() {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TakeUntil,
            )))
        } else if let Ok((rest, _)) = eof::<_, nom::error::Error<&str>>(input) {
            Ok((rest, (ParseState::Eoi, true)))
        } else if let Ok((rest, _)) = line_ending::<_, nom::error::Error<&str>>(input) {
            Ok((rest, (ParseState::Code, true)))
        } else {
            Ok((input, (ParseState::Code, false)))
        }
    }
}
