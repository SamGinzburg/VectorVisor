(module
  (memory (export "memory") 1)
  (func $_start (result i32)
    (i32.const 1)
    (i32.const -5)
    (i32.store8)
    (i32.const 1)
    (i32.load8_s)
  )
  (export "_start" (func $_start))
)