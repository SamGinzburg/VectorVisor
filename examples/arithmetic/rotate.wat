(module
  (func $_start (result i32)
    (local $l2 i64)
    (i32.const 0xFFFF)
    (i32.const 4)
    (i32.rotl)
  )
  (export "_start" (func $_start))
)