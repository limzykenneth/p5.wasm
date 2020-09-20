use wasm_bindgen::prelude::*;
use super::p5_wasm::P5Wasm;

// math/trigonometry.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn acos(&self, value: f64) -> f64 {
		value.acos()
	}

	pub fn asin(&self, value: f64) -> f64 {
		value.asin()
	}

	pub fn atan(&self, value: f64) -> f64 {
		value.atan()
	}

	pub fn atan2(&self, y: f64, x: f64) -> f64 {
		y.atan2(x)
	}

	pub fn cos(&self, angle: f64) -> f64 {
		angle.cos()
	}

	pub fn sin(&self, angle: f64) -> f64 {
		angle.sin()
	}

	pub fn tan(&self, angle: f64) -> f64 {
		angle.tan()
	}

	pub fn degrees(&self, radians: f64) -> f64 {
		radians.to_degrees()
	}

	pub fn radians(&self, degrees: f64) -> f64 {
		degrees.to_radians()
	}
}