//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_noise(){
	let mut p5 = P5Wasm::new();

	p5.noise_seed(5.0);
	assert_eq!(p5.noise(1.2), 0.5268739465644248);

	p5.noise_seed(-5.0);
	assert_eq!(p5.noise(1.2), 0.6269776498370303);

	p5.noise_seed(2.22);
	assert_eq!(p5.noise(1.2), 0.3777067486458202);

	p5.noise_seed(-2.22);
	assert_eq!(p5.noise(1.2), 0.2517338180876808);
}
