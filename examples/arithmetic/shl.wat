(module
  (func $_start (result i32)
    (local $l2 i32)
    (i32.const 100)
    (i32.const -999999999)
    (i32.shl)
  )
  (export "_start" (func $_start))
)