(module
  (func $_start (result i32)
    (local $l2 i64)
    (i64.const 0xFF00000000000000)
    (i32.wrap_i64)
  )
  (export "_start" (func $_start))
)
