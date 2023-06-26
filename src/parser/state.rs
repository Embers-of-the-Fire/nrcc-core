#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseState {
    Soi,
    Plain,
    Comment,
    String,
    SubLanguage,
    Eoi,
}
