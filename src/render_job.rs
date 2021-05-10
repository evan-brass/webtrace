use std::{
	slice::from_raw_parts_mut,
	sync::{Condvar, Mutex},
};

use super::bitmap::{Bitmap, Color};

struct Progress {
	passes: usize,
	current_line: usize,
	lines_done: usize,
}

pub struct RenderJob {
	progress: Mutex<Progress>,
	worker_notify: Condvar,
	output: Bitmap,
}
impl RenderJob {
	pub fn new() -> Self {
		Self {
			progress: Mutex::new(Progress {
				passes: 0,
				current_line: 0,
				lines_done: 0,
			}),
			worker_notify: Condvar::new(),
			output: Bitmap::new(256, 256),
		}
	}
	pub fn start_render(&self, passes: usize) -> bool {
		if let Ok(mut guard) = self.progress.try_lock() {
			if guard.passes == 0 {
				guard.passes = passes;
				self.worker_notify.notify_all();
				return true;
			}
		}
		// Failed to start render:
		false
	}
	pub fn is_done(&self) -> bool {
		if let Ok(guard) = self.progress.try_lock() {
			guard.passes == 0
		} else {
			false
		}
	}
	pub fn status(&self) -> bool {
		self.output.view();
		!self.is_done()
	}
	pub fn work(&self) {
		let mut progress = self.progress.lock().unwrap();
		loop {
			if progress.passes > 0 && progress.current_line < self.output.height() {
				// Take a line and work on it
				let y = progress.current_line;
				progress.current_line += 1;
				drop(progress);

				// Render the line
				let width = self.output.width;
				let offset = y * width;
				let line = unsafe {
					from_raw_parts_mut(
						self.output.data.as_ptr().offset(offset as isize) as *mut Color,
						width,
					)
				};
				for (i, color) in line.iter_mut().enumerate() {
					let x = i % width;

					// TODO: Sample instead
					*color = Color {
						r: x as u8,
						g: y as u8,
						b: 255 / 4,
						a: 255,
					};
				}

				// Update the progress
				progress = self.progress.lock().unwrap();
				progress.lines_done += 1;
				if progress.lines_done == self.output.height() {
					progress.passes -= 1;
					progress.lines_done = 0;
					progress.current_line = 0;
					self.worker_notify.notify_all();
				}
			} else {
				progress = self.worker_notify.wait(progress).unwrap();
			}
		}
	}
}
