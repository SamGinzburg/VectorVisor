(module
  (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func))
  (import "wasi_unstable" "fd_write" (func (;0;) (type 0)))
  (import "wasi_unstable" "proc_exit" (func (;1;) (type 1)))
  (func (;2;) (type 2)
    i32.const 1
    i32.const 8
    i32.const 1
    i32.const 12
    call 0)
  (memory (;0;) 1)
  (export "memory" (memory 0))
  (export "_start" (func 2))
  (data (;0;) (i32.const 8) "\10\00\00\00\0d\00\00\00")
  (data (;1;) (i32.const 16) "Hello World!\0a"))
