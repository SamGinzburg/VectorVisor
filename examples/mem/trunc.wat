(module
  (memory (export "memory") 1)
  (func $_start (result i32)
    (i32.const 5)
    (i32.const 0xFFFFFFFF)
    (i32.store16)
    (i32.const 5)
    (i32.load)
  )
  (export "_start" (func $_start))
)