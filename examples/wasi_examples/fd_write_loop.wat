;; repro for ICF
(module
  (type $t0 (func (param i32 i32 i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t0)))
  (import "wasi_unstable" "proc_exit" (func $wasi_unstable.proc_exit (type $t1)))
  (func $_eq (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.eq)
  )
 (func $_print100 (local $x i32)
  ;; x = 0
  (i32.const 1)
  (set_local $x)
  (block $B0
  (block $B1
    (loop $L0
    i32.const 1
    i32.const 8
    i32.const 1
    i32.const 12
    call $wasi_unstable.fd_write
      (get_local $x)
      (i32.const 1)
      (i32.sub)
      (set_local $x)
      (get_local $x)
      (i32.const 0)
      (call $_eq)
      (br_if $B0)
      (br $L0))))
    return)
  (func $_start (type $t2)
    loop $L1
    call $_print100
    end)
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (data $d0 (i32.const 8) "\10\00\00\00\0d\00\00\00")
  (data $d1 (i32.const 16) "Hello World!\0a"))
