mod tests {
    {% for language in languages %}
    #[test]
    fn test_language_{{language.name}}() {
        use crate::parser::{CoreParser, ParseResult, CommentResult};
        use crate::language::LanguageType;
        let parser = {
            let mut p = CoreParser::from_lang(&LanguageType::{{language.ident}});
            p.init_content(std::fs::read_to_string("{{language.file}}").unwrap().as_str());
            p
        };
        let result = parser.parse().unwrap();
        assert_eq!(result, {{language.predict}});
    }
    {% endfor %}
}