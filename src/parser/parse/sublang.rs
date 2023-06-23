use nom::{bytes::complete::{take_until, tag}, character::complete::space0, IResult};

use crate::{
    language::{LanguageSyntax, LanguageType},
    parser::{tag_all, ParseResult},
};

use super::CoreParser;

pub fn parse_sublang<'a>(syntax: &LanguageSyntax, input: &'a str) -> IResult<&'a str, (LanguageType, ParseResult)> {
    let (input, _) = space0(input)?;
    let (input, (pair, lang)) = tag_all(syntax.sublang_pairs, |p| p.0.left)(input)?;
    let (rest, content) = take_until(pair.right)(input)?;
    let mut lang_parser: CoreParser = CoreParser::from_lang(lang);
    lang_parser.init_content(content);
    let result = lang_parser.parse().unwrap();
    let (rest, _) = tag(pair.right)(rest)?;
    Ok((rest, (*lang, result)))
}
