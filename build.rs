use std::{
    collections::BTreeMap,
    env,
    fs::{self, File},
    path::Path,
};

use regex::Regex;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

fn main() -> anyhow::Result<()> {
    let template = {
        let mut t = Tera::default();
        t.add_template_file("./templates/tests.tera.rs", Some("tests"))
            .expect("Error loading test template");
        t.add_template_file("./templates/language_type.tera.rs", Some("lang_type"))
            .expect("Error loading languages template");
        t.add_template_file("./templates/language_syntax.tera.rs", Some("syntax"))
            .expect("Error loading languages syntax template");
        t.add_template_file("./templates/language_file.tera.rs", Some("file"))
            .expect("Error loading languages file definition template");
        t
    };

    generate_language(&template)?;
    if Path::new("./tests/").exists() {
        generate_tests(&template)?;
    } else {
        fs::write(Path::new(&env::var("OUT_DIR")?).join("tests_tera.rs"), "")?;
    }
    Ok(())
}

fn generate_tests(template: &Tera) -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let tests: BTreeMap<String, TestLang> = serde_yaml::from_reader(
        File::open("./tests/test_config.yaml").expect("Error loading test config"),
    )
    .expect("Error loading test config");

    #[derive(Serialize)]
    struct Language {
        name: String,
        ident: String,
        file: String,
        predict: String,
        detect: Vec<String>,
    }
    #[derive(Serialize)]
    struct TestContext {
        languages: Vec<Language>,
    }

    let context = {
        let mut v = Vec::new();
        for (k, d) in tests {
            v.push(Language {
                ident: k,
                name: d.name,
                file: d.file,
                predict: d.stats.ser(0),
                detect: d.file_detect
            })
        }
        v
    };

    let context = Context::from_serialize(TestContext { languages: context })?;

    let result = template
        .render("tests", &context)
        .expect("Error rendering test config");

    let output_path = Path::new(&out_dir).join("tests_tera.rs");
    fs::write(output_path, result).expect("Error writing test config");

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestLang {
    name: String,
    file: String,
    stats: TestData,
    #[serde(default = "empty_vec")]
    file_detect: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestData {
    code: usize,
    blank: usize,
    all: usize,
    sub_language: BTreeMap<String, TestData>,
    comment: TestCommentData,
}

impl TestData {
    fn ser(&self, level: usize) -> String {
        format!(
            "ParseResult {{
    code: {},
    blank: {},
    all: {},
    comment: CommentResult {{
        doc: {},
        normal: {},
        doc_quote: {},
    }},
    sub_language: {{
        #[allow(unused_mut)]
        let mut m{} = std::collections::BTreeMap::new();
        {}
        m{}
    }}
}}",
            self.code,
            self.blank,
            self.all,
            self.comment.doc,
            self.comment.normal,
            self.comment.doc_quote,
            level,
            self.sub_language
                .iter()
                .map(|(k, v)| format!(
                    "m{}.insert(LanguageType::{}, {});",
                    level,
                    k,
                    v.ser(level + 1)
                ))
                .collect::<Vec<_>>()
                .join(""),
            level
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCommentData {
    doc: usize,
    normal: usize,
    doc_quote: usize,
}

fn generate_language(template: &Tera) -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Error loading output directory");
    let lang: BTreeMap<String, LanguageDefinition> = serde_yaml::from_reader(
        File::open("./languages.yaml").expect("Error loading languages config"),
    )?;

    #[derive(Serialize)]
    struct Lang {
        ident: String,
        name: String,
        aliases: Vec<String>,
        syntax: String,
        file: String,
    }
    #[derive(Serialize)]
    struct LangContext {
        languages: Vec<Lang>,
    }

    let context = {
        let mut v = Vec::new();
        for (ident, def) in lang {
            v.push(Lang {
                ident,
                aliases: {
                    let mut iv = def.alias;
                    iv.push(def.name.clone());
                    iv
                },
                name: def.name,
                syntax: generate_syntax(template, def.syntax)?,
                file: {
                    def.file.extension.check_regex()?;
                    def.file.file_name.check_regex()?;
                    generate_file_definition(template, def.file)?
                },
            })
        }
        v
    };

    let context = Context::from_serialize(LangContext { languages: context })
        .expect("Error loading languages context");
    let result = template
        .render("lang_type", &context)
        .expect("Error rendering languages template");

    let output_path = Path::new(&out_dir).join("language_syntax_tera.rs");
    fs::write(output_path, result).expect("Error writing languages config");
    Ok(())
}

fn generate_file_definition(template: &Tera, file: LanguageFile) -> anyhow::Result<String> {
    let context =
        Context::from_serialize(file).expect("Error loading languages file definition context");
    let result = template
        .render("file", &context)
        .expect("Error rendering languages file definition template");
    Ok(result)
}

fn generate_syntax(template: &Tera, syntax: LanguageSyntax) -> anyhow::Result<String> {
    let context = Context::from_serialize(syntax).expect("Error loading languages syntax context");
    let result = template
        .render("syntax", &context)
        .expect("Error rendering languages syntax template");
    Ok(result)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LanguageDefinition {
    name: String,
    #[serde(default = "empty_vec")]
    alias: Vec<String>,
    syntax: LanguageSyntax,
    file: LanguageFile,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LanguageFile {
    extension: FileSetting,
    file_name: FileSetting,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FileSetting {
    #[serde(default = "empty_vec")]
    regex: Vec<String>,
    #[serde(default = "empty_vec")]
    case_insensitive: Vec<String>,
    #[serde(default = "empty_vec")]
    plain: Vec<String>,
}

impl FileSetting {
    fn check_regex(&self) -> Result<(), regex::Error> {
        for regex in self.regex.iter() {
            match Regex::new(regex) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LanguageSyntax {
    #[serde(default = "empty_vec")]
    block: Vec<(String, String)>,
    line_prefix: Option<String>,
    #[serde(default = "true_func")]
    ignore_prefix_space: bool,
    comment: LanguageComment,
    quote: LanguageQuote,
    #[serde(default = "empty_vec")]
    sub_language: Vec<(String, String, String)>,
}

fn true_func() -> bool {
    true
}

fn empty_vec<T>() -> Vec<T> {
    vec![]
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LanguageComment {
    #[serde(default = "empty_vec")]
    multi: Vec<(String, String)>,
    #[serde(default = "empty_vec")]
    single: Vec<String>,
    #[serde(default = "empty_vec")]
    doc: Vec<String>,
    #[serde(default = "empty_vec")]
    doc_multi: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LanguageQuote {
    #[serde(default = "empty_vec")]
    normal: Vec<(String, String)>,
    #[serde(default = "empty_vec")]
    literal: Vec<(String, String)>,
    #[serde(default = "empty_vec")]
    doc: Vec<(String, String)>,
}
