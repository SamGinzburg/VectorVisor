(module
  (func $_lt (param $p0 i32) (param $p1 i32) (result i32)
    (local.get $p0)
    (local.get $p1)
    (i32.lt_u)
  )

  (func $_start (result i32)
    (i32.const 99)
    (i32.const 100)
    (call $_lt)
  )
  (export "_start" (func $_start))
)