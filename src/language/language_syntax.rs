use super::LanguageType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyntaxType {
    Soi,
    Blank,
    Code,
    DocString,
    LitString,
    String,
    DocMultiComment,
    MultiComment,
    DocComment,
    SimpleComment,
    SubLanguage,
    Eoi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxPair {
    pub name: &'static str,
    pub left: &'static str,
    pub right: &'static str,
}

#[derive(Debug, Clone)]
pub struct LanguageSyntax {
    pub line_prefix: Option<&'static str>,
    pub ignore_prefix_space: bool,
    pub(crate) doc_comment: &'static [&'static str],
    pub(crate) simple_comment: &'static [&'static str],
    pub(crate) block_pairs: &'static [SyntaxPair],
    pub(crate) doc_comment_pairs: &'static [SyntaxPair],
    pub(crate) comment_pairs: &'static [SyntaxPair],
    pub(crate) literal_quote_pairs: &'static [SyntaxPair],
    pub(crate) quote_pairs: &'static [SyntaxPair],
    pub(crate) doc_quote_pairs: &'static [SyntaxPair],
    pub(crate) sublang_pairs: &'static [(SyntaxPair, LanguageType)],
}

impl LanguageSyntax {
    pub fn find_right_block_pair(&self, left: &str) -> Option<&SyntaxPair> {
        self.block_pairs.iter().find(|&p| p.left == left)
    }
    pub fn find_left_block_pair(&self, right: &str) -> Vec<&SyntaxPair> {
        let mut ls = Vec::new();
        for p in self.block_pairs.iter() {
            if p.right == right {
                ls.push(p);
            }
        }
        ls
    }

    pub fn find_right_comment_pair(&self, left: &str) -> Option<&SyntaxPair> {
        self.comment_pairs.iter().find(|&p| p.left == left)
    }
    pub fn find_left_comment_pair(&self, right: &str) -> Vec<&SyntaxPair> {
        let mut ls = Vec::new();
        for p in self.comment_pairs.iter() {
            if p.right == right {
                ls.push(p);
            }
        }
        ls
    }

    pub fn find_right_doc_comment_pair(&self, left: &str) -> Option<&SyntaxPair> {
        self.doc_comment_pairs.iter().find(|&p| p.left == left)
    }
    pub fn find_left_doc_comment_pair(&self, right: &str) -> Vec<&SyntaxPair> {
        let mut ls = Vec::new();
        for p in self.doc_comment_pairs.iter() {
            if p.right == right {
                ls.push(p);
            }
        }
        ls
    }

    pub fn find_right_quote_pair(&self, left: &str) -> Option<&SyntaxPair> {
        self.quote_pairs.iter().find(|&p| p.left == left)
    }
    pub fn find_left_quote_pair(&self, right: &str) -> Vec<&SyntaxPair> {
        let mut ls = Vec::new();
        for p in self.quote_pairs.iter() {
            if p.right == right {
                ls.push(p);
            }
        }
        ls
    }

    pub fn find_right_lit_quote_pair(&self, left: &str) -> Option<&SyntaxPair> {
        self.literal_quote_pairs.iter().find(|&p| p.left == left)
    }
    pub fn find_left_lit_quote_pair(&self, right: &str) -> Vec<&SyntaxPair> {
        let mut ls = Vec::new();
        for p in self.literal_quote_pairs.iter() {
            if p.right == right {
                ls.push(p);
            }
        }
        ls
    }

    pub fn find_right_sublang_pair(&self, left: &str) -> Option<&(SyntaxPair, LanguageType)> {
        self.sublang_pairs.iter().find(|&p| p.0.left == left)
    }
    pub fn find_left_sublang_pair(&self, right: &str) -> Option<&(SyntaxPair, LanguageType)> {
        self.sublang_pairs.iter().find(|&p| p.0.right == right)
    }
}
