#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseState {
    Code,
    Comment,
    Blank,
    Eoi,
}
