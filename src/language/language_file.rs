use std::path::Path;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct LanguageFile {
    pub extension: Option<FileItem>,
    pub file_name: Option<FileItem>,
}

impl LanguageFile {
    pub fn is_match_file(&self, file: &Path) -> bool {
        self.is_match(
            file.file_name().and_then(|e| e.to_str()),
            file.extension().and_then(|e| e.to_str()),
        )
    }
    pub fn is_match(&self, file_name: Option<&str>, ext: Option<&str>) -> bool {
        self.is_match_file_name(file_name) || self.is_match_extension(ext)
    }

    fn is_match_file_name(&self, file_name: Option<&str>) -> bool {
        if let Some(file_name) = file_name {
            if let Some(e) = &self.file_name {
                e.is_match(file_name)
            } else {
                false
            }
        } else {
            false
        }
    }
    fn is_match_extension(&self, ext: Option<&str>) -> bool {
        if let Some(ext) = ext {
            if let Some(e) = &self.extension {
                e.is_match(ext)
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileItem {
    pub regex: &'static [Regex],
    pub case_insensitive: &'static [&'static str],
    pub plain: &'static [&'static str],
}

impl FileItem {
    pub fn is_match(&self, content: &str) -> bool {
        self.plain.contains(&content)
            || self
                .case_insensitive
                .contains(&content.to_ascii_lowercase().as_str())
            || self.regex.iter().any(|r| r.is_match(content))
    }
}
