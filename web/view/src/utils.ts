export function setWasmImports(imports: Record<string, Function>) {
  window.wasm_imports = {
    ...window.wasm_imports,
    ...imports
  };
};
