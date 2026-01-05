pub mod interpreter;
pub mod parser;
pub mod tokenizer;
pub mod value;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use thiserror::Error;

use crate::interpreter::InterpreterError;
use crate::{
    interpreter::{InterpreterResults, openscad_interpret},
    parser::openscad_parse,
    tokenizer::{TokenizerError, openscad_tokenize},
};

#[derive(Debug, PartialEq, Clone)]
pub struct WithPosition<T: PartialEq> {
    pub item: T,
    pub start: usize,
    pub end: usize,
}

impl<T: PartialEq> WithPosition<T> {
    pub fn new(item: T, start: usize, end: usize) -> Self {
        Self { item, start, end }
    }
}

#[derive(Error, Debug)]
pub enum OpenscadError {
    #[error("Invalid directory: {message}")]
    InvalidDirectoryError { message: String },
    #[error("File read error \"{filename}\": {message}")]
    FileReadError { filename: PathBuf, message: String },
    #[error("Tokenizer error: {0:?}")]
    TokenizerError(#[from] TokenizerError),
    #[error("Tokenizer error: {errors:?}")]
    InterpreterErrors { errors: Vec<InterpreterError> },
}

pub fn openscad_file_to_scene_data(filename: &Path) -> Result<InterpreterResults, OpenscadError> {
    let prev_current_dir =
        env::current_dir().map_err(|err| OpenscadError::InvalidDirectoryError {
            message: format!("could not get current directory: {err}"),
        })?;

    let dir = if let Some(dir) = filename.parent() {
        dir
    } else {
        return Err(OpenscadError::InvalidDirectoryError {
            message: format!("{filename:?} does not have a parent"),
        });
    };

    env::set_current_dir(dir).map_err(|err| OpenscadError::InvalidDirectoryError {
        message: format!("could not set current directory to: {dir:?}: {err}"),
    })?;

    let result = match fs::read_to_string(filename) {
        Ok(contents) => openscad_string_to_scene_data(&contents),
        Err(err) => Err(OpenscadError::FileReadError {
            filename: filename.to_owned(),
            message: err.to_string(),
        }),
    };
    env::set_current_dir(&prev_current_dir).map_err(|err| {
        OpenscadError::InvalidDirectoryError {
            message: format!("could not restore current directory to: {prev_current_dir:?}: {err}"),
        }
    })?;
    result
}

pub fn openscad_string_to_scene_data(input: &str) -> Result<InterpreterResults, OpenscadError> {
    let tokens = openscad_tokenize(input)?;
    let parse_results = openscad_parse(tokens);

    if !parse_results.errors.is_empty() {
        todo!("{:?}", parse_results.errors);
    }

    let interpret_results = openscad_interpret(parse_results.statements);
    if !interpret_results.errors.is_empty() {
        return Err(OpenscadError::InterpreterErrors {
            errors: interpret_results.errors,
        });
    }

    Ok(interpret_results)
}
