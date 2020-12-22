#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn random(){
	let mut p5 = P5Wasm::new();

	p5.random_seed(2.22);
	assert_eq!(p5.random(), 0.23684307769872248);
}

#[wasm_bindgen_test]
fn random_range(){
	let mut p5 = P5Wasm::new();

	p5.random_seed(2.22);
	assert_eq!(p5.random_range(1.2, 2.2), 1.4368430776987224);
}