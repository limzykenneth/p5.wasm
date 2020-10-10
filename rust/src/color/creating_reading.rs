use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;
use super::color::Color;

// color/creating_reading.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn color(&self, v1: f64, v2: f64, v3: f64, v4: f64) -> Color {
		Color::new(vec!(v1, v2, v3, v4))
	}
}