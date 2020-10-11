#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn to_string(){
	let p5 = P5Wasm::new();
	let mut c = p5.color(0.0, 255.0, 255.0, 255.0);

	assert_eq!(c.to_string(JsValue::from_str("#rrggbb")), "#00ffff");
	assert_eq!(c.to_string(JsValue::from_str("#rrggbbaa")), "#00ffffff");
	assert_eq!(c.to_string(JsValue::from_str("#rgb")), "#0ff");
	assert_eq!(c.to_string(JsValue::from_str("#rgba")), "#0fff");
	assert_eq!(c.to_string(JsValue::from_str("rgb")), "rgb(0, 255, 255)");
	assert_eq!(c.to_string(JsValue::from_str("rgb%")), "rgb(0%, 100%, 100%)");
	assert_eq!(c.to_string(JsValue::from_str("rgba%")), "rgba(0%, 100%, 100%, 100%)");
	assert_eq!(c.to_string(JsValue::from_str("hsb")), "hsb(180, 100, 100)");
	assert_eq!(c.to_string(JsValue::from_str("hsb%")), "hsb(50%, 100%, 100%)");
	assert_eq!(c.to_string(JsValue::from_str("hsba")), "hsba(180, 100, 100, 1)");
	assert_eq!(c.to_string(JsValue::from_str("hsba%")), "hsba(50%, 100%, 100%, 100%)");
	assert_eq!(c.to_string(JsValue::from_str("hsl")), "hsl(180, 100, 50)");
	assert_eq!(c.to_string(JsValue::from_str("hsl%")), "hsl(50%, 100%, 50%)");
	assert_eq!(c.to_string(JsValue::from_str("hsla")), "hsla(180, 100, 50, 1)");
	assert_eq!(c.to_string(JsValue::from_str("hsla%")), "hsla(50%, 100%, 50%, 100%)");
	assert_eq!(c.to_string(JsValue::from_str("rgba")), "rgba(0, 255, 255, 1)");
	assert_eq!(c.to_string(JsValue::UNDEFINED), "rgba(0, 255, 255, 1)");
}

#[wasm_bindgen_test]
fn red(){
	let p5 = P5Wasm::new();
	let c = p5.color(0.0, 200.0, 255.0, 50.0);

	assert_eq!(c.red(), 0.0);
}

#[wasm_bindgen_test]
fn green(){
	let p5 = P5Wasm::new();
	let c = p5.color(0.0, 200.0, 255.0, 50.0);

	assert_eq!(c.green(), 200.0);
}

#[wasm_bindgen_test]
fn blue(){
	let p5 = P5Wasm::new();
	let c = p5.color(0.0, 200.0, 255.0, 50.0);

	assert_eq!(c.blue(), 255.0);
}

#[wasm_bindgen_test]
fn alpha(){
	let p5 = P5Wasm::new();
	let c = p5.color(0.0, 200.0, 255.0, 50.0);

	assert_eq!(c.alpha(), 50.0);
}

#[wasm_bindgen_test]
fn set_red(){
	let p5 = P5Wasm::new();
	let mut c = p5.color(0.0, 200.0, 255.0, 50.0);
	c.set_red(20.0);

	assert_eq!(c.red(), 20.0);
}

#[wasm_bindgen_test]
fn set_green(){
	let p5 = P5Wasm::new();
	let mut c = p5.color(0.0, 200.0, 255.0, 50.0);
	c.set_green(30.0);

	assert_eq!(c.green(), 30.0);
}

#[wasm_bindgen_test]
fn set_blue(){
	let p5 = P5Wasm::new();
	let mut c = p5.color(0.0, 200.0, 255.0, 50.0);
	c.set_blue(40.0);

	assert_eq!(c.blue(), 40.0);
}

#[wasm_bindgen_test]
fn set_alpha(){
	let p5 = P5Wasm::new();
	let mut c = p5.color(0.0, 200.0, 255.0, 50.0);
	c.set_alpha(0.0);

	assert_eq!(c.alpha(), 0.0);
}