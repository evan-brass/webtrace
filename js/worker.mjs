import llvm_imports from './llvm-imports.mjs';
import ffi_imports from './ffi-imports.mjs';
import { progress_imports_worker } from './progress-imports.mjs';

self.onmessage = async ({ data: { memory, module } }) => {
	const imports = {
		env: {
			memory,
			...llvm_imports(memory),
			...ffi_imports(memory),
			...progress_imports_worker()
		}
	};

	const instance = await WebAssembly.instantiate(module, imports);

	const { work } = instance.exports;

	// In the future, this work function will be divergent (never return)
	work();
}
