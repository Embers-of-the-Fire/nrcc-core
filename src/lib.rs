pub mod error;
pub mod language;
pub mod parser;

include!(concat!(env!("OUT_DIR"), "/tests_tera.rs"));
