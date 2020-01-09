use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

const PERLIN_SIZE:usize = 4095;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = Math)]
	fn random() -> f64;
}

#[wasm_bindgen]
pub struct P5Wasm {
	pub(crate) perlin: Vec<f64>,
	pub(crate) perlin_octaves: usize,
	pub(crate) perlin_amp_falloff: f64,
}

#[wasm_bindgen]
impl P5Wasm {
	pub fn new() -> P5Wasm {
		set_panic_hook();

		let mut p = Vec::<f64>::new();
		for _ in 0..PERLIN_SIZE {
			p.push(random());
		}

		P5Wasm {
			perlin: p,
			perlin_octaves: 4,
			perlin_amp_falloff: 0.5
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_noise(){
		let p5 = P5Wasm::new();
		assert!(p5.noise(1.2) > 0.0);
		assert!(p5.noise(1.2) < 1.0);
	}
}