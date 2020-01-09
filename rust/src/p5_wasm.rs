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
	pub(crate) perlin_lcg: lcg::LCG,
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
			perlin_amp_falloff: 0.5,
			perlin_lcg: lcg::LCG::new()
		}
	}
}

mod lcg {
	#![allow(non_upper_case_globals)]
	const m:f64 = 4294967296.0;
	const a:f64 = 1664525.0;
	const c:f64 = 1013904223.0;

	pub struct LCG {
		seed: f64,
		z: f64,
	}

	impl LCG {
		pub(super) fn new() -> LCG {
			LCG {
				seed: super::random(),
				z: super::random(),
			}
		}

		pub(crate) fn set_seed(&mut self, val: f64){
			self.seed = (val as u32) as f64;
			self.z = self.seed;
		}

		pub(crate) fn rand(&mut self) -> f64{
			self.z = (a * self.z + c) % m;

			self.z / m
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