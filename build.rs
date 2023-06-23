use std::{
    collections::BTreeMap,
    env,
    fs::{self, File},
    path::Path,
    process::Command,
};

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=templates/language_syntax.tera.rs");
    println!("cargo:rerun-if-changed=templates/language_type.tera.rs");
    println!("cargo:rerun-if-changed=templates/tests.tera.rs");
    println!("cargo:rerun-if-changed=languages.yaml");
    println!("cargo:rerun-if-changed=tests/test_config.yaml");
    generate_language()?;
    generate_tests()?;
    Ok(())
}

fn generate_tests() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let tests: BTreeMap<String, TestLang> =
        serde_yaml::from_reader(File::open("./tests/test_config.yaml")?)?;
    let template = {
        let mut t = Tera::default();
        t.add_template_file("./templates/tests.tera.rs", Some("tests"))?;
        t
    };

    #[derive(Serialize)]
    struct Language {
        name: String,
        ident: String,
        file: String,
        predict: String,
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
                predict: d.stats.ser(0)
            })
        }
        v
    };

    let context = Context::from_serialize(TestContext {
        languages: context
    })?;

    let result = template.render("tests", &context)?;

    let output_path = Path::new(&out_dir).join("tests_tera.rs");
    fs::write(output_path.clone(), result)?;
    Command::new("rustfmt").args([output_path.to_str().unwrap()]).spawn()?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestLang {
    name: String,
    file: String,
    stats: TestData,
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
        let mut m{} = std::collections::HashMap::new();
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
            self.sub_language.iter().map(|(k, v)| format!("m{}.insert(LanguageType::{}, {});", level, k, v.ser(level + 1))).collect::<Vec<_>>().join(""),
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

fn generate_language() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let lang: BTreeMap<String, LanguageDefinition> =
        serde_yaml::from_reader(File::open("./languages.yaml")?)?;
    let template = {
        let mut t = Tera::default();
        t.add_template_file("./templates/language_type.tera.rs", Some("lang_type"))?;
        t
    };

    #[derive(Serialize)]
    struct Lang {
        ident: String,
        name: String,
        aliases: Vec<String>,
        syntax: String,
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
                syntax: generate_syntax(def.syntax)?,
            })
        }
        v
    };

    let context = Context::from_serialize(LangContext { languages: context })?;
    let result = template.render("lang_type", &context)?;

    let output_path = Path::new(&out_dir).join("language_syntax_tera.rs");
    fs::write(output_path.clone(), result)?;
    Command::new("rustfmt").args([output_path.to_str().unwrap()]).spawn()?;
    Ok(())
}

fn generate_syntax(syntax: LanguageSyntax) -> anyhow::Result<String> {
    let template = {
        let mut t = Tera::default();
        t.add_template_file("./templates/language_syntax.tera.rs", Some("syntax"))?;
        t
    };
    let context = Context::from_serialize(syntax)?;
    let result = template.render("syntax", &context)?;
    Ok(result)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LanguageDefinition {
    name: String,
    #[serde(default = "empty_vec")]
    alias: Vec<String>,
    syntax: LanguageSyntax,
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
