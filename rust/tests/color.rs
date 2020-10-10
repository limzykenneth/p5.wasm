#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn to_string(){
	let p5 = P5Wasm::new();
	let mut c = p5.color(255.0, 255.0, 255.0, 255.0);

	assert_eq!(c.to_string("#rrggbb"), "#ffffff");
	assert_eq!(c.to_string("#rrggbbaa"), "#ffffffff");
	assert_eq!(c.to_string("#rgb"), "#fff");
	assert_eq!(c.to_string("#rgba"), "#ffff");
	assert_eq!(c.to_string("rgb"), "rgb(255, 255, 255)");
	assert_eq!(c.to_string("rgb%"), "rgb(100%, 100%, 100%)");
	assert_eq!(c.to_string("rgba%"), "rgba(100%, 100%, 100%, 100%)");
	assert_eq!(c.to_string("hsb"), "hsb(0, 0, 100)");
	assert_eq!(c.to_string("rgba"), "rgba(255, 255, 255, 1)");
}