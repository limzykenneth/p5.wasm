#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn color_mode(){
	let mut p5 = P5Wasm::new();
	p5.color_mode("rgb".to_string(), JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED);
	p5.color_mode("hsb".to_string(), JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED);
	p5.color_mode("hsl".to_string(), JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED);

	p5.color_mode("rgb".to_string(), JsValue::from_f64(255.0), JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED);
	p5.color_mode("rgb".to_string(), JsValue::from_f64(255.0), JsValue::from_f64(100.0), JsValue::from_f64(0.0), JsValue::UNDEFINED);
	p5.color_mode("rgb".to_string(), JsValue::from_f64(255.0), JsValue::from_f64(100.0), JsValue::from_f64(0.0), JsValue::from_f64(50.0));
}

#[wasm_bindgen_test]
fn red(){
	let p5 = P5Wasm::new();
	let color = p5.color(255.0, 200.0, 100.0, 50.0);

	assert_eq!(p5.red(color), 255.0);
}

#[wasm_bindgen_test]
fn green(){
	let p5 = P5Wasm::new();
	let color = p5.color(255.0, 200.0, 100.0, 50.0);

	assert_eq!(p5.green(color), 200.0);
}

#[wasm_bindgen_test]
fn blue(){
	let p5 = P5Wasm::new();
	let color = p5.color(255.0, 200.0, 100.0, 50.0);

	assert_eq!(p5.blue(color), 100.0);
}

#[wasm_bindgen_test]
fn alpha(){
	let p5 = P5Wasm::new();
	let color = p5.color(255.0, 200.0, 100.0, 50.0);

	assert_eq!(p5.alpha(color), 50.0);
}