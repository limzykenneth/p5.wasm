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
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.red(color), 255.0);
}

#[wasm_bindgen_test]
fn green(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.green(color), 200.0);
}

#[wasm_bindgen_test]
fn blue(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.blue(color), 100.0);
}

#[wasm_bindgen_test]
fn alpha(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.alpha(color), 50.0);
}

#[wasm_bindgen_test]
fn hue(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.hue(color), 38.70967741935483);
}

#[wasm_bindgen_test]
fn saturation(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.saturation(color), 100.0);
}

#[wasm_bindgen_test]
fn brightness(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.brightness(color), 100.0);
}

#[wasm_bindgen_test]
fn lightness(){
	let p5 = P5Wasm::new();
	let color = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));

	assert_eq!(p5.lightness(color), 69.6078431372549);
}

#[wasm_bindgen_test]
fn lerp_color(){
	let p5 = P5Wasm::new();
	let c1 = p5.color(JsValue::from_f64(255.0), JsValue::from_f64(200.0), JsValue::from_f64(100.0), JsValue::from_f64(50.0));
	let c2 = p5.color(JsValue::from_f64(90.0), JsValue::from_f64(80.0), JsValue::from_f64(70.0), JsValue::from_f64(60.0));

	assert_eq!(p5.lerp_color(c1, c2, 0.75).to_string(JsValue::UNDEFINED), "rgba(131, 110, 78, 0.22549019607843138)")
}