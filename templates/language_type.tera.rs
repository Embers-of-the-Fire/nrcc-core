#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum LanguageType {
    {% for language in languages %}
    #[serde(rename="{{language.ident}}", {%- for alias in language.aliases %}alias="{{alias}}",{% endfor -%})] {{language.ident}},
    {% endfor %}
}

impl LanguageType {
    pub const fn get_language_syntax(&self) -> LanguageSyntax {
        match self {
            {% for language in languages %}
            Self::{{language.ident}} => {{language.syntax}},
            {% endfor -%}
        }
    }

    pub const fn get_name(&self) -> &'static str {
        match self {
            {% for language in languages %}
            Self::{{language.ident}} => "{{language.name}}",
            {% endfor -%}
        }
    }
}
