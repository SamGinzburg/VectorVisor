(module
  (memory (export "memory") 1)
  (func $_start (result i32)
    (i64.const 0xAFFF7FFFAFFF7FFF)
    (i32.wrap_i64)
  )
  (export "_start" (func $_start))
)