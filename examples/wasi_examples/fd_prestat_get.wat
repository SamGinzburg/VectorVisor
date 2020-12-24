;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx
;; RUST_LOG=trace wasmtime wasi_examples/fd_prestat_get.wat --dir=.
(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32 i32 i32 i32) (result i32)))

  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t3)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))
  (import "wasi_snapshot_preview1" "fd_prestat_get" (func $fd_prestat_get (param i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_prestat_dir_name" (func $fd_prestat_dir_name (param i32 i32 i32) (result i32)))

  (func $_start (type $t2)
    i32.const 3  ;; pre-opened fd to query, this will always be the first opened fd (wasi-libc just checks 1by1 during init)
    i32.const 16  ;; buf pointer where metadata will be stored 
    call $fd_prestat_get
    drop
    ;; buf points at the preopen type, buf + 4 points at the size of the name, u32 in size
    i32.const 20
    i32.load
    ;; the value loaded should be equal to the strlen of --dir="." -> 1
  )
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start)))