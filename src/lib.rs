use std::sync::{Condvar, Mutex};

mod ffi;

mod bitmap;
mod ray;
mod render_job;
mod scene;
use self::{render_job::RenderJob, scene::Scene};

static S: Scene = Scene {};

static CURRENT_JOB: Mutex<Option<RenderJob>> = Mutex::new(None);
static JOB_CHANGE: Condvar = Condvar::new();

#[no_mangle]
pub extern "C" fn work() {
	ffi::log("Starting to work...");
	while let Ok(lock) = CURRENT_JOB.lock() {
		if let Some(ref job) = *lock {
			job.work()
		} else {
			JOB_CHANGE.wait(lock);
		}
	}
	ffi::log("Done working.");
}

#[no_mangle]
pub extern "C" fn start_render() {}
