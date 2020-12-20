;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx

(module
  (type $t0 (func (param i32 i32 i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (result i32)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t0)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))

  (func $_start (type $t2)
    i32.const 1
    i32.const 8
    i32.const 1
    i32.const 12
    call $wasi_unstable.fd_write)
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\0d\00\00\00")
  (data $d1 (i32.const 16) "Hello World!\0a"))