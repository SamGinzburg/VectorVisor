;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx

(module
  (type $t0 (func (param i32 i32 i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32 i32) (result i32)))
  (type $t4 (func (param i32 i32)))

  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t0)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))
  (import "env" "serverless_invoke" (func $serverless_invoke (type $t3)))
  (import "env" "serverless_response" (func $serverless_response (type $t4)))

  (func $_start
    loop $loop1
  
      i32.const 100 ;; buf_ptr
      i32.const 1024 ;; buf_len
      call $serverless_invoke

      i32.const 1
      i32.const 8
      i32.const 1
      i32.const 12
      call $wasi_unstable.fd_write
      drop

      i32.const 100 ;; buf_ptr
      i32.const 1024 ;; buf_len
      call $serverless_response

      br $loop1
    end
    )
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\0d\00\00\00")
  (data $test (i32.const 100) "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
  (data $d1 (i32.const 16) "Hello World!\0a"))
