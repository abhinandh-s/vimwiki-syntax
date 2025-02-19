#![allow(dead_code, unused_labels, unused_imports)]
use thiserror::Error;

/// error type for markdown parsing
#[derive(Debug, Error)]
pub enum NeoError {
    #[error("Parsing error at position {pos}: {message}")]
    ParseError { pos: usize, message: String },
    #[error("Unexpected end of input")]
    UnexpectedEOF,
}
