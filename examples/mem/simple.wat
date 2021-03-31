(module
  (memory (export "memory") 1)
  (func $_start (result i32)
    (i32.const 5)
    loop
      (i32.const 0)
      br_if 0
    end
    (i32.const 5)
    (i32.add)
  )
  (export "_start" (func $_start))
)