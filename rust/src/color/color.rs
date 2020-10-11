use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use super::color_conversion;
use wasm_bindgen::JsValue;
use crate::p5_wasm::P5Wasm;

// color/p5.Color.js
#[wasm_bindgen]
pub struct Color {
	array: Vec<f64>,
	mode: String,
	maxes: HashMap< String, Vec<f64> >,
	hsba: Option< Vec<f64> >,
	hsla: Option< Vec<f64> >,
}

#[wasm_bindgen]
impl Color {
	pub fn new(inst: &P5Wasm, vals: Vec<f64>) -> Color {
		Color {
			array: vals.iter().map(|v| v / 255.0).collect(),
			mode: inst.color_mode.clone(),
			maxes: inst.color_maxes.clone(),
			hsba: None,
			hsla: None,
		}
	}

	pub fn to_string(&mut self, f: JsValue) -> String {
		let format;
		let val;
		if f.is_undefined() {
			format = "rgba";
		} else if f.is_string() {
			val = f.as_string().unwrap();
			format = &val;
		} else {
			panic!("Invalid value passed to to_string() function");
		}

		let arr = &self.array;

		match format {
			"#rrggbb" => {
				let r = match Some(format_radix(arr[0] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};
				let g = match Some(format_radix(arr[1] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};
				let b = match Some(format_radix(arr[2] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};

				format!("#{}{}{}", r, g, b)
			}
			"#rrggbbaa" => {
				let r = match Some(format_radix(arr[0] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};
				let g = match Some(format_radix(arr[1] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};
				let b = match Some(format_radix(arr[2] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};
				let a = match Some(format_radix(arr[3] as u32 * 255, 16)) {
					Some(x) if x.chars().count() < 2 => "0".to_owned() + &x,
					Some(x) => x,
					None => String::from(""),
				};

				format!("#{}{}{}{}", r, g, b, a)
			}
			"#rgb" => {
				let r = format_radix(arr[0].round() as u32 * 15, 16);
				let g = format_radix(arr[1].round() as u32 * 15, 16);
				let b = format_radix(arr[2].round() as u32 * 15, 16);

				format!("#{}{}{}", r, g, b)
			}
			"#rgba" => {
				let r = format_radix(arr[0].round() as u32 * 15, 16);
				let g = format_radix(arr[1].round() as u32 * 15, 16);
				let b = format_radix(arr[2].round() as u32 * 15, 16);
				let a = format_radix(arr[3].round() as u32 * 15, 16);

				format!("#{}{}{}{}", r, g, b, a)
			}
			"rgb" => {
				format!("rgb({}, {}, {})", arr[0] * 255.0, arr[1] * 255.0, arr[2] * 255.0)
			}
			"rgb%" => {
				let r = (arr[0] * 100.0).to_precision(3);
				let g = (arr[1] * 100.0).to_precision(3);
				let b = (arr[2] * 100.0).to_precision(3);

				format!("rgb({}%, {}%, {}%)", r, g, b)
			}
			"rgba%" => {
				let r = (arr[0] * 100.0).to_precision(3);
				let g = (arr[1] * 100.0).to_precision(3);
				let b = (arr[2] * 100.0).to_precision(3);
				let a = (arr[2] * 100.0).to_precision(3);

				format!("rgba({}%, {}%, {}%, {}%)", r, g, b, a)
			}
			"hsb" | "hsv" => {
				if self.hsba.is_none() {
					self.hsba = Some(color_conversion::rgba_to_hsba(arr.to_vec()));
				}
				let hsba = self.hsba.as_ref().unwrap();
				let maxes = self.maxes.get("hsb").unwrap();

				format!("hsb({}, {}, {})", hsba[0] * maxes[0], hsba[1] * maxes[1], hsba[2] * maxes[2])
			}
			"hsb%" | "hsv%" => {
				if self.hsba.is_none() {
					self.hsba = Some(color_conversion::rgba_to_hsba(arr.to_vec()));
				}
				let hsba = self.hsba.as_ref().unwrap();

				let h = (hsba[0] * 100.0).to_precision(3);
				let s = (hsba[1] * 100.0).to_precision(3);
				let b = (hsba[2] * 100.0).to_precision(3);

				format!("hsb({}%, {}%, {}%)", h, s, b)
			}
			"hsba" | "hsva" => {
				if self.hsba.is_none() {
					self.hsba = Some(color_conversion::rgba_to_hsba(arr.to_vec()));
				}
				let hsba = self.hsba.as_ref().unwrap();
				let maxes = self.maxes.get("hsb").unwrap();

				format!("hsba({}, {}, {}, {})", hsba[0] * maxes[0], hsba[1] * maxes[1], hsba[2] * maxes[2], hsba[3] * maxes[3])
			}
			"hsba%" | "hsva%" => {
				if self.hsba.is_none() {
					self.hsba = Some(color_conversion::rgba_to_hsba(arr.to_vec()));
				}
				let hsba = self.hsba.as_ref().unwrap();

				let h = (hsba[0] * 100.0).to_precision(3);
				let s = (hsba[1] * 100.0).to_precision(3);
				let b = (hsba[2] * 100.0).to_precision(3);
				let a = (arr[2] * 100.0).to_precision(3);

				format!("hsba({}%, {}%, {}%, {}%)", h, s, b, a)
			}
			"hsl" => {
				if self.hsla.is_none() {
					self.hsla = Some(color_conversion::rgba_to_hsla(arr.to_vec()));
				}

				let hsla = self.hsla.as_ref().unwrap();
				let maxes = self.maxes.get("hsl").unwrap();

				format!("hsl({}, {}, {})", hsla[0] * maxes[0], hsla[1] * maxes[1], hsla[2] * maxes[2])
			}
			"hsl%" => {
				if self.hsla.is_none() {
					self.hsla = Some(color_conversion::rgba_to_hsla(arr.to_vec()));
				}
				let hsla = self.hsla.as_ref().unwrap();

				let h = (hsla[0] * 100.0).to_precision(3);
				let s = (hsla[1] * 100.0).to_precision(3);
				let l = (hsla[2] * 100.0).to_precision(3);

				format!("hsl({}%, {}%, {}%)", h, s, l)
			}
			"hsla" => {
				if self.hsla.is_none() {
					self.hsla = Some(color_conversion::rgba_to_hsla(arr.to_vec()));
				}
				let hsla = self.hsla.as_ref().unwrap();
				let maxes = self.maxes.get("hsl").unwrap();

				format!("hsla({}, {}, {}, {})", hsla[0] * maxes[0], hsla[1] * maxes[1], hsla[2] * maxes[2], hsla[3] * maxes[3])
			}
			"hsla%" => {
				if self.hsla.is_none() {
					self.hsla = Some(color_conversion::rgba_to_hsla(arr.to_vec()));
				}
				let hsla = self.hsla.as_ref().unwrap();

				let h = (hsla[0] * 100.0).to_precision(3);
				let s = (hsla[1] * 100.0).to_precision(3);
				let l = (hsla[2] * 100.0).to_precision(3);
				let a = (arr[2] * 100.0).to_precision(3);

				format!("hsla({}%, {}%, {}%, {}%)", h, s, l, a)
			}
			"rgba" | _ => {
				format!("rgba({}, {}, {}, {})", arr[0] * 255.0, arr[1] * 255.0, arr[2] * 255.0, arr[3])
			}
		}
	}

	pub fn set_red(&mut self, new_red: f64) {
		self.array[0] = new_red / self.maxes.get("rgb").unwrap()[0];
	}

	pub fn set_green(&mut self, new_green: f64) {
		self.array[1] = new_green / self.maxes.get("rgb").unwrap()[1];
	}

	pub fn set_blue(&mut self, new_blue: f64) {
		self.array[2] = new_blue / self.maxes.get("rgb").unwrap()[2];
	}

	pub fn set_alpha(&mut self, new_alpha: f64) {
		self.array[3] = new_alpha / self.maxes.get("rgb").unwrap()[3];
	}

	pub fn red(&self) -> f64 {
		self.array[0] * self.maxes.get("rgb").unwrap()[0]
	}

	pub fn green(&self) -> f64 {
		self.array[1] * self.maxes.get("rgb").unwrap()[1]
	}

	pub fn blue(&self) -> f64 {
		self.array[2] * self.maxes.get("rgb").unwrap()[2]
	}

	pub fn alpha(&self) -> f64 {
		self.array[3] * self.maxes.get(&self.mode).unwrap()[3]
	}
}

fn format_radix(mut x: u32, radix: u32) -> String {
	let mut result = vec![];

	loop {
		let m = x % radix;
		x = x / radix;

		// will panic if you use a bad radix (< 2 or > 36).
		result.push(std::char::from_digit(m, radix).unwrap());
	    if x == 0 {
			break;
		}
	}
	result.into_iter().rev().collect()
}

trait ToPrecision {
	fn to_precision(&self, n: u32) -> String;
}

impl ToPrecision for f64 {
	fn to_precision(&self, n: u32) -> String {
		if *self == 0.0 {
	        return 0.0.to_string();
	    }

	    let d = self.abs().log10().ceil();
	    let power = n - d as u32;
	    let magnitude = 10_i32.pow(power);
	    let shifted = (*self * magnitude as f64).round();
	    let ret = shifted / magnitude as f64;

	    ret.to_string()
	}
}