use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct MatchRange {
    start: usize,
    end: usize,
    value: String,
}

#[wasm_bindgen]
pub fn match_regex(pattern: &str, text: &str) -> Result<JsValue, JsValue> {
    let re = Regex::new(pattern).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let matches: Vec<MatchRange> = re
        .find_iter(text)
        .map(|m| {
            let range = m.range();
            MatchRange {
                start: range.start,
                end: range.end,
                value: m.as_str().to_string(),
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&matches).map_err(|e| JsValue::from_str(&e.to_string()))
}
