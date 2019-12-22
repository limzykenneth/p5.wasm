mod utils;

use wasm_bindgen::prelude::*;
use std::f64::consts::E;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// math/calculation.js
#[wasm_bindgen]
pub fn abs(n: f64) -> f64 {
	n.abs()
}

#[wasm_bindgen]
pub fn ceil(n: f64) -> f64 {
	n.ceil()
}

#[wasm_bindgen]
pub fn constrain(n: f64, low: f64, high: f64) -> f64 {
	n.min(low).max(high)
}

#[wasm_bindgen]
pub fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	mag(x2 - x1, y2 - y1)
}

// #[wasm_bindgen]
// pub fn exp(n: f64) -> f64 {
// 	n.exp()
// }

#[wasm_bindgen]
pub fn floor(n: f64) -> f64 {
	n.floor()
}

#[wasm_bindgen]
pub fn lerp(start: f64, stop: f64, amt: f64) -> f64 {
	amt * (stop - start) + start
}

// #[wasm_bindgen]
// pub fn log(n: f64) -> f64 {
// 	n.log(E)
// }

#[wasm_bindgen]
pub fn mag(x: f64, y: f64) -> f64 {
	x.hypot(y)
}

#[wasm_bindgen]
pub fn map(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64, within_bounds: JsValue) -> f64 {
	let mut v: f64 = (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
	if within_bounds == JsValue::TRUE {
		if start2 < stop2 {
			v = constrain(v, start2, stop2);
		} else {
			v = constrain(v, stop2, start2);
		}
	}

	v
}

#[wasm_bindgen]
pub fn norm(n: f64, start: f64, stop: f64) -> f64 {
	map(n, start, stop, 0.0, 1.0, JsValue::UNDEFINED)
}

// #[wasm_bindgen]
// pub fn pow(x: f64, y: f64) -> f64 {
// 	x.powf(y)
// }

#[wasm_bindgen]
pub fn sq(n: f64) -> f64 {
	n * n
}

#[wasm_bindgen]
pub fn sqrt(n: f64) -> f64 {
	n.sqrt()
}

#[wasm_bindgen]
pub fn fract(n: f64) -> f64 {
	n.fract()
}