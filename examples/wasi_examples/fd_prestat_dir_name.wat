;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx
;; RUST_LOG=trace wasmtime wasi_examples/fd_prestat_dir_name.wat --dir=.
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
    i32.const 3  ;; fd
    i32.const 50 ;; store the string at this pointer
    i32.const 20 ;; load the string length from this pointer (will match result from fd_prestat_get)
    i32.load
    call $fd_prestat_dir_name
    drop
    ;; store the length
    i32.const 12
    i32.const 20
    i32.load
    i32.store
    ;; print the path
    i32.const 1
    i32.const 8
    i32.const 1
    i32.const 10
    call $wasi_unstable.fd_write
  )
  (memory $memory 1)
  (data $d0 (i32.const 8) "\32\00\00\00\00\00\00\00")
  (export "memory" (memory 0))
  (export "_start" (func $_start)))