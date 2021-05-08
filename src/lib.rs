mod ffi;

#[no_mangle]
pub extern "C" fn work() {
	ffi::log("Starting to work...");
}
