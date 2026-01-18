use caustic_core::utils::line_and_column_at_offset;
use caustic_openscad::Position;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct WasmPosition {
    pub start: usize,
    pub end: usize,
    pub start_line: Option<usize>,
    pub start_column: Option<usize>,
    pub filename: String,
}

impl From<&Position> for WasmPosition {
    fn from(value: &Position) -> Self {
        let code = value.source.get_code();
        let line_column = line_and_column_at_offset(code, value.start);

        Self {
            start: value.start,
            end: value.end,
            start_line: line_column.map(|c| c.0),
            start_column: line_column.map(|c| c.1),
            filename: value.source.get_filename().to_owned(),
        }
    }
}
