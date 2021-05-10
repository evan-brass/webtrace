#![allow(improper_ctypes_definitions, improper_ctypes)]
mod bitmap;
mod ffi;
mod render_job;
use lazy_static::lazy_static;

use self::{
	bitmap::{Bitmap, Color},
	ffi::{get_seed, log},
	render_job::RenderJob,
};

lazy_static! {
	static ref RENDER_JOB: RenderJob = RenderJob::new();
}

#[no_mangle]
pub extern "C" fn start_render() -> bool {
	RENDER_JOB.start_render(1)
}

#[no_mangle]
pub extern "C" fn status() -> bool {
	RENDER_JOB.status()
}

#[no_mangle]
pub extern "C" fn work() {
	log("Worker Started");
	RENDER_JOB.work();
}
