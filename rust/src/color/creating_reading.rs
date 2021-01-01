use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;
use super::color::Color;
use wasm_bindgen::JsValue;

// color/creating_reading.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn color(&self, v1: JsValue, v2: JsValue, v3: JsValue, v4: JsValue) -> Color {
		Color::new(self, v1, v2, v3, v4)
	}

	pub fn red(&self, c: &Color) -> f64 {
		c.red()
	}

	pub fn green(&self, c: &Color) -> f64 {
		c.green()
	}

	pub fn blue(&self, c: &Color) -> f64 {
		c.blue()
	}

	pub fn alpha(&self, c: &Color) -> f64 {
		c.alpha()
	}

	pub fn hue(&self, c: &mut Color) -> f64 {
		c.hue()
	}

	pub fn saturation(&self, c: &mut Color) -> f64 {
		c.saturation()
	}

	pub fn brightness(&self, c: &mut Color) -> f64 {
		c.brightness()
	}

	pub fn lightness(&self, c: &mut Color) -> f64 {
		c.lightness()
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

	pub fn lerp_color(&self, c1: &mut Color, c2: &mut Color, amt: f64) -> Color {
		let from_array: Vec<f64>;
		let to_array: Vec<f64>;

		if self.color_mode == "rgb" {
			from_array = c1.levels.iter()
				.map(|level| {
					level / 255.0
				})
				.collect();

			to_array = c2.levels.iter()
				.map(|level| {
					level / 255.0
				})
				.collect();

		} else if self.color_mode == "hsb" {
			c1.brightness();
			c2.brightness();
			from_array = c1.hsba.as_ref().unwrap().to_vec();
			to_array = c2.hsba.as_ref().unwrap().to_vec();

		} else if self.color_mode == "hsl" {
			c1.lightness();
			c2.lightness();
			from_array = c1.hsla.as_ref().unwrap().to_vec();
			to_array = c2.hsla.as_ref().unwrap().to_vec();

		} else {
			panic!("{} cannot be used for interpolation.", self.color_mode);
		}

		let amount = amt.constrain(0.0, 1.0);

		let l0 = self.lerp(from_array[0], to_array[0], amount) * self.color_maxes.get(&self.color_mode).unwrap()[0];
		let l1 = self.lerp(from_array[1], to_array[1], amount) * self.color_maxes.get(&self.color_mode).unwrap()[1];
		let l2 = self.lerp(from_array[2], to_array[2], amount) * self.color_maxes.get(&self.color_mode).unwrap()[2];
		let l3 = self.lerp(from_array[3], to_array[3], amount) * self.color_maxes.get(&self.color_mode).unwrap()[3];
	
		self.color(JsValue::from_f64(l0), JsValue::from_f64(l1), JsValue::from_f64(l2),JsValue::from_f64(l3))
	}
}

trait Constrain {
	fn constrain(&self, min: f64, max: f64) -> f64;
}

impl Constrain for f64 {
	fn constrain(&self, min: f64, max: f64) -> f64 {
		let mut result = self;
		if self < &min {
			result = &min;
		} else if self > &max {
			result = &max;
		}

		*result
	}
}