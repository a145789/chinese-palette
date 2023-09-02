use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tsify::Tsify;

use wasm_bindgen::prelude::*;

#[derive(Debug, Tsify, Serialize, Deserialize, Clone)]
pub struct Cmyk(u8, u8, u8, u8);

#[derive(Debug, Tsify, Serialize, Deserialize, Clone)]
pub struct Rgb(u8, u8, u8);

#[derive(Debug, Tsify, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct Color {
    CMYK: Cmyk,
    RGB: Rgb,
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

#[derive(Tsify, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct ColorResult {
    total: usize,
    data: Vec<Color>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Colors {
    color: Vec<Color>,
}

#[wasm_bindgen]
impl Colors {
    pub fn new(color_str: &str) -> Colors {
        let parsed_color: Result<Vec<Color>, _> = from_str(color_str);

        println!("{:?}", parsed_color);
        let color = parsed_color
            .unwrap_or_default()
            .iter()
            .map(|item| Color {
                CMYK: item.CMYK.clone(),
                RGB: item.RGB.clone(),
                hex: item.hex.clone(),
                name: item.name.clone(),
                pinyin: item.pinyin.clone(),
            })
            .collect();

        Colors { color }
    }

    #[wasm_bindgen()]
    pub fn get_colors(
        &self,
        name: Option<String>,
        sortby: Option<SortBy>,
        page: usize,
        limit: usize,
    ) -> ColorResult {
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
                                let sum_a: u8 = a.RGB.0 + a.RGB.1 + a.RGB.2;
                                let sum_b: u8 = b.RGB.0 + b.RGB.1 + b.RGB.2;
                                sum_a.cmp(&sum_b)
                            }),
                            SortBy::Asc => filtered_colors.sort_by(|a, b| {
                                let sum_a: u8 = a.RGB.0 + a.RGB.1 + a.RGB.2;
                                let sum_b: u8 = b.RGB.0 + b.RGB.1 + b.RGB.2;
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

        let total = colors.len();
        let data = if limit > total {
            colors
        } else {
            colors[page..limit].to_vec()
        };

        ColorResult { total, data }
    }
}

#[wasm_bindgen]
pub fn greet(s: &str) -> Colors {
    Colors::new(s)
}
