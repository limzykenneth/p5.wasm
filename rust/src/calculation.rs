use wasm_bindgen::prelude::*;
use super::p5_wasm::P5Wasm;
use std::f64::consts::E;

// math/calculation.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn abs(&self, n: f64) -> f64{
		n.abs()
	}

	pub fn ceil(&self, n: f64) -> f64 {
		n.ceil()
	}

	pub fn constrain(&self, n: f64, low: f64, high: f64) -> f64 {
		n.min(low).max(high)
	}

	pub fn dist(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
		((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
	}

	pub fn dist3d(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
		((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
	}

	pub fn exp(&self, n: f64) -> f64 {
		n.exp()
	}

	pub fn floor(&self, n: f64) -> f64 {
		n.floor()
	}

	pub fn lerp(&self, start: f64, stop: f64, amt: f64) -> f64 {
		amt * (stop - start) + start
	}

	pub fn log(&self, n: f64) -> f64 {
		n.log(E)
	}

	pub fn mag(&self, x: f64, y: f64) -> f64 {
		(x.powi(2) + y.powi(2)).sqrt()
	}

	pub fn map(&self, n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64, within_bounds: JsValue) -> f64 {
		let mut v: f64 = (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
		if within_bounds == JsValue::TRUE {
			if start2 < stop2 {
				v = self.constrain(v, start2, stop2);
			} else {
				v = self.constrain(v, stop2, start2);
			}
		}

		v
	}

	pub fn norm(&self, n: f64, start: f64, stop: f64) -> f64 {
		self.map(n, start, stop, 0.0, 1.0, JsValue::UNDEFINED)
	}

	pub fn pow(&self, x: f64, y: f64) -> f64 {
		x.powf(y)
	}

	pub fn sq(&self, n: f64) -> f64 {
		n * n
	}

	pub fn sqrt(&self, n: f64) -> f64 {
		n.sqrt()
	}

	pub fn fract(&self, n: f64) -> f64 {
		n.fract()
	}
}