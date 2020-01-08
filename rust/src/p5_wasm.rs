use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub struct P5Wasm;

#[wasm_bindgen]
impl P5Wasm {
	pub fn new() -> P5Wasm {
		set_panic_hook();

		P5Wasm
	}
}