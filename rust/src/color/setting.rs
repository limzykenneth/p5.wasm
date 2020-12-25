use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;
use super::color::Color;

// color/setting.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn color_mode(&mut self, mode: String, max1: f64, max2: f64, max3: f64, maxA: f64){
		if mode == "rgb" || mode == "hsb" || mode == "hsl" {
			self.mode = mode;

			self.maxes.insert(mode, vec!(max1, max2, max3, maxA));
		}
	}
}