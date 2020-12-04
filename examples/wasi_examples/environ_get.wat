;; see https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/witx/wasi_snapshot_preview1.witx
;; wasmtime --env="ONE=1" examples/wasi_examples/environ_sizes_get.wat
(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32 i32 i32 i32) (result i32)))

  (import "wasi_unstable" "environ_sizes_get" (func $wasi_unstable.environ_sizes_get (type $t0)))
  (import "wasi_unstable" "environ_get" (func $wasi_unstable.environ_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t3)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))
  (func $_start (type $t2)
    i32.const 0  ;; offset for where the number of arguments are stored
    i32.const 0  ;; value 
    i32.store    ;;
    i32.const 4  ;; offset for the size of the argument string data
    i32.const 0  ;; value 
    i32.store    ;;
    i32.const 0
    i32.const 4
    call $wasi_unstable.environ_sizes_get
    drop
    ;; okay we have got the sizes, now call environ_get
    i32.const 500 ;; char** environ, the pointer to a buffer for all the env_var pointers
                  ;; the length is equivalent to the number of env vars * 4
    i32.const 16 ;; char* env string buffer, the pointer to the string containing the env
                 ;; the length is given by environ_sizes_get
    call $wasi_unstable.environ_get
    drop
    i32.const 1
    i32.const 8
    i32.const 1
    i32.const 10
    call $wasi_unstable.fd_write
    )
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\ff\00\00\00")
  (data $d1 (i32.const 16) "Hello World!\0a"))