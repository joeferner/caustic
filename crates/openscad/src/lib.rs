pub mod interpreter;
pub mod parser;
pub mod source;
pub mod tokenizer;
pub mod value;

use std::fmt::Display;
use std::sync::Arc;

use caustic_core::Random;
use thiserror::Error;

use crate::interpreter::InterpreterError;
use crate::parser::ParserError;
use crate::source::Source;
use crate::{
    interpreter::{InterpreterResults, openscad_interpret},
    parser::openscad_parse,
    tokenizer::{TokenizerError, openscad_tokenize},
};

#[derive(Debug, Clone)]
pub struct WithPosition<T: PartialEq> {
    pub item: T,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub source: Arc<Box<dyn Source>>,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source.to_string(self.start, self.end))
    }
}

impl<T: PartialEq> WithPosition<T> {
    pub fn new(item: T, position: Position) -> Self {
        Self { item, position }
    }

    fn equals(&self, other: &WithPosition<T>) -> bool {
        self.item.eq(&other.item)
            && self.position.start == other.position.start
            && self.position.end == other.position.end
            && self
                .position
                .source
                .equals(other.position.source.as_ref().as_ref())
    }
}

impl<T: PartialEq> PartialEq for WithPosition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

#[derive(Error, Debug)]
pub enum OpenscadError {
    #[error("Source error: {0}")]
    SourceError(String),
    #[error("Tokenizer error: {0:?}")]
    TokenizerError(#[from] TokenizerError),
    #[error("Parser error: {errors:?}")]
    ParserErrors { errors: Vec<ParserError> },
    #[error("Tokenizer error: {errors:?}")]
    InterpreterErrors { errors: Vec<InterpreterError> },
}

pub fn run_openscad(
    source: Arc<Box<dyn Source>>,
    random: Arc<dyn Random>,
) -> Result<InterpreterResults, OpenscadError> {
    let tokens = openscad_tokenize(source.clone())?;
    let parse_results = openscad_parse(tokens, source);

    if !parse_results.errors.is_empty() {
        return Err(OpenscadError::ParserErrors {
            errors: parse_results.errors,
        });
    }

    let interpret_results = openscad_interpret(parse_results.statements, random);
    if !interpret_results.errors.is_empty() {
        return Err(OpenscadError::InterpreterErrors {
            errors: interpret_results.errors,
        });
    }

    Ok(interpret_results)
}
