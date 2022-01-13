(module
  (import "wasi_unstable" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (;0;) 1)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (func $_start (result i32)
    i32.const 1 ;; stdout
    i32.const 0 ;; iovec ptr
    i32.const 2 ;; entries
    i32.const 24 ;; out bytes
    call $fd_write
  )
  (data (i32.const 0) "\10\00\00\00\02\00\00\00")
  (data (i32.const 8) "\12\00\00\00\02\00\00\00ABCD")
)