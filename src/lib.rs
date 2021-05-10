#![allow(improper_ctypes_definitions, improper_ctypes)]
mod bitmap;
mod ffi;
use self::{
	bitmap::{Bitmap, Color},
	ffi::{get_seed, log},
};

#[no_mangle]
pub extern "C" fn render() {
	log("Starting to render...");

	let mut output: Bitmap = Bitmap::new(256, 256);

	for (i, color) in output.data.iter_mut().enumerate() {
		let x = i % output.width;
		let y = i / output.width;

		*color = Color {
			r: x as u8,
			g: y as u8,
			b: 255 / 4,
			a: 255,
		};
	}
	output.view();
	log("Done rendering.");
}

#[no_mangle]
pub extern "C" fn work() {}
