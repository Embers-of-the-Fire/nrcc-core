#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
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
    pub const fn all_language_syntax() -> &'static [(LanguageType, LanguageSyntax)] {
        &[
            {% for language in languages %}
            (Self::{{language.ident}}, {{language.syntax}}),
            {% endfor -%}
        ]
    }

    pub const fn get_name(&self) -> &'static str {
        match self {
            {% for language in languages %}
            Self::{{language.ident}} => "{{language.name}}",
            {% endfor -%}
        }
    }
    pub const fn all_name() -> &'static [(LanguageType, &'static str)] {
        &[
            {% for language in languages %}
            (Self::{{language.ident}}, "{{language.name}}"),
            {% endfor -%}
        ]
    }

    pub const fn get_language_file(&self) -> LanguageFile {
        match self {
            {% for language in languages %}
            Self::{{language.ident}} => {{language.file}},
            {% endfor -%}
        }
    }
    pub const fn all_language_file() -> &'static [(LanguageType, LanguageFile)] {
        &[
            {% for language in languages %}
            (Self::{{language.ident}}, {{language.file}}),
            {% endfor -%}
        ]
    }
}
