;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx

(module
  (type $t0 (func (param i32 i32 i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t3 (func (param i32 i32)))
  (type $t2 (func (param i32 i32) (result i32)))

  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t0)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))
  (import "wasi_snapshot_preview1" "random_get" (func $random (type $t2)))

  (func $_start (result i32)
      i32.const 100 ;; buf_ptr
      i32.const 1024 ;; buf_len
      call $random
      drop
      i32.const 100
      i32.load
    )
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\0d\00\00\00")
  (data $d1 (i32.const 16) "Hello World!\0a"))
