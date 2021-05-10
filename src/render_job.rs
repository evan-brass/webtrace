use super::{
	bitmap::{Bitmap, Color},
	ray::Ray,
	scene::Scene,
};
use std::sync::{Arc, Condvar, Mutex};

struct Camera {
	// TODO: Depth of field (focal plane / focal length + aperture)
	// TODO: field of view
	aspect_ratio: f32,
	pointing: Ray,
}
impl Camera {
	fn ray(&self, _u: f32, _v: f32) -> Ray {
		Default::default()
	}
}
struct Sampler {}
impl Sampler {
	fn sample(&self, _ray: Ray, _scene: &Scene) -> Color {
		Color::default()
	}
}

struct Progress {
	passes: usize,
	line: usize,
}
pub struct RenderJob {
	camera: Arc<Camera>,
	scene: Arc<Scene>,
	output: Bitmap,
	progress: Mutex<Progress>,
	worker_notify: Condvar,
}
impl RenderJob {
	pub fn is_done(&self) -> bool {
		self.progress.lock().unwrap().passes == 0
	}
	pub fn new(scene: Arc<Scene>, camera: Arc<Camera>, image_width: usize, passes: usize) -> Self {
		let output = Bitmap::new(
			image_width,
			(image_width as f32 / camera.aspect_ratio).round() as usize,
		);
		Self {
			scene,
			camera,
			output,
			worker_notify: Condvar::new(),
			progress: Mutex::new(Progress { passes, line: 0 }),
		}
	}

	// Called by a worker to work on a render job
	pub fn work(&self) {}

	// Called by the main thread to visualize the
	pub fn progress(&self) {}
}
