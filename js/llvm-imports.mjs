// As best as I can understand, these imports are an accident.  My guess is that Rustc thought llvm would output instructions to replace these calls.
// The instructions aren't standardized yet in WebAssembly and they've changed names / codes in the drafts which I believe has tripped up llvm/rustc.
// In any case, until Nightly includes a fix, I'm hoping this will work.

export default function llvm_imports(shared_memory) {
	// So... WebAssembly is little-endian, however, Int32Array is system endian, however again, JS Atomics can only be used with Int32Array
	// If then, the system is big-endian, we need to use the big-endian equivalent of the little endian expected value (exp) in our call to the Atomic operation.

	const is_big_endian = (function () {
		// TODO: Verify that this works.
		const test_value = 257;
		const arr = new Int32Array(1);
		const view = new DataView(arr.buffer);
		view.setInt32(0, test_value, true);
		return arr[0] === test_value;
	})();

	const convert = new DataView(new ArrayBuffer(4));
	return {
		["llvm.wasm.atomic.wait.i32"](ptr, exp, timeout) {
			const i32 = new Int32Array(shared_memory.buffer, ptr);

			if (is_big_endian) {
				// TODO: Verify that this works.
				// Set as little endian, and get as big endian because the test will check the big-endian value
				convert.setInt32(0, exp, true);
				exp = convert.getInt32(0, false);
			}

			const r = Atomics.wait(i32, 0, exp, timeout);
			return ["ok", "not-equal", "timed-out"].indexOf(r);
		},
		["llvm.wasm.atomic.wait.i64"](_ptr, _exp, _timeout) {
			// No 64 bit support
			const err = new Error("64 bit support is not available yet.");
			console.error(err);
			throw err;
		},
		["llvm.wasm.atomic.notify"](ptr, cnt) {
			const i32 = new Int32Array(shared_memory.buffer, ptr);
			return Atomics.notify(i32, 0, cnt);
		}
	};
}