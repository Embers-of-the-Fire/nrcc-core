LanguageFile {
    extension: {% if extension is defined %} Some(FileItem {
        regex: &[{%- for reg in extension.regex %} regex::Regex::new("{{reg}}").unwrap(), {% endfor -%}],
        case_insensitive: &[{%- for ci in extension.case_insensitive %} "{{ci}}", {% endfor -%}],
        plain: &[{%- for ci in extension.plain %} "{{ci}}", {% endfor -%}],
    }) {% else %} None {% endif %},
    file_name: {% if file_name is defined %} Some(FileItem {
        regex: &[{%- for reg in file_name.regex %} regex::Regex::new("{{reg}}").unwrap(), {% endfor -%}],
        case_insensitive: &[{%- for ci in file_name.case_insensitive %} "{{ci}}", {% endfor -%}],
        plain: &[{%- for ci in file_name.plain %} "{{ci}}", {% endfor -%}],
    }) {% else %} None {% endif %},
}