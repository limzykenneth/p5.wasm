use wasm_bindgen::prelude::*;
use super::p5_wasm::P5Wasm;
use std::f64::INFINITY;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
}

macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Vector {
	x: f64,
	y: f64,
	z: f64,
}

// Unlike JS counterparts, these never return the vector instance
#[wasm_bindgen]
impl Vector {
	pub fn new(x: f64, y: f64, z: f64) -> Vector{
		Vector {
			x: x,
			y: y,
			z: z,
		}
	}

	pub fn to_string(&self) -> String{
		format!("p5.wasm.Vector Object : [{}, {}, {}]", self.x, self.y, self.z).to_string()
	}

	pub fn set_vector(&mut self, vector: Vector){
		self.x = vector.x;
		self.y = vector.y;
		self.z = vector.z;
	}

	pub fn set_1d(&mut self, x: f64){
		self.x = x;
	}

	pub fn set_2d(&mut self, x: f64, y: f64){
		self.x = x;
		self.y = y;
	}

	pub fn set_3d(&mut self, x: f64, y: f64, z: f64){
		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn copy(&self) -> Vector{
		Vector{
			x: self.x,
			y: self.y,
			z: self.z,
		}
	}

	pub fn add_vector(&mut self, vector: Vector){
		self.x += vector.x;
		self.y += vector.y;
		self.z += vector.z;
	}

	pub fn add_1d(&mut self, x: f64){
		self.x += x;
	}

	pub fn add_2d(&mut self, x: f64, y: f64){
		self.x += x;
		self.y += y;
	}

	pub fn add_3d(&mut self, x: f64, y: f64, z: f64){
		self.x += x;
		self.y += y;
		self.z += z;
	}

	pub fn sub_vector(&mut self, vector: Vector){
		self.x -= vector.x;
		self.y -= vector.y;
		self.z -= vector.z;
	}

	pub fn sub_1d(&mut self, x: f64){
		self.x -= x;
	}

	pub fn sub_2d(&mut self, x: f64, y: f64){
		self.x -= x;
		self.y -= y;
	}

	pub fn sub_3d(&mut self, x: f64, y: f64, z: f64){
		self.x -= x;
		self.y -= y;
		self.z -= z;
	}

	pub fn mult(&mut self, n: f64){
		if n == INFINITY {
			console_warn!("p5.wasm.Vector.prototype.mult: n is not a finite number");
		} else {
			self.x *= n;
			self.y *= n;
			self.z *= n;
		}
	}

	pub fn div(&mut self, n: f64){
		if n == INFINITY {
			console_warn!("p5.wasm.Vector.prototype.mult: n is not a finite number");
		} else if n == 0.0 {
			console_warn!("p5.wasm.Vector.prototype.mult: divide by 0");
		} else {
			self.x /= n;
			self.y /= n;
			self.z /= n;
		}
	}

	pub fn mag(&self) -> f64{
		self.mag_sq().sqrt()
	}

	pub fn mag_sq(&self) -> f64{
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn dot_vector(&self, vector: Vector) -> f64{
		self.dot_3d(vector.x, vector.y, vector.z)
	}

	pub fn dot_1d(&self, x: f64) -> f64{
		self.dot_3d(x, 0.0, 0.0)
	}

	pub fn dot_2d(&self, x: f64, y: f64) -> f64{
		self.dot_3d(x, y, 0.0)
	}

	pub fn dot_3d(&self, x: f64, y: f64, z: f64) -> f64{
		self.x * x + self.y * y + self.z * z
	}

	// Non-static version only
	pub fn cross(&self, v: Vector) -> Vector{
		let x = self.y * v.z - self.z * v.y;
		let y = self.z * v.x - self.x * v.z;
		let z = self.x * v.y - self.y * v.x;

		Vector{
			x: x,
			y: y,
			z: z,
		}
	}
}

#[wasm_bindgen]
impl P5Wasm {
	pub fn create_vector(&self) -> Vector{
		Vector::new(0.0, 0.0, 0.0)
	}
}