#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use p5_wasm::p5_wasm::P5Wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn to_string(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.to_string(), "p5.wasm.Vector Object : [1.2, 2.2, 3.2]".to_string());
}

#[wasm_bindgen_test]
fn set_vector(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector();
	let v2 = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.set_vector(&v2);

	assert_eq!(v.x, 1.2);
	assert_eq!(v.y, 2.2);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn set_1d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector();

	v.set_1d(1.2);

	assert_eq!(v.x, 1.2);
	assert_eq!(v.y, 0.0);
	assert_eq!(v.z, 0.0);
}

#[wasm_bindgen_test]
fn set_2d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector();

	v.set_2d(1.2, 2.2);

	assert_eq!(v.x, 1.2);
	assert_eq!(v.y, 2.2);
	assert_eq!(v.z, 0.0);
}

#[wasm_bindgen_test]
fn set_3d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector();

	v.set_3d(1.2, 2.2, 3.2);

	assert_eq!(v.x, 1.2);
	assert_eq!(v.y, 2.2);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn copy(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector();
	let v2 = v.copy();

	assert_eq!(v.x, v2.x);
	assert_eq!(v.y, v2.y);
	assert_eq!(v.z, v2.z);
}

#[wasm_bindgen_test]
fn add_vector(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	v.add_vector(&v2);

	assert_eq!(v.x, 5.4);
	assert_eq!(v.y, 7.4);
	assert_eq!(v.z, 9.4);
}

#[wasm_bindgen_test]
fn add_1d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.add_1d(4.2);

	assert_eq!(v.x, 5.4);
	assert_eq!(v.y, 2.2);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn add_2d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.add_2d(4.2, 5.2);

	assert_eq!(v.x, 5.4);
	assert_eq!(v.y, 7.4);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn add_3d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.add_3d(4.2, 5.2, 6.2);

	assert_eq!(v.x, 5.4);
	assert_eq!(v.y, 7.4);
	assert_eq!(v.z, 9.4);
}

#[wasm_bindgen_test]
fn sub_vector(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	v.sub_vector(&v2);

	assert_eq!(v.x, -3.0);
	assert_eq!(v.y, -3.0);
	assert_eq!(v.z, -3.0);
}

#[wasm_bindgen_test]
fn sub_1d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.sub_1d(4.2);

	assert_eq!(v.x, -3.0);
	assert_eq!(v.y, 2.2);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn sub_2d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.sub_2d(4.2, 5.2);

	assert_eq!(v.x, -3.0);
	assert_eq!(v.y, -3.0);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn sub_3d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.sub_3d(4.2, 5.2, 6.2);

	assert_eq!(v.x, -3.0);
	assert_eq!(v.y, -3.0);
	assert_eq!(v.z, -3.0);
}

#[wasm_bindgen_test]
fn mult(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.mult(2.0);

	assert_eq!(v.x, 2.4);
	assert_eq!(v.y, 4.4);
	assert_eq!(v.z, 6.4);
}

#[wasm_bindgen_test]
fn div(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.div(2.0);

	assert_eq!(v.x, 0.6);
	assert_eq!(v.y, 1.1);
	assert_eq!(v.z, 1.6);
}

#[wasm_bindgen_test]
fn mag(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.mag(), 4.064480286580316);
}

#[wasm_bindgen_test]
fn mag_sq(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.mag_sq(), 16.520000000000003);
}

#[wasm_bindgen_test]
fn dot_vector(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	assert_eq!(v.dot_vector(&v2), 36.32000000000001);
}

#[wasm_bindgen_test]
fn dot_1d(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.dot_1d(4.2), 5.04);
}

#[wasm_bindgen_test]
fn dot_2d(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.dot_2d(4.2, 5.2), 16.48);
}

#[wasm_bindgen_test]
fn dot_3d(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.dot_3d(4.2, 5.2, 6.2), 36.32000000000001);
}

#[wasm_bindgen_test]
fn cross(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	let r = v.cross(&v2);

	assert_eq!(r.x, -2.9999999999999982);
	assert_eq!(r.y, 6.000000000000002);
	assert_eq!(r.z, -3.0000000000000018);
}

#[wasm_bindgen_test]
fn dist(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	assert_eq!(v.dist(&v2), 5.196152422706632);
	assert_eq!(v2.x, 4.2);
	assert_eq!(v2.y, 5.2);
	assert_eq!(v2.z, 6.2);
}

#[wasm_bindgen_test]
fn normalize(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.normalize();

	assert_eq!(v.x, 0.2952406987830737);
	assert_eq!(v.y, 0.5412746144356352);
	assert_eq!(v.z, 0.7873085300881966);
}

#[wasm_bindgen_test]
fn limit(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.limit(0.5);

	assert_eq!(v.x, 0.14762034939153684);
	assert_eq!(v.y, 0.2706373072178176);
	assert_eq!(v.z, 0.3936542650440983);
}

#[wasm_bindgen_test]
fn set_mag(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.set_mag(2.0);

	assert_eq!(v.x, 0.5904813975661474);
	assert_eq!(v.y, 1.0825492288712704);
	assert_eq!(v.z, 1.5746170601763931);
}

#[wasm_bindgen_test]
fn heading(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.heading(), 1.0714496051147666);
}

#[wasm_bindgen_test]
fn rotate(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.rotate(2.0);

	assert_eq!(v.x, -4.05448566789455);
	assert_eq!(v.y, 0.2848613150950695);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn angle_between(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	assert_eq!(v.angle_between(&v2), -0.19963080123649965);
}

#[wasm_bindgen_test]
fn lerp_vector(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	v.lerp_vector(&v2, 0.5);

	assert_eq!(v.x, 2.7);
	assert_eq!(v.y, 3.7);
	assert_eq!(v.z, 4.7);
}

#[wasm_bindgen_test]
fn lerp_1d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.lerp_1d(4.2, 0.5);

	assert_eq!(v.x, 2.7);
	assert_eq!(v.y, 2.2);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn lerp_2d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.lerp_2d(4.2, 5.2, 0.5);

	assert_eq!(v.x, 2.7);
	assert_eq!(v.y, 3.7);
	assert_eq!(v.z, 3.2);
}

#[wasm_bindgen_test]
fn lerp_3d(){
	let p5 = P5Wasm::new();
	let mut v = p5.create_vector_3d(1.2, 2.2, 3.2);

	v.lerp_3d(4.2, 5.2, 6.2, 0.5);

	assert_eq!(v.x, 2.7);
	assert_eq!(v.y, 3.7);
	assert_eq!(v.z, 4.7);
}

#[wasm_bindgen_test]
fn array(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.array(), vec![1.2, 2.2, 3.2]);
}

#[wasm_bindgen_test]
fn equals_vector(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);
	let v2 = p5.create_vector_3d(4.2, 5.2, 6.2);

	assert_eq!(v.equals_vector(&v2), false);
	assert_eq!(v.equals_vector(&p5.create_vector_3d(1.2, 2.2, 3.2)), true);
}

#[wasm_bindgen_test]
fn equals_1d(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.equals_1d(4.2), false);
	assert_eq!(v.equals_1d(1.2), true);
}

#[wasm_bindgen_test]
fn equals_2d(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.equals_2d(4.2, 5.2), false);
	assert_eq!(v.equals_2d(1.2, 2.2), true);
}

#[wasm_bindgen_test]
fn equals_3d(){
	let p5 = P5Wasm::new();
	let v = p5.create_vector_3d(1.2, 2.2, 3.2);

	assert_eq!(v.equals_3d(4.2, 5.2, 6.2), false);
	assert_eq!(v.equals_3d(1.2, 2.2, 3.2), true);
}