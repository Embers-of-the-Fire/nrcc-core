use std::{collections::BTreeMap, ops::{Add, AddAssign}};

use crate::language::LanguageType;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParseResult {
    pub code: usize,
    pub blank: usize,
    pub all: usize,
    pub comment: CommentResult,
    pub sub_language: BTreeMap<LanguageType, ParseResult>,
}

impl ParseResult {
    pub fn join(&mut self, other: (LanguageType, Self)) {
        self.sub_language.entry(other.0).or_insert(other.1.clone());
        self.code += other.1.code;
        self.blank += other.1.blank;
        self.all += other.1.all;
        self.comment += other.1.comment;
    }
}

impl Add for ParseResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            code: self.code + rhs.code,
            blank: self.blank + rhs.blank,
            all: self.all + rhs.all,
            comment: self.comment + rhs.comment,
            sub_language: {
                let mut map: BTreeMap<LanguageType, ParseResult> = BTreeMap::new();
                for (k, v) in self.sub_language.iter() {
                    map.entry(*k).or_insert(v.clone());
                }
                for (k, v) in rhs.sub_language.iter() {
                    *map.entry(*k).or_insert(ParseResult::default()) += v.clone();
                }
                map
            }
        }
    }
}

impl AddAssign for ParseResult {
    fn add_assign(&mut self, rhs: Self) {
        self.code += rhs.code;
        self.blank += rhs.blank;
        self.all += rhs.all;
        self.comment += rhs.comment;
        for (k, v) in rhs.sub_language.iter() {
            *self.sub_language.entry(*k).or_insert(ParseResult::default()) += v.clone()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CommentResult {
    pub doc: usize,
    pub normal: usize,
    pub doc_quote: usize,
}

impl Add for CommentResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            doc: self.doc + rhs.doc,
            normal: self.normal + rhs.normal,
            doc_quote: self.doc_quote + rhs.doc_quote, 
        }
    }
}

impl AddAssign for CommentResult {
    fn add_assign(&mut self, rhs: Self) {
        self.doc += rhs.doc;
        self.normal += rhs.normal;
        self.doc_quote += rhs.doc_quote;
    }
}
