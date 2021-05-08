extern "C" {
	fn console_log(ptr: *const u8, length: u32);
	fn get_random(ptr: *const u8, length: u32);
}
pub fn get_seed() -> [u8; 16] {
	let seed = [0; 16];
	unsafe {
		get_random(seed.as_ptr(), seed.len() as u32);
	}
	seed
}
pub fn log<T: AsRef<str>>(s: T) {
	unsafe {
		console_log(s.as_ref().as_ptr(), s.as_ref().len() as u32);
	}
}
