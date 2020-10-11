use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;
use super::color::Color;
use wasm_bindgen::JsValue;

// color/creating_reading.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn color(&self, v1: f64, v2: f64, v3: f64, v4: f64) -> Color {
		Color::new(self, vec!(v1, v2, v3, v4))
	}

	pub fn color_mode(&mut self, mode: String, max_1: JsValue, max_2: JsValue, max_3: JsValue, max_a: JsValue) {
		if mode == "rgb" || mode == "hsb" || mode == "hsl" {
			self.color_mode = mode.clone();

			let mut arg_count = 0;
			if !max_1.is_undefined() {
				arg_count += 1;
			}
			if !max_2.is_undefined() {
				arg_count += 1;
			}
			if !max_3.is_undefined() {
				arg_count += 1;
			}
			if !max_a.is_undefined() {
				arg_count += 1;
			}

			if arg_count != 0 {
				match arg_count {
					1 => {
						let max_1 = max_1.as_f64().expect("Invalid value type passed in to `max_1` argument");
						self.color_maxes.insert(mode, vec!(max_1, max_1, max_1, max_1));
					},
					3 => {
						let max_1 = max_1.as_f64().expect("Invalid value type passed in to `max_1` argument");
						let max_2 = max_2.as_f64().expect("Invalid value type passed in to `max_2` argument");
						let max_3 = max_3.as_f64().expect("Invalid value type passed in to `max_3` argument");

						let current_alpha = self.color_maxes.get(&mode.clone()).unwrap()[3];
						self.color_maxes.insert(mode, vec!(max_1, max_2, max_3, current_alpha));
					},
					4 => {
						let max_1 = max_1.as_f64().expect("Invalid value type passed in to `max_1` argument");
						let max_2 = max_2.as_f64().expect("Invalid value type passed in to `max_2` argument");
						let max_3 = max_3.as_f64().expect("Invalid value type passed in to `max_3` argument");
						let max_a = max_a.as_f64().expect("Invalid value type passed in to `max_a` argument");

						self.color_maxes.insert(mode, vec!(max_1, max_2, max_3, max_a));
					},
					_ => panic!(format!("Invalid number of arguments passed: {}", arg_count)),
				}
			}
		} else {
			panic!(format!("{} is not a valid color mode", mode));
		}
	}
}