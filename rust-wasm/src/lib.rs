mod utils;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct Color {
    CMYK: (u32, u32, u32, u32),
    RGB: (u32, u32, u32),
    hex: String,
    name: String,
    pinyin: String,
}

#[derive(PartialEq, Eq)]
#[wasm_bindgen]
pub enum SortBy {
    Desc,
    Asc,
}

#[wasm_bindgen]
pub struct Colors {
    color: Vec<Color>,
}

#[wasm_bindgen]
impl Colors {
    pub fn new(color_str: &str) -> Colors {
        let parsed_color: Result<Vec<Color>, _> = from_str(color_str);
        let color = parsed_color.unwrap_or_default();

        Colors { color }
    }

    pub fn get_colors(
        &self,
        name: Option<String>,
        sortby: Option<SortBy>,
    ) -> Result<JsValue, JsValue> {
        let colors: Vec<Color> = match name {
            Some(name) => {
                let mut filtered_colors: Vec<Color> = self
                    .color
                    .clone()
                    .into_iter()
                    .filter(|item| item.name.contains(&name))
                    .collect();

                match sortby {
                    Some(sortby) => {
                        match sortby {
                            SortBy::Desc => filtered_colors.sort_by(|a, b| {
                                let sum_a: u32 = a.RGB.0 + a.RGB.1 + a.RGB.2;
                                let sum_b: u32 = b.RGB.0 + b.RGB.1 + b.RGB.2;
                                sum_a.cmp(&sum_b)
                            }),
                            SortBy::Asc => filtered_colors.sort_by(|a, b| {
                                let sum_a: u32 = a.RGB.0 + a.RGB.1 + a.RGB.2;
                                let sum_b: u32 = b.RGB.0 + b.RGB.1 + b.RGB.2;
                                sum_b.cmp(&sum_a)
                            }),
                        }
                        filtered_colors
                    }
                    _ => vec![],
                }
            }
            None => vec![],
        };

        Ok(serde_wasm_bindgen::to_value(&colors)?)
    }
}

#[wasm_bindgen]
pub fn greet(s: &str) -> Colors {
    Colors::new(s)
}
