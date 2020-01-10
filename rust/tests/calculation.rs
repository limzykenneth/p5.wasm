#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::JsValue;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_abs(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.abs(1.2), 1.2);
	assert_eq!(p5.abs(-1.2), 1.2);
}

#[wasm_bindgen_test]
fn test_ceil(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.ceil(1.2), 2.0);
	assert_eq!(p5.ceil(-1.2), -1.0);
}

#[wasm_bindgen_test]
fn test_constrain(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.constrain(1.2, 0.0, 1.0), 1.0);
	assert_eq!(p5.constrain(0.5, 0.0, 1.0), 0.5);
}

#[wasm_bindgen_test]
fn test_dist(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.dist(2.0, 4.0, 6.0, 8.0), 5.656854249492381);
}

#[wasm_bindgen_test]
fn test_dist3d(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.dist3d(2.0, 4.0, 6.0, 8.0, 10.0, 12.0), 10.392304845413264);
}

#[wasm_bindgen_test]
fn test_exp(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.exp(1.2), 3.3201169227365472);
}

#[wasm_bindgen_test]
fn test_floor(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.floor(1.2), 1.0);
	assert_eq!(p5.floor(-1.2), -2.0);
}

#[wasm_bindgen_test]
fn test_lerp(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.lerp(0.5, 0.0, 100.0), -49.5);
}

#[wasm_bindgen_test]
fn test_log(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.log(1.2), 0.1823215567939546);
}

#[wasm_bindgen_test]
fn test_mag(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.mag(3.0, 4.0), 5.0);
}

#[wasm_bindgen_test]
fn test_map(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.map(0.2, 0.0, 1.0, 0.0, 100.0, JsValue::FALSE), 20.0);
}

#[wasm_bindgen_test]
fn test_map_constrain(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.map(1.2, 0.0, 1.0, 0.0, 100.0, JsValue::FALSE), 120.0);
	assert_eq!(p5.map(1.2, 0.0, 1.0, 0.0, 100.0, JsValue::TRUE), 100.0);
}

#[wasm_bindgen_test]
fn test_norm(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.norm(50.0, 0.0, 100.0), 0.5);
}

#[wasm_bindgen_test]
fn test_pow(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.pow(2.0, 3.0), 8.0);
}

#[wasm_bindgen_test]
fn test_round(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.round(1.2), 1.0);
	assert_eq!(p5.round(-1.2), -1.0);
}

#[wasm_bindgen_test]
fn test_round_decimal(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.round_decimal(1.23456789, 2), 1.23);
	assert_eq!(p5.round_decimal(-1.23456789, 2), -1.23);
}

#[wasm_bindgen_test]
fn test_sq(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.sq(1.2), 1.44);
}

#[wasm_bindgen_test]
fn test_sqrt(){
	let p5 = P5Wasm::new();

	assert_eq!(p5.sqrt(1.2), 1.0954451150103321);
}

// #[wasm_bindgen_test]
// fn test_fract(){
// 	let p5 = P5Wasm::new();

// 	assert_eq!(p5.fract(1.2), 0.2);
// }