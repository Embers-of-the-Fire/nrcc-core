#[cfg(test)]
mod tests {
    use crate::parser::{CoreParser, ParseResult, CommentResult};
    use crate::language::LanguageType;
    use std::path::Path;
    {% for language in languages %}
    #[test]
    fn test_language_{{language.name}}() {
        let parser = {
            let mut p = CoreParser::from_lang(&LanguageType::{{language.ident}});
            p.init_content(std::fs::read_to_string("{{language.file}}").unwrap().as_str());
            p
        };
        let result = parser.parse().unwrap();
        assert_eq!(result, {{language.predict}});

        let file = LanguageType::{{language.ident}}.get_language_file();
        let to_parse = &[{%- for s in language.detect %}Path::new("{{s}}"), {% endfor -%}];
        for p in to_parse.iter() {
            let res = file.is_match_file(p);
            assert!(res);
        }
    }
    {% endfor %}
}