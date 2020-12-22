use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;
use std::collections::HashMap;

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
	pub(crate) _lcg: lcg::LCG,
	pub(crate) color_mode: String,
	pub(crate) color_maxes: HashMap< String, Vec<f64> >,
}

#[wasm_bindgen]
impl P5Wasm {
	pub fn new() -> P5Wasm {
		set_panic_hook();

		let mut p = Vec::<f64>::new();
		for _ in 0..PERLIN_SIZE+1 {
			p.push(random());
		}

		let mut maxes = HashMap::new();
		maxes.insert("rgb".to_string(), vec!(255.0, 255.0, 255.0, 255.0));
		maxes.insert("hsb".to_string(), vec!(360.0, 100.0, 100.0, 1.0));
		maxes.insert("hsl".to_string(), vec!(360.0, 100.0, 100.0, 1.0));

		P5Wasm {
			perlin: p,
			perlin_octaves: 4,
			perlin_amp_falloff: 0.5,
			perlin_lcg: lcg::LCG::new(),
			_lcg: lcg::LCG::new(),
			color_mode: String::from("rgb"),
			color_maxes: maxes,
		}
	}
}

mod lcg {
	#![allow(non_upper_case_globals)]
	const m:f64 = 4294967296.0;
	const a:f64 = 1664525.0;
	const c:f64 = 1013904223.0;

	pub struct LCG {
		z: f64,
	}

	impl LCG {
		pub(super) fn new() -> LCG {
			LCG {
				z: super::random(),
			}
		}

		pub(crate) fn set_seed(&mut self, val: f64){
			self.z = (val as i32 as u32 >> 0) as f64;
		}

		pub(crate) fn rand(&mut self) -> f64{
			self.z = (a * self.z + c) % m;

			self.z / m
		}
	}
}