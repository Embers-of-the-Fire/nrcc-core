use nom::bytes::complete::{tag, take_until};

use crate::{
    error::CoreError,
    language::{LanguageType, SyntaxPair},
    parser::{NomError, ParseResult},
};

use super::CoreParser;

pub fn split_sublang_part<'a>(
    pair: &SyntaxPair,
    sub_lang: &LanguageType,
    leading: &'a str,
    lines: &mut impl Iterator<Item = &'a str>,
) -> Result<(&'a str, ParseResult), CoreError> {
    let finish_tag = pair.right;
    let mut sublang_content: Vec<&'a str> = Vec::new();
    {
        let line = leading;
        if let Ok((rest, content)) = take_until::<_, _, NomError>(finish_tag)(line) {
            sublang_content.push(content);
            let syntax = sub_lang.get_language_syntax();
            let result = CoreParser::parse_lines(sublang_content.clone().into_iter(), &syntax)?;
            let (rest, _) = tag::<_, _, NomError>(finish_tag)(rest)?;
            return Ok((rest, result));
        } else {
            sublang_content.push(line);
        }
    }
    loop {
        if let Some(line) = lines.next() {
            if let Ok((rest, content)) = take_until::<_, _, NomError>(finish_tag)(line) {
                sublang_content.push(content);
                let syntax = sub_lang.get_language_syntax();
                let result = CoreParser::parse_lines(sublang_content.clone().into_iter(), &syntax)?;
                let (rest, _) = tag::<_, _, NomError>(finish_tag)(rest)?;
                break Ok((rest, result));
            } else {
                sublang_content.push(line);
                continue;
            }
        } else {
            break Err(CoreError::SyntaxError(
                "Ended sub language part.".to_string(),
            ));
        }
    }
}
