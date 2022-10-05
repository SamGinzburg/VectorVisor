;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx
;; wasmtime --env="ONE=1" examples/wasi_examples/environ_sizes_get.wat
(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32 i32 i32 i32) (result i32)))

  (import "wasi_unstable" "environ_get" (func $wasi_unstable.environ_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t3)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))
  (func $_start (type $t2)
    i32.const 4
    i32.const 10000
    call $wasi_unstable.environ_get
    drop
    i32.const 8
    i32.load
  )
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
)
