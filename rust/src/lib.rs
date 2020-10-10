mod utils;
pub mod p5_wasm;

mod math {
	mod calculation;
	mod noise;
	mod vector;
	mod trigonometry;
}

mod color {
	mod color;
	mod color_conversion;
	mod creating_reading;
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;