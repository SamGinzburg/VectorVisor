import * as __import0 from "wasi_snapshot_preview1";
async function instantiate(module, imports = {}) {
  const __module0 = imports.wasi_snapshot_preview1;
  const adaptedImports = {
    wasi_snapshot_preview1: Object.assign(Object.create(__module0), {
      fd_write(fd, iovs, iovs_len, nwritten) {
        // ~lib/@assemblyscript/wasi-shim/assembly/bindings/wasi_snapshot_preview1/fd_write(u32, usize, usize, usize) => u16
        fd = fd >>> 0;
        iovs = iovs >>> 0;
        iovs_len = iovs_len >>> 0;
        nwritten = nwritten >>> 0;
        return __module0.fd_write(fd, iovs, iovs_len, nwritten);
      },
    }),
    env: Object.assign(Object.create(globalThis), imports.env || {}),
  };
  const { exports } = await WebAssembly.instantiate(module, adaptedImports);
  exports._start();
  return exports;
}
export const {
  memory,
  
} = await (async url => instantiate(
  await (async () => {
    try { return await globalThis.WebAssembly.compileStreaming(globalThis.fetch(url)); }
    catch { return globalThis.WebAssembly.compile(await (await import("node:fs/promises")).readFile(url)); }
  })(), {
    wasi_snapshot_preview1: __maybeDefault(__import0),
  }
))(new URL("release.wasm", import.meta.url));
function __maybeDefault(module) {
  return typeof module.default === "object" && Object.keys(module).length == 1
    ? module.default
    : module;
}
