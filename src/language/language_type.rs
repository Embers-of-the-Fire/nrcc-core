use serde::{Deserialize, Serialize};

use super::{LanguageSyntax, SyntaxPair, LanguageFile, FileItem};

include!(concat!(env!("OUT_DIR"), "/language_syntax_tera.rs"));

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum LanguageType {
    #[serde(rename = "Rust", alias = "Rust", alias = "Rustlang")]
    Rust,
}

impl LanguageType {
    pub const fn get_language_syntax(&self) -> LanguageSyntax {
        match self {
            Self::Rust => LanguageSyntax {
                line_prefix: None,
                ignore_prefix_space: true,
                block_pairs: &[SyntaxPair {
                    name: "simple",
                    left: "{",
                    right: "}",
                }],
                comment_pairs: &[SyntaxPair {
                    name: "simple",
                    left: "/*",
                    right: "*/",
                }],
                quote_pairs: &[SyntaxPair {
                    name: "simple",
                    left: "\"",
                    right: "\"",
                }],
                literal_quote_pairs: &[
                    SyntaxPair {
                        name: "1st",
                        left: "r#\"",
                        right: "\"#",
                    },
                    SyntaxPair {
                        name: "2nd",
                        left: "r##\"",
                        right: "\"##",
                    },
                    SyntaxPair {
                        name: "3rd",
                        left: "r###\"",
                        right: "\"###",
                    },
                ],
                simple_comment: &["//"],
                doc_comment: &["///"],
                doc_quote_pairs: &[SyntaxPair {
                    name: "doccomment",
                    left: "/dd",
                    right: "d/",
                }],
                sublang_pairs: &[(
                    SyntaxPair {
                        name: "subrust",
                        left: "<subrust>",
                        right: "</subrust>",
                    },
                    LanguageType::Rust,
                )],
                doc_comment_pairs: &[],
            },
        }
    }
}
*/