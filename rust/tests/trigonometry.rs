#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn acos(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.acos(0.5), 1.0471975511965979);
}

#[wasm_bindgen_test]
fn asin(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.asin(0.5), 0.5235987755982989);
}

#[wasm_bindgen_test]
fn atan(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.atan(0.5), 0.4636476090008061);
}

#[wasm_bindgen_test]
fn atan2(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.atan2(0.2, 1.0), 0.19739555984988078);
}

#[wasm_bindgen_test]
fn cos(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.cos(0.5), 0.8775825618903728);
}

#[wasm_bindgen_test]
fn sin(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.sin(0.5), 0.479425538604203);
}

#[wasm_bindgen_test]
fn tan(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.tan(0.5), 0.5463024898437905);
}

#[wasm_bindgen_test]
fn degrees(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.degrees(0.5), 28.64788975654116);
}

#[wasm_bindgen_test]
fn radians(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.radians(30.0), 0.5235987755982988);
}