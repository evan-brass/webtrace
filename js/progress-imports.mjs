export default function progress_imports(shared_memory) {
	const canvas = document.querySelector('canvas');
	const dctx = canvas.getContext('2d');

	return {
		view_bitmap(ptr, width, len) {
			// Make sure that the canvas is set to the proper size
			const height = len / (width * 4);
			canvas.width = width;
			canvas.height = height;

			const sbuff = new Uint8ClampedArray(shared_memory.buffer, ptr, len);
			const abuff = new Uint8ClampedArray(len);
			for (let i = 0; i < len; ++i) {
				abuff[i] = sbuff[i];
			}
			const id = new ImageData(abuff, width);
			dctx.putImageData(id, 0, 0);
		}
	};
}

export function progress_imports_worker() {
	return {
		view_bitmap() {
			throw new Error("view_bitmap mustn't be called from a worker.");
		}
	}
}