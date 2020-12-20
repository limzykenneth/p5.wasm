use wasm_bindgen::prelude::*;
use crate::p5_wasm::P5Wasm;

const PERLIN_YWRAPB:usize = 4;
const PERLIN_YWRAP:usize = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB:usize = 8;
const PERLIN_ZWRAP:usize = 1 << PERLIN_ZWRAPB;
const PERLIN_SIZE:usize = 4095;

// math/noise.js
#[wasm_bindgen]
impl P5Wasm {
	pub fn noise(&self, x:f64) -> f64 {
		self.perlin_noise(x, 0.0, 0.0)
	}

	pub fn noise2d(&self, x:f64, y:f64) -> f64 {
		self.perlin_noise(x, y, 0.0)
	}

	pub fn noise3d(&self, x:f64, y:f64, z:f64) -> f64 {
		self.perlin_noise(x, y, z)
	}

	fn perlin_noise(&self, x:f64, y:f64, z:f64) -> f64 {
		let x = x.abs();
		let y = y.abs();
		let z = z.abs();

		let mut xi = x.floor() as usize;
		let mut yi = y.floor() as usize;
		let mut zi = z.floor() as usize;

		let mut xf = x - xi as f64;
		let mut yf = y - yi as f64;
		let mut zf = z - zi as f64;

		let mut r = 0.0;
		let mut ampl = 0.5;

		for _ in 0..self.perlin_octaves {
			let mut of = xi + (yi << PERLIN_YWRAPB) + (zi << PERLIN_ZWRAPB);

			let rxf = scaled_cosine(xf);
			let ryf = scaled_cosine(yf);

			let mut n1:f64 = self.perlin[of & PERLIN_SIZE];
			n1 += rxf * (self.perlin[(of + 1) & PERLIN_SIZE] - n1);
			let mut n2 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_SIZE];
			n2 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_SIZE] - n2);
			n1 += ryf * (n2 - n1);

			of += PERLIN_ZWRAP;
			n2 = self.perlin[of & PERLIN_SIZE];
		    n2 += rxf * (self.perlin[(of + 1) & PERLIN_SIZE] - n2);
		    let mut n3 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_SIZE];
		    n3 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_SIZE] - n3);
		    n2 += ryf * (n3 - n2);

			n1 += scaled_cosine(zf) * (n2 - n1);

		    r += n1 * ampl;
			ampl *= self.perlin_amp_falloff;
			xi <<= 1;
			xf *= 2.0;
			yi <<= 1;
			yf *= 2.0;
			zi <<= 1;
			zf *= 2.0;

			if xf >= 1.0 {
				xi += 1;
				xf -= 1.0;
			}
			if yf >= 1.0 {
				yi += 1;
				yf -= 1.0;
			}
			if zf >= 1.0 {
				zi += 1;
				zf -= 1.0;
			}
		}

		r
	}

	pub fn noise_detail(&mut self, lod: usize, falloff: f64){
		if lod > 0 {
			self.perlin_octaves = lod;
		}
		if falloff > 0.0 {
			self.perlin_amp_falloff = falloff;
		}
	}

	pub fn noise_seed(&mut self, seed: f64){
		self.perlin_lcg.set_seed(seed);
		self.perlin = Vec::<f64>::new();
		for _ in 0..PERLIN_SIZE+1 {
			self.perlin.push(self.perlin_lcg.rand());
		}
	}
}

fn scaled_cosine(i:f64) -> f64 {
	0.5 * (1.0 - (i * std::f64::consts::PI).cos())
}