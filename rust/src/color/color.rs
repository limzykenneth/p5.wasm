use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;
use std::collections::HashMap;
use super::color_conversion;

// color/p5.Color.js
#[wasm_bindgen]
pub struct Color {
	array: Vec<f64>,
	mode: String,
	maxes: HashMap< String, Vec<f64> >,
	hsba: Option< Vec<f64> >,
}

#[wasm_bindgen]
impl Color {
	pub fn new(vals: Vec<f64>) -> Color {
		let mut maxes = HashMap::new();
		maxes.insert("rgb".to_string(), vec!(255.0, 255.0, 255.0, 255.0));
		maxes.insert("hsb".to_string(), vec!(360.0, 100.0, 100.0, 1.0));
		maxes.insert("hsl".to_string(), vec!(360.0, 100.0, 100.0, 1.0));

		Color {
			array: vals.iter().map(|v| v / 255.0).collect(),
			mode: String::from("RGB"),
			maxes: maxes,
			hsba: None
		}
	}

	pub fn to_string(&mut self, format: &str) -> String {
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
				// "".to_string()
				// self.maxes["hsb"][0].to_string()
				let maxes = self.maxes.get("hsb").unwrap();
				format!("hsb({}, {}, {})", hsba[0] * maxes[0], hsba[1] * maxes[1], hsba[2] * maxes[2])
			}
			"rgba" | _ => {
				format!("rgba({}, {}, {}, {})", arr[0] * 255.0, arr[1] * 255.0, arr[2] * 255.0, arr[3])
			}
		}
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