import llvm_imports from './llvm-imports.mjs';
import ffi_imports from './ffi-imports.mjs';

self.onmessage = async ({ data: { memory, module } }) => {
	const imports = {
		env: {
			memory,
			...llvm_imports(memory),
			...ffi_imports(memory)
		}
	};

	const instance = await WebAssembly.instantiate(module, imports);

	const { work } = instance.exports;

	// In the future, this work function will be divergent (never return)
	work();
}
