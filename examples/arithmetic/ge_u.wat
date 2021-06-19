(module
  (memory (export "memory") 48)
  (func $fmt_u64 (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
        local.get 0
        i64.const 10000
        i64.ge_u
  )
 
  (func $_start (result i32)
    (local $l2 i32)
    (i64.const 0x1337)
    (i32.const 112312323)
    (i32.const 10)
    (call $fmt_u64)
  )

  (global $__stack_pointer (mut i32) (i32.const 1000000))
  (export "_start" (func $_start))
)
