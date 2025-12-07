pub mod converter;
pub mod interpreter;
pub mod parser;
pub mod tokenizer;

use std::fs;

use rust_raytracer_core::SceneData;

use crate::{
    converter::openscad_convert, interpreter::openscad_interpret, parser::openscad_parse,
    tokenizer::openscad_tokenize,
};

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub enum OpenscadError {
    FileReadError(String, String),
}

pub fn openscad_file_to_scene_data(filename: &str) -> Result<SceneData, OpenscadError> {
    match fs::read_to_string(filename) {
        Ok(contents) => openscad_string_to_scene_data(&contents),
        Err(err) => Err(OpenscadError::FileReadError(
            filename.to_owned(),
            err.to_string(),
        )),
    }
}

pub fn openscad_string_to_scene_data(input: &str) -> Result<SceneData, OpenscadError> {
    let tokens = openscad_tokenize(input);
    let parse_results = openscad_parse(tokens);

    if !parse_results.errors.is_empty() {
        todo!("{:?}", parse_results.errors);
    }

    let interpret_results = openscad_interpret(parse_results.statements);
    if !interpret_results.errors.is_empty() {
        todo!("{:?}", interpret_results.errors);
    }

    let scene_data = openscad_convert(interpret_results.trees);

    Ok(scene_data)
}
