import llvm_imports from './llvm-imports.mjs';
import ffi_imports from './ffi-imports.mjs';
import progress_imports from './progress-imports.mjs';

if (!window.crossOriginIsolate) {
	document.body.insertAdjacentHTML('beforeend', `<p style="color: red;">Not CrossOriginIsolated - expect failures</p>`);
}

(async function () {
	const module = await WebAssembly.compileStreaming(
		fetch("wasm/webtrace.wasm")
	);

	// TODO: Calculate the min / max from the wasm module
	const memory = new WebAssembly.Memory({ initial: 17, maximum: 16384, shared: true });
	const imports = {
		env: {
			memory,
			...llvm_imports(memory),
			...ffi_imports(memory),
			...progress_imports(memory)
		}
	};

	const instance = await WebAssembly.instantiate(module, imports);


	// Instantiate our Workers:
	for (let i = 0; i < 4; ++i) {
		const worker = new Worker('js/worker.mjs', {
			type: 'module'
		});
		worker.onerror = e => console.error(e.message);
		worker.postMessage({ memory, module });
	}

	const { start_render, status } = instance.exports;
	const render_btn = document.getElementById('start-render');
	render_btn.innerText = "Render";
	render_btn.disabled = false;
	render_btn.onclick = () => {
		if (!start_render()) {
			alert('Failed to start render');
			return;
		}
		render_btn.disabled = true;

		// Start the progress loop
		(function update() {
			if (status()) {
				requestAnimationFrame(update);
			} else {
				render_btn.disabled = false;
			}
		})();
	};
})()