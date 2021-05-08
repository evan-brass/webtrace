export default function ffi_imports(shared_memory) {
	const decoder = new TextDecoder('utf8', {
		fatal: true
	});
	return {
		get_random(ptr, len) {
			let buff = new Uint8Array(len);
			crypto.getRandomValues(buff);
			const target = new Uint8Array(shared_memory.buffer, ptr, len);
			target.set(buff);
		},
		console_log(ptr, len) {
			const buff = new Uint8Array(len);
			buff.set(new Uint8Array(shared_memory.buffer, ptr, len));
			console.log(decoder.decode(buff));
		}
	};
}