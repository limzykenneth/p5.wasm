use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;

// math/random.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn random(&mut self) -> f64 {
		self._lcg.rand()
	}

	pub fn random_range(&mut self, min: f64, max: f64) -> f64 {
		let mut minimum = min;
		let mut maximum = max;

		if minimum > maximum {
			let tmp = minimum;
			minimum = maximum;
			maximum = tmp;
		}
		self._lcg.rand() * (maximum - minimum) + minimum
	}

	pub fn random_seed(&mut self, seed: f64) {
		self._lcg.set_seed(seed);
	}
}