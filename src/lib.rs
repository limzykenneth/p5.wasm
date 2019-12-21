mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// math/calculation.js
#[wasm_bindgen]
pub fn map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32, within_bounds: JsValue) -> f32 {
	let mut v: f32 = (n - start1) / (stop1 - start1) * (stop2 - start2) + start2;
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
pub fn lerp(start: f32, stop: f32, amt: f32) -> f32 {
	amt * (stop - start) + start
}

#[wasm_bindgen]
pub fn constrain(n: f32, low: f32, high: f32) -> f32 {
	n.min(low).max(high)
}

#[wasm_bindgen]
pub fn mag(x: f32, y: f32) -> f32 {
	x.hypot(y)
}

#[wasm_bindgen]
pub fn norm(n: f32, start: f32, stop: f32) -> f32 {
	map(n, start, stop, 0.0, 1.0, JsValue::UNDEFINED)
}
